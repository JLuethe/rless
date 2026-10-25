//! Bounded component model, using the actual isolated safe-core implementation.
//! No claim of migrated pager callers or production fuzz budget credit.
use rless_safe_core::arena::with_arena;
use rless_safe_core::bytebuf::ByteBuf;
use rless_safe_core::coordinate::{CoordinateError, RetainedRange, SourceOffset};
use rless_safe_core::{ReserveFault, StorageError};

#[derive(Default, Debug)]
pub struct Stats {
    pub actions: usize,
    pub reserve_attempts: usize,
    pub reserve_failures: usize,
}

pub fn exercise(input: &[u8]) -> Stats {
    let mut stats = Stats::default();
    let mut owner = ByteBuf::new();
    let mut model = Vec::<u8>::new();
    for chunk in input.chunks_exact(4).take(1024) {
        stats.actions += 1;
        if model.len() > 4096 {
            owner.reset();
            model.clear();
        }
        let mut staged = model.clone();
        let mut fault = ReserveFault::new(if chunk[0] & 16 != 0 { 1 } else { usize::MAX }).unwrap();
        let a = chunk[1] as usize % (model.len() + 2);
        let b = chunk[2] as usize % (model.len() + 2);
        let source = &chunk[1..];
        let mut expected = Ok(());
        let result = match chunk[0] % 8 {
            0 => {
                staged.extend_from_slice(source);
                owner.try_extend_with_fault(source, &mut fault)
            }
            1 => {
                if a > b || b > model.len() {
                    expected = Err(StorageError::InvalidRange);
                } else {
                    staged.extend_from_slice(&model[a..b]);
                }
                owner.try_extend_from_within_with_fault(a..b, &mut fault)
            }
            2 => {
                assert_eq!(owner.pop(), staged.pop());
                Ok(())
            }
            3 => {
                let length = chunk[1] as usize;
                if length > model.len() {
                    expected = Err(StorageError::InvalidRange);
                } else {
                    staged.truncate(length);
                }
                owner.truncate(length)
            }
            4 => {
                owner.reset();
                staged.clear();
                Ok(())
            }
            5 => {
                staged.clear();
                staged.extend_from_slice(source);
                owner.try_replace_with_fault(source, &mut fault)
            }
            6 => {
                if a > b || b > model.len() {
                    expected = Err(StorageError::InvalidRange);
                } else {
                    staged = model[a..b].to_vec();
                }
                owner.try_replace_from_within_with_fault(a..b, &mut fault)
            }
            _ => {
                if let Some(offset) = model.iter().position(|&v| v == 0) {
                    expected = Err(StorageError::InteriorNul { offset });
                }
                owner
                    .try_nul_terminated_with_fault(&mut fault)
                    .map(|owned| {
                        assert_eq!(owned.as_bytes(), model);
                        assert_eq!(owned.as_bytes_with_nul().last(), Some(&0));
                        assert_eq!(owned.as_bytes_with_nul().len(), model.len() + 1);
                    })
            }
        };
        stats.reserve_attempts += fault.attempts();
        stats.reserve_failures += fault.hits();
        if result == Err(StorageError::AllocationFailed) {
            assert_eq!(fault.hits(), 1, "unexpected native allocation failure");
        } else {
            assert_eq!(fault.hits(), 0);
            assert_eq!(result, expected);
            if result.is_ok() {
                model = staged;
            }
        }
        assert_eq!(owner.as_bytes(), model);
        assert!(owner.len() <= owner.capacity());
    }
    with_arena(|mut arena| {
        let mut history = Vec::new();
        for chunk in input.chunks_exact(3).take(256) {
            match chunk[0] % 4 {
                0 => {
                    let value = u16::from_le_bytes([chunk[1], chunk[2]]);
                    let mut fault =
                        ReserveFault::new(if chunk[0] & 16 != 0 { 1 } else { usize::MAX }).unwrap();
                    match arena.try_insert_with_fault(value, &mut fault) {
                        Ok(handle) => {
                            assert_eq!(fault.hits(), 0);
                            history.push((handle, Some(value)));
                        }
                        Err(rejected) => {
                            assert_eq!(rejected.error, StorageError::AllocationFailed);
                            assert_eq!(rejected.value, value);
                            assert_eq!(fault.hits(), 1);
                        }
                    }
                    stats.reserve_attempts += fault.attempts();
                    stats.reserve_failures += fault.hits();
                }
                1 if !history.is_empty() => {
                    let index = chunk[1] as usize % history.len();
                    let (handle, value) = &mut history[index];
                    assert_eq!(arena.remove(*handle), value.take());
                }
                2 if !history.is_empty() => {
                    let index = chunk[1] as usize % history.len();
                    let (handle, value) = &mut history[index];
                    match (arena.get_mut(*handle), value.as_mut()) {
                        (Some(actual), Some(expected)) => {
                            *actual = chunk[2] as u16;
                            *expected = *actual;
                        }
                        (None, None) => {}
                        _ => panic!("stale handle changed an unrelated live slot"),
                    }
                }
                _ => {}
            }
            assert_eq!(
                arena.len(),
                history.iter().filter(|(_, value)| value.is_some()).count()
            );
            for (handle, expected) in &history {
                assert_eq!(arena.get(*handle), expected.as_ref());
            }
        }
    });
    if input.len() >= 8 {
        let raw = u64::from_le_bytes(input[..8].try_into().unwrap());
        let offset = SourceOffset::new(raw);
        assert_eq!(offset.is_ok(), raw <= i64::MAX as u64);
        if let Ok(start) = offset {
            let length = input.len() % 256;
            if let Ok(range) = RetainedRange::new(start, length) {
                assert_eq!(range.boundary(range.end()).unwrap().get(), length);
                assert_eq!(range.index(range.end()), Err(CoordinateError::IndexRange));
            }
            assert_eq!(start.try_regex_offset().is_ok(), raw <= i32::MAX as u64);
        }
    }
    stats
}
