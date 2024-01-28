use std::ops::Range;

use utils::range::{RangeIter, RangeSet, RangeSetIter, RangeUnion};

/// A range within the source bytes.
///
/// This can be either a contiguous range or a range set.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SourceRange {
    /// A contiguous range.
    Range(Range<usize>),
    /// A set of ranges
    RangeSet(RangeSet<usize>),
}

impl SourceRange {
    /// Returns an iterator over the indices of the range.
    pub fn iter(&self) -> SourceRangeIter<'_> {
        match self {
            Self::Range(range) => SourceRangeIter(SourceRangeIterInner::Range(range.clone())),
            Self::RangeSet(range_set) => {
                SourceRangeIter(SourceRangeIterInner::RangeSet(range_set.iter()))
            }
        }
    }

    /// Returns an iterator over the ranges of the range.
    pub fn iter_ranges(&self) -> impl Iterator<Item = Range<usize>> + '_ {
        match self {
            Self::Range(range) => {
                SourceRangesIter(SourceRangesIterInner::Range(Some(range.clone())))
            }
            Self::RangeSet(set) => {
                SourceRangesIter(SourceRangesIterInner::RangeSet(set.iter_ranges()))
            }
        }
    }

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

/// An iterator over the indices of a source range.
pub struct SourceRangeIter<'a>(SourceRangeIterInner<'a>);

enum SourceRangeIterInner<'a> {
    Range(Range<usize>),
    RangeSet(RangeSetIter<'a, usize>),
}

impl Iterator for SourceRangeIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            SourceRangeIterInner::Range(range) => range.next(),
            SourceRangeIterInner::RangeSet(set) => set.next(),
        }
    }
}

/// An iterator over the ranges of a source range.
pub struct SourceRangesIter<'a>(SourceRangesIterInner<'a>);

enum SourceRangesIterInner<'a> {
    Range(Option<Range<usize>>),
    RangeSet(RangeIter<'a, usize>),
}

impl Iterator for SourceRangesIter<'_> {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            SourceRangesIterInner::Range(range) => range.take(),
            SourceRangesIterInner::RangeSet(set) => set.next(),
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
