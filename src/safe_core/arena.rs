//! Scoped, main-thread owner identities and checked slot generations.
//!
//! Cross-owner handles are rejected by Rust's invariant lifetime brand:
//! ```compile_fail
//! use rless_safe_core::arena::{with_arena, Arena};
//! with_arena(|mut first: Arena<'_, u8>| {
//!     let handle = first.try_insert(7).unwrap();
//!     with_arena(|second: Arena<'_, u8>| { second.get(handle); });
//! });
//! ```
//! A handle cannot escape its owning scope:
//! ```compile_fail
//! use rless_safe_core::arena::{with_arena, Arena};
//! let escaped = with_arena(|mut owner: Arena<'_, u8>| owner.try_insert(7).unwrap());
//! ```

use crate::reserve::{reserve, Gate, Unfaulted};
use crate::StorageError;
use std::marker::PhantomData;
use std::rc::Rc;

// The nested mutable reference makes the lifetime invariant. Rc in PhantomData
// adds main-thread confinement without allocating a runtime identity object.
type Brand<'id> = PhantomData<(&'id mut &'id (), Rc<()>)>;

#[derive(Debug)]
pub struct InsertError<T> {
    pub error: StorageError,
    pub value: T,
}

pub struct Handle<'id, T> {
    slot: usize,
    generation: u64,
    brand: Brand<'id>,
    kind: PhantomData<fn() -> T>,
}
impl<T> Copy for Handle<'_, T> {}
impl<T> Clone for Handle<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> std::fmt::Debug for Handle<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Handle")
            .field("slot", &self.slot)
            .field("generation", &self.generation)
            .finish()
    }
}
impl<T> PartialEq for Handle<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.slot == other.slot && self.generation == other.generation
    }
}
impl<T> Eq for Handle<'_, T> {}

struct Slot<T> {
    value: Option<T>,
    generation: u64,
    next_free: Option<usize>,
    retired: bool,
}

/// Creation is private; `with_arena` supplies a fresh invariant owner brand.
pub struct Arena<'id, T> {
    slots: Vec<Slot<T>>,
    free_head: Option<usize>,
    live: usize,
    brand: Brand<'id>,
}

/// The result cannot contain this call's branded arena/handles. The eventual
/// PagerState event loop may live inside this scope, after integration is gated in.
pub fn with_arena<T, R>(f: impl for<'id> FnOnce(Arena<'id, T>) -> R) -> R {
    f(Arena {
        slots: Vec::new(),
        free_head: None,
        live: 0,
        brand: PhantomData,
    })
}

