use crate::StorageError;

pub(crate) trait Gate {
    fn before_reserve(&mut self) -> Result<(), StorageError>;
}

pub(crate) struct Unfaulted;
impl Gate for Unfaulted {
    fn before_reserve(&mut self) -> Result<(), StorageError> {
        Ok(())
    }
}

pub(crate) fn checked_len<T>(current: usize, additional: usize) -> Result<usize, StorageError> {
    let needed = current
        .checked_add(additional)
        .ok_or(StorageError::CapacityOverflow)?;
    let bytes = needed
        .checked_mul(std::mem::size_of::<T>())
        .ok_or(StorageError::CapacityOverflow)?;
    if bytes > isize::MAX as usize {
        return Err(StorageError::CapacityOverflow);
    }
    Ok(needed)
}

pub(crate) fn reserve<T>(
    values: &mut Vec<T>,
    additional: usize,
    gate: &mut impl Gate,
) -> Result<(), StorageError> {
    let needed = checked_len::<T>(values.len(), additional)?;
    if needed > values.capacity() {
        gate.before_reserve()?;
        values
            .try_reserve(additional)
            .map_err(|_| StorageError::AllocationFailed)?;
    }
    Ok(())
}

/// Component-only rejection at the selected actual reserve attempt.
/// A reserve satisfied by existing capacity does not consume an ordinal.
#[cfg(any(test, feature = "fault-testing"))]
#[derive(Debug)]
pub struct ReserveFault {
    fail_at: usize,
    attempts: usize,
    hits: usize,
}

#[cfg(any(test, feature = "fault-testing"))]
impl ReserveFault {
    pub fn new(fail_at: usize) -> Result<Self, StorageError> {
        if fail_at == 0 {
            return Err(StorageError::InvalidFaultOrdinal);
        }
        Ok(Self {
            fail_at,
            attempts: 0,
            hits: 0,
        })
    }
    pub fn attempts(&self) -> usize {
        self.attempts
    }
    pub fn hits(&self) -> usize {
        self.hits
    }
}

#[cfg(any(test, feature = "fault-testing"))]
impl Gate for ReserveFault {
    fn before_reserve(&mut self) -> Result<(), StorageError> {
        self.attempts = self
            .attempts
            .checked_add(1)
            .ok_or(StorageError::CapacityOverflow)?;
        if self.attempts == self.fail_at {
            self.hits += 1;
            Err(StorageError::AllocationFailed)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn length_and_element_bytes_are_checked_before_any_reserve() {
        assert_eq!(
            checked_len::<u8>(usize::MAX, 1),
            Err(StorageError::CapacityOverflow)
        );
        assert_eq!(
            checked_len::<u8>(isize::MAX as usize, 1),
            Err(StorageError::CapacityOverflow)
        );
        assert_eq!(
            checked_len::<u64>(0, usize::MAX / 8),
            Err(StorageError::CapacityOverflow)
        );
        assert_eq!(checked_len::<u8>(0, 0), Ok(0));
    }
}
