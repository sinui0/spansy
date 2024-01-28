use std::ops::Range;

use utils::range::{RangeSet, RangeUnion};

/// A range within the source bytes.
///
/// This can be either a contiguous range or a range set.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceRange {
    /// A contiguous range.
    Range(Range<usize>),
    /// A set of ranges
    RangeSet(RangeSet<usize>),
}

impl SourceRange {
    /// Returns the number of bytes in the range.
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Self::Range(range) => range.len(),
            Self::RangeSet(range_set) => range_set.len(),
        }
    }

    /// Returns `true` if the range is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Range(range) => range.is_empty(),
            Self::RangeSet(range_set) => range_set.is_empty(),
        }
    }

    /// Shifts the range right by the given offset.
    ///
    /// # Panics
    ///
    /// Panics if the offset causes the range to overflow `usize::MAX`.
    pub(crate) fn offset(&mut self, offset: usize) {
        match self {
            Self::Range(range) => {
                range.start += offset;
                range.end += offset;
            }
            Self::RangeSet(range_set) => range_set.shift_right(&offset),
        }
    }

    /// Partitions the range at the given index and offsets the right side by the given distance.
    ///
    /// # Panics
    ///
    /// - if the index is not within the range.
    /// - if the offsetting causes the range to overflow `usize::MAX`.
    #[allow(dead_code)]
    pub(crate) fn partition(&mut self, at: usize, distance: usize) {
        match self {
            SourceRange::Range(range) => {
                *self = SourceRange::RangeSet(RangeSet::from(range.clone()));
                self.partition(at, distance);
            }
            SourceRange::RangeSet(set) => {
                let mut right = set.split_off(&at);
                right.shift_right(&distance);
                *set = set.union(&right);
            }
        }
    }
}

impl From<SourceRange> for RangeSet<usize> {
    fn from(range: SourceRange) -> Self {
        match range {
            SourceRange::Range(range) => RangeSet::from(range),
            SourceRange::RangeSet(range_set) => range_set,
        }
    }
}

impl PartialEq<Range<usize>> for SourceRange {
    fn eq(&self, other: &Range<usize>) -> bool {
        match self {
            SourceRange::Range(range) => range == other,
            SourceRange::RangeSet(_) => false,
        }
    }
}

impl PartialEq<SourceRange> for Range<usize> {
    fn eq(&self, other: &SourceRange) -> bool {
        other == self
    }
}

impl PartialEq<Range<usize>> for &SourceRange {
    fn eq(&self, other: &Range<usize>) -> bool {
        *self == other
    }
}

impl PartialEq<SourceRange> for &Range<usize> {
    fn eq(&self, other: &SourceRange) -> bool {
        other == *self
    }
}