impl<'id, T> Arena<'id, T> {
    pub fn len(&self) -> usize {
        self.live
    }
    pub fn is_empty(&self) -> bool {
        self.live == 0
    }
    pub fn slots(&self) -> usize {
        self.slots.len()
    }
    pub fn try_insert(&mut self, value: T) -> Result<Handle<'id, T>, InsertError<T>> {
        self.insert_gate(value, &mut Unfaulted)
    }
    fn insert_gate(
        &mut self,
        value: T,
        gate: &mut impl Gate,
    ) -> Result<Handle<'id, T>, InsertError<T>> {
        let Some(live) = self.live.checked_add(1) else {
            return Err(InsertError {
                error: StorageError::CapacityOverflow,
                value,
            });
        };
        let index = if let Some(index) = self.free_head {
            // An intrusive free list needs no allocation on removal or reuse.
            let slot = &mut self.slots[index];
            self.free_head = slot.next_free;
            slot.next_free = None;
            slot.value = Some(value);
            index
        } else {
            if let Err(error) = reserve(&mut self.slots, 1, gate) {
                return Err(InsertError { error, value });
            }
            let index = self.slots.len();
            self.slots.push(Slot {
                value: Some(value),
                generation: 0,
                next_free: None,
                retired: false,
            });
            index
        };
        self.live = live;
        Ok(Handle {
            slot: index,
            generation: self.slots[index].generation,
            brand: PhantomData,
            kind: PhantomData,
        })
    }
    pub fn get(&self, handle: Handle<'id, T>) -> Option<&T> {
        let slot = self.slots.get(handle.slot)?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.value.as_ref()
    }
    pub fn get_mut(&mut self, handle: Handle<'id, T>) -> Option<&mut T> {
        let slot = self.slots.get_mut(handle.slot)?;
        if slot.generation != handle.generation {
            return None;
        }
        slot.value.as_mut()
    }
    pub fn remove(&mut self, handle: Handle<'id, T>) -> Option<T> {
        let slot = self.slots.get_mut(handle.slot)?;
        if slot.generation != handle.generation {
            return None;
        }
        let value = slot.value.take()?;
        self.live -= 1;
        if let Some(next) = slot.generation.checked_add(1) {
            slot.generation = next;
            slot.next_free = self.free_head;
            self.free_head = Some(handle.slot);
        } else {
            slot.retired = true;
            // Retired slots are never placed back on the free list.
        }
        Some(value)
    }
    #[cfg(any(test, feature = "fault-testing"))]
    pub fn try_insert_with_fault(
        &mut self,
        value: T,
        fault: &mut crate::ReserveFault,
    ) -> Result<Handle<'id, T>, InsertError<T>> {
        self.insert_gate(value, fault)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    #[test]
    fn reuse_never_revives_stale_handles_and_keeps_live_count() {
        with_arena(|mut a| {
            let first = a.try_insert(10).unwrap();
            assert_eq!(a.remove(first), Some(10));
            assert_eq!(a.remove(first), None);
            let second = a.try_insert(20).unwrap();
            assert_eq!(first.slot, second.slot);
            assert_ne!(first, second);
            assert_eq!(a.get(first), None);
            assert_eq!(a.get(second), Some(&20));
            *a.get_mut(second).unwrap() = 30;
            assert_eq!(a.remove(second), Some(30));
            assert!(a.is_empty());
            assert_eq!(a.slots(), 1);
        });
    }
    #[test]
    fn failed_insertion_returns_value_without_commit_or_drop() {
        struct Value(Rc<Cell<usize>>);
        impl Drop for Value {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }
        let drops = Rc::new(Cell::new(0));
        with_arena(|mut a| {
            let mut f = crate::ReserveFault::new(1).unwrap();
            let failure = a
                .try_insert_with_fault(Value(drops.clone()), &mut f)
                .err()
                .unwrap();
            assert_eq!(failure.error, StorageError::AllocationFailed);
            assert_eq!((a.len(), a.slots(), drops.get(), f.hits()), (0, 0, 0, 1));
            let handle = a.try_insert_with_fault(failure.value, &mut f).ok().unwrap();
            drop(a.remove(handle));
            assert_eq!(drops.get(), 1);
            let mut unused_fault = crate::ReserveFault::new(1).unwrap();
            a.try_insert_with_fault(Value(drops.clone()), &mut unused_fault)
                .ok()
                .unwrap();
            assert_eq!(unused_fault.attempts(), 0);
        });
        assert_eq!(drops.get(), 2);
    }
    #[test]
    fn generation_exhaustion_retires_slot_without_wrapping() {
        with_arena(|mut a| {
            let first = a.try_insert(1).unwrap();
            a.slots[first.slot].generation = u64::MAX - 1;
            let near_limit = Handle {
                generation: u64::MAX - 1,
                ..first
            };
            assert_eq!(a.remove(near_limit), Some(1));
            let last = a.try_insert(2).unwrap();
            assert_eq!(last.generation, u64::MAX);
            assert_eq!(a.remove(last), Some(2));
            assert!(a.slots[last.slot].retired);
            let fresh = a.try_insert(3).unwrap();
            assert_ne!(last.slot, fresh.slot);
            assert_eq!(a.get(last), None);
            assert_eq!(a.get(near_limit), None);
            assert_eq!(a.get(fresh), Some(&3));
        });
    }
}
