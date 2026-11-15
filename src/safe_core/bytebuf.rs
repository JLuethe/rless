use crate::reserve::{reserve, Gate, Unfaulted};
use crate::StorageError;
use std::ops::Range;

/// One owner for arbitrary bytes. No pointer or mutable slice escapes.
///
/// A slice cannot remain borrowed through mutation:
/// ```compile_fail
/// use rless_safe_core::bytebuf::ByteBuf;
/// let mut bytes = ByteBuf::try_from_slice(b"old").unwrap();
/// let retained = bytes.as_bytes();
/// bytes.try_extend(b"new").unwrap();
/// assert_eq!(retained, b"old");
/// ```
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ByteBuf {
    bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NulTerminatedBytes {
    bytes: Vec<u8>,
}

impl NulTerminatedBytes {
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.bytes.len() - 1]
    }
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        &self.bytes
    }
}

impl ByteBuf {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn try_from_slice(source: &[u8]) -> Result<Self, StorageError> {
        let mut owner = Self::new();
        owner.try_extend(source)?;
        Ok(owner)
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
    pub fn capacity(&self) -> usize {
        self.bytes.capacity()
    }
    pub fn pop(&mut self) -> Option<u8> {
        self.bytes.pop()
    }
    pub fn reset(&mut self) {
        self.bytes.clear();
    }
    pub fn truncate(&mut self, length: usize) -> Result<(), StorageError> {
        if length > self.len() {
            return Err(StorageError::InvalidRange);
        }
        self.bytes.truncate(length);
        Ok(())
    }
    pub fn try_push(&mut self, byte: u8) -> Result<(), StorageError> {
        self.try_extend(&[byte])
    }
    pub fn try_extend(&mut self, source: &[u8]) -> Result<(), StorageError> {
        self.extend_gate(source, &mut Unfaulted)
    }
    fn extend_gate(&mut self, source: &[u8], gate: &mut impl Gate) -> Result<(), StorageError> {
        reserve(&mut self.bytes, source.len(), gate)?;
        self.bytes.extend_from_slice(source);
        Ok(())
    }
    fn range(&self, range: &Range<usize>) -> Result<(), StorageError> {
        if range.start > range.end || range.end > self.len() {
            return Err(StorageError::InvalidRange);
        }
        Ok(())
    }
    pub fn try_extend_from_within(&mut self, range: Range<usize>) -> Result<(), StorageError> {
        self.within_gate(range, &mut Unfaulted)
    }
    fn within_gate(
        &mut self,
        range: Range<usize>,
        gate: &mut impl Gate,
    ) -> Result<(), StorageError> {
        self.range(&range)?;
        reserve(&mut self.bytes, range.end - range.start, gate)?;
        // Range indexes refer to the original bytes, reacquired after reserve.
        self.bytes.extend_from_within(range);
        Ok(())
    }
    pub fn try_replace(&mut self, source: &[u8]) -> Result<(), StorageError> {
        self.replace_gate(source, &mut Unfaulted)
    }
    fn replace_gate(&mut self, source: &[u8], gate: &mut impl Gate) -> Result<(), StorageError> {
        let mut staged = Vec::new();
        reserve(&mut staged, source.len(), gate)?;
        staged.extend_from_slice(source);
        self.bytes = staged;
        Ok(())
    }
    pub fn try_replace_from_within(&mut self, range: Range<usize>) -> Result<(), StorageError> {
        self.replace_within_gate(range, &mut Unfaulted)
    }
    fn replace_within_gate(
        &mut self,
        range: Range<usize>,
        gate: &mut impl Gate,
    ) -> Result<(), StorageError> {
        self.range(&range)?;
        let mut staged = Vec::new();
        reserve(&mut staged, range.end - range.start, gate)?;
        staged.extend_from_slice(&self.bytes[range]);
        self.bytes = staged;
        Ok(())
    }
    pub fn try_nul_terminated(&self) -> Result<NulTerminatedBytes, StorageError> {
        self.nul_gate(&mut Unfaulted)
    }
    fn nul_gate(&self, gate: &mut impl Gate) -> Result<NulTerminatedBytes, StorageError> {
        if let Some(offset) = self.bytes.iter().position(|&b| b == 0) {
            return Err(StorageError::InteriorNul { offset });
        }
        let length = self
            .len()
            .checked_add(1)
            .ok_or(StorageError::CapacityOverflow)?;
        let mut staged = Vec::new();
        reserve(&mut staged, length, gate)?;
        staged.extend_from_slice(&self.bytes);
        staged.push(0);
        Ok(NulTerminatedBytes { bytes: staged })
    }

