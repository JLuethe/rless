//! Numeric domains remain distinct; a coordinate still needs owner/range validation.
//!
//! ```compile_fail
//! use rless_safe_core::coordinate::{BufferIndex, DisplayColumn};
//! let column: DisplayColumn = BufferIndex::new(1);
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateError {
    NegativeSourceOffset,
    BeforeStart,
    Overflow,
    NativeRange,
    IndexRange,
    ZeroLine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceOffset(i64);

impl SourceOffset {
    pub const ZERO: Self = Self(0);
    pub fn new(value: u64) -> Result<Self, CoordinateError> {
        i64::try_from(value)
            .map(Self)
            .map_err(|_| CoordinateError::Overflow)
    }
    pub fn from_signed(value: i64) -> Result<Self, CoordinateError> {
        if value < 0 {
            Err(CoordinateError::NegativeSourceOffset)
        } else {
            Ok(Self(value))
        }
    }
    pub fn get(self) -> i64 {
        self.0
    }
    pub fn checked_add(self, bytes: u64) -> Result<Self, CoordinateError> {
        let bytes = i64::try_from(bytes).map_err(|_| CoordinateError::Overflow)?;
        self.0
            .checked_add(bytes)
            .map(Self)
            .ok_or(CoordinateError::Overflow)
    }
    pub fn distance_from(self, start: Self) -> Result<u64, CoordinateError> {
        if self < start {
            return Err(CoordinateError::BeforeStart);
        }
        Ok((self.0 - start.0) as u64)
    }
    /// GNU regex's narrow offset domain does not limit the file offset domain.
    pub fn try_regex_offset(self) -> Result<i32, CoordinateError> {
        i32::try_from(self.0).map_err(|_| CoordinateError::NativeRange)
    }
}

macro_rules! coordinate {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(usize);
        impl $name {
            pub fn new(value: usize) -> Self {
                Self(value)
            }
            pub fn get(self) -> usize {
                self.0
            }
            pub fn checked_add(self, amount: usize) -> Result<Self, CoordinateError> {
                self.0
                    .checked_add(amount)
                    .map(Self)
                    .ok_or(CoordinateError::Overflow)
            }
            pub fn try_native_i32(self) -> Result<i32, CoordinateError> {
                i32::try_from(self.0).map_err(|_| CoordinateError::NativeRange)
            }
        }
    };
}

coordinate!(BufferIndex);
coordinate!(DisplayRow);
coordinate!(DisplayColumn);

/// One-based logical line; unknown/unavailable uses an explicit outer state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LogicalLine(u64);
impl LogicalLine {
    pub fn new(value: u64) -> Result<Self, CoordinateError> {
        if value == 0 {
            Err(CoordinateError::ZeroLine)
        } else {
            Ok(Self(value))
        }
    }
    pub fn get(self) -> u64 {
        self.0
    }
    pub fn checked_add(self, lines: u64) -> Result<Self, CoordinateError> {
        self.0
            .checked_add(lines)
            .map(Self)
            .ok_or(CoordinateError::Overflow)
    }
    pub fn try_native_i64(self) -> Result<i64, CoordinateError> {
        i64::try_from(self.0).map_err(|_| CoordinateError::NativeRange)
    }
}

/// A description of initialized retained bytes, not an owning file/view handle.
/// The future input owner must revalidate this description after invalidation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetainedRange {
    start: SourceOffset,
    end: SourceOffset,
}
impl RetainedRange {
    pub fn new(start: SourceOffset, initialized_len: usize) -> Result<Self, CoordinateError> {
        let length = u64::try_from(initialized_len).map_err(|_| CoordinateError::Overflow)?;
        Ok(Self {
            start,
            end: start.checked_add(length)?,
        })
    }
    pub fn start(self) -> SourceOffset {
        self.start
    }
    pub fn end(self) -> SourceOffset {
        self.end
    }
    pub fn boundary(self, position: SourceOffset) -> Result<BufferIndex, CoordinateError> {
        let distance = position.distance_from(self.start)?;
        if position > self.end {
            return Err(CoordinateError::IndexRange);
        }
        usize::try_from(distance)
            .map(BufferIndex)
            .map_err(|_| CoordinateError::IndexRange)
    }
    pub fn index(self, position: SourceOffset) -> Result<BufferIndex, CoordinateError> {
        if position == self.end {
            return Err(CoordinateError::IndexRange);
        }
        self.boundary(position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_domain_rejects_sentinels_and_preserves_wide_offsets() {
        assert_eq!(
            SourceOffset::from_signed(-1),
            Err(CoordinateError::NegativeSourceOffset)
        );
        assert_eq!(SourceOffset::new(u64::MAX), Err(CoordinateError::Overflow));
        let wide = SourceOffset::new(1u64 << 33).unwrap();
        assert_eq!(wide.get(), 1i64 << 33);
        assert_eq!(wide.try_regex_offset(), Err(CoordinateError::NativeRange));
        let max = SourceOffset::new(i64::MAX as u64).unwrap();
        assert_eq!(max.checked_add(1), Err(CoordinateError::Overflow));
        assert_eq!(max.distance_from(SourceOffset::ZERO), Ok(i64::MAX as u64));
        assert_eq!(
            SourceOffset::ZERO.distance_from(max),
            Err(CoordinateError::BeforeStart)
        );
    }
    #[test]
    fn native_conversion_boundaries_do_not_truncate() {
        assert_eq!(
            SourceOffset::new(i32::MAX as u64)
                .unwrap()
                .try_regex_offset(),
            Ok(i32::MAX)
        );
        assert_eq!(
            SourceOffset::new(i32::MAX as u64 + 1)
                .unwrap()
                .try_regex_offset(),
            Err(CoordinateError::NativeRange)
        );
        assert_eq!(
            DisplayColumn::new(i32::MAX as usize).try_native_i32(),
            Ok(i32::MAX)
        );
        assert_eq!(
            DisplayRow::new(i32::MAX as usize + 1).try_native_i32(),
            Err(CoordinateError::NativeRange)
        );
        assert_eq!(
            BufferIndex::new(usize::MAX).checked_add(1),
            Err(CoordinateError::Overflow)
        );
        assert_eq!(LogicalLine::new(0), Err(CoordinateError::ZeroLine));
        assert_eq!(
            LogicalLine::new(u64::MAX).unwrap().checked_add(1),
            Err(CoordinateError::Overflow)
        );
        assert_eq!(
            LogicalLine::new(u64::MAX).unwrap().try_native_i64(),
            Err(CoordinateError::NativeRange)
        );
    }
    #[test]
    fn retained_indices_and_end_boundaries_are_distinct() {
        let start = SourceOffset::new(1u64 << 33).unwrap();
        let range = RetainedRange::new(start, 3).unwrap();
        assert_eq!(
            range.index(start.checked_add(2).unwrap()),
            Ok(BufferIndex::new(2))
        );
        assert_eq!(range.index(range.end()), Err(CoordinateError::IndexRange));
        assert_eq!(range.boundary(range.end()), Ok(BufferIndex::new(3)));
        assert_eq!(
            range.boundary(start.checked_add(4).unwrap()),
            Err(CoordinateError::IndexRange)
        );
        assert_eq!(
            range.index(SourceOffset::ZERO),
            Err(CoordinateError::BeforeStart)
        );
        let empty = RetainedRange::new(start, 0).unwrap();
        assert_eq!(empty.index(start), Err(CoordinateError::IndexRange));
        assert_eq!(empty.boundary(start), Ok(BufferIndex::new(0)));
        assert!(RetainedRange::new(SourceOffset::new(i64::MAX as u64).unwrap(), 1).is_err());
    }
}