    #[cfg(any(test, feature = "fault-testing"))]
    pub fn try_extend_with_fault(
        &mut self,
        source: &[u8],
        fault: &mut crate::ReserveFault,
    ) -> Result<(), StorageError> {
        self.extend_gate(source, fault)
    }
    #[cfg(any(test, feature = "fault-testing"))]
    pub fn try_extend_from_within_with_fault(
        &mut self,
        range: Range<usize>,
        fault: &mut crate::ReserveFault,
    ) -> Result<(), StorageError> {
        self.within_gate(range, fault)
    }
    #[cfg(any(test, feature = "fault-testing"))]
    pub fn try_replace_with_fault(
        &mut self,
        source: &[u8],
        fault: &mut crate::ReserveFault,
    ) -> Result<(), StorageError> {
        self.replace_gate(source, fault)
    }
    #[cfg(any(test, feature = "fault-testing"))]
    pub fn try_nul_terminated_with_fault(
        &self,
        fault: &mut crate::ReserveFault,
    ) -> Result<NulTerminatedBytes, StorageError> {
        self.nul_gate(fault)
    }
    #[cfg(any(test, feature = "fault-testing"))]
    pub fn try_replace_from_within_with_fault(
        &mut self,
        range: Range<usize>,
        fault: &mut crate::ReserveFault,
    ) -> Result<(), StorageError> {
        self.replace_within_gate(range, fault)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ReserveFault;

    #[test]
    fn arbitrary_bytes_reset_reuse_pop_and_empty_ranges() {
        let mut b = ByteBuf::new();
        assert_eq!(b.pop(), None);
        b.try_extend_from_within(0..0).unwrap();
        let all: Vec<_> = (0..=255).collect();
        b.try_extend(&all).unwrap();
        assert_eq!(b.as_bytes(), &all);
        let capacity = b.capacity();
        b.reset();
        assert!(b.is_empty());
        assert_eq!(b.capacity(), capacity);
        b.try_extend(b"reused").unwrap();
        assert_eq!(b.pop(), Some(b'd'));
        b.truncate(2).unwrap();
        assert_eq!(b.as_bytes(), b"re");
        assert_eq!(b.truncate(3), Err(StorageError::InvalidRange));
        assert_eq!(b.as_bytes(), b"re");
    }

    #[test]
    fn overlapping_self_copy_and_replacement_match_original_ranges() {
        let mut b = ByteBuf::try_from_slice(b"abcde").unwrap();
        b.try_extend_from_within(1..5).unwrap();
        assert_eq!(b.as_bytes(), b"abcdebcde");
        b.try_replace_from_within(2..7).unwrap();
        assert_eq!(b.as_bytes(), b"cdebc");
        for range in [6..7, 4..3, 0..usize::MAX] {
            assert_eq!(
                b.try_extend_from_within(range),
                Err(StorageError::InvalidRange)
            );
            assert_eq!(b.as_bytes(), b"cdebc");
        }
        b.try_replace_from_within(5..5).unwrap();
        assert!(b.is_empty());
    }

    #[test]
    fn reserve_failure_preserves_owner_and_retry_succeeds() {
        let mut b = ByteBuf::new();
        let mut f = ReserveFault::new(1).unwrap();
        assert_eq!(
            b.try_extend_with_fault(b"abc", &mut f),
            Err(StorageError::AllocationFailed)
        );
        assert_eq!((b.len(), b.capacity(), f.hits()), (0, 0, 1));
        b.try_extend_with_fault(b"abc", &mut f).unwrap();
        let capacity = b.capacity();
        let extra = vec![b'x'; capacity + 1];
        let mut f = ReserveFault::new(1).unwrap();
        assert_eq!(
            b.try_extend_with_fault(&extra, &mut f),
            Err(StorageError::AllocationFailed)
        );
        assert_eq!((b.as_bytes(), b.capacity()), (b"abc".as_slice(), capacity));
        b.try_extend_with_fault(&extra, &mut f).unwrap();
        assert_eq!(f.hits(), 1);
        assert_eq!(b.len(), extra.len() + 3);
    }

    #[test]
    fn replacement_and_self_copy_fail_without_partial_commit() {
        let mut b = ByteBuf::try_from_slice(b"original").unwrap();
        let capacity = b.capacity();
        let mut f = ReserveFault::new(1).unwrap();
        assert_eq!(
            b.try_replace_with_fault(b"replacement", &mut f),
            Err(StorageError::AllocationFailed)
        );
        assert_eq!(
            (b.as_bytes(), b.capacity()),
            (b"original".as_slice(), capacity)
        );
        b.try_replace_with_fault(b"replacement", &mut f).unwrap();
        while b.len() <= b.capacity() / 2 {
            b.try_push(1).unwrap();
        }
        let before = b.as_bytes().to_vec();
        let mut f = ReserveFault::new(1).unwrap();
        assert_eq!(
            b.try_extend_from_within_with_fault(0..b.len(), &mut f),
            Err(StorageError::AllocationFailed)
        );
        assert_eq!(b.as_bytes(), before);
        b.try_extend_from_within_with_fault(0..b.len(), &mut f)
            .unwrap();
        assert_eq!(&b.as_bytes()[before.len()..], before);
    }

    #[test]
    fn c_boundary_is_owned_fallible_and_rejects_interior_nul() {
        let b = ByteBuf::try_from_slice(&[0xff, 0x80]).unwrap();
        let mut f = ReserveFault::new(1).unwrap();
        assert_eq!(
            b.try_nul_terminated_with_fault(&mut f),
            Err(StorageError::AllocationFailed)
        );
        let c = b.try_nul_terminated_with_fault(&mut f).unwrap();
        assert_eq!(c.as_bytes(), &[0xff, 0x80]);
        assert_eq!(c.as_bytes_with_nul(), &[0xff, 0x80, 0]);
        drop(b);
        assert_eq!(c.as_bytes_with_nul(), &[0xff, 0x80, 0]);
        let b = ByteBuf::try_from_slice(b"a\0b").unwrap();
        assert_eq!(
            b.try_nul_terminated(),
            Err(StorageError::InteriorNul { offset: 1 })
        );
        assert_eq!(
            ByteBuf::new()
                .try_nul_terminated()
                .unwrap()
                .as_bytes_with_nul(),
            &[0]
        );
    }

    #[test]
    fn fault_ordinals_count_only_actual_reserves() {
        assert!(matches!(
            ReserveFault::new(0),
            Err(StorageError::InvalidFaultOrdinal)
        ));
        let mut b = ByteBuf::new();
        let mut f = ReserveFault::new(2).unwrap();
        b.try_extend_with_fault(b"", &mut f).unwrap();
        assert_eq!(f.attempts(), 0);
        b.try_extend_with_fault(b"abc", &mut f).unwrap();
        assert_eq!((f.attempts(), f.hits()), (1, 0));
        let extra = vec![0; b.capacity() + 1];
        assert_eq!(
            b.try_extend_with_fault(&extra, &mut f),
            Err(StorageError::AllocationFailed)
        );
        assert_eq!((f.attempts(), f.hits()), (2, 1));
        assert_eq!(b.as_bytes(), b"abc");
    }
}
