//! Coordinate-sorted index (CSI) reference sequence and fields.

mod bin;
mod metadata;

pub use self::{bin::Bin, metadata::Metadata};

use bit_vec::BitVec;

/// A CSI reference sequence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceSequence {
    bins: Vec<Bin>,
    metadata: Option<Metadata>,
}

impl ReferenceSequence {
    /// Creates a CSI reference sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_csi::index::ReferenceSequence;
    /// let reference_sequence = ReferenceSequence::new(Vec::new(), None);
    /// ```
    pub fn new(bins: Vec<Bin>, metadata: Option<Metadata>) -> Self {
        Self { bins, metadata }
    }

    /// Returns the list of bins in the reference sequence.
    ///
    /// This list does not include the metadata pseudo-bin. Use [`Self::metadata`] instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_csi::index::ReferenceSequence;
    /// let reference_sequence = ReferenceSequence::new(Vec::new(), None);
    /// assert!(reference_sequence.bins().is_empty());
    /// ```
    pub fn bins(&self) -> &[Bin] {
        &self.bins
    }

    /// Returns metadata for this reference sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_bgzf as bgzf;
    /// use noodles_csi::index::{reference_sequence::Metadata, ReferenceSequence};
    ///
    /// let reference_sequence = ReferenceSequence::new(Vec::new(), Some(Metadata::new(
    ///     bgzf::VirtualPosition::from(610),
    ///     bgzf::VirtualPosition::from(1597),
    ///     55,
    ///     0,
    /// )));
    ///
    /// assert!(reference_sequence.metadata().is_some());
    /// ```
    pub fn metadata(&self) -> Option<&Metadata> {
        self.metadata.as_ref()
    }

    /// Returns a list of bins in this reference sequence that intersects the given range.
    ///
    /// `start` and `end` are 1-based, inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_csi::index::ReferenceSequence;
    /// let reference_sequence = ReferenceSequence::new(Vec::new(), None);
    /// let query_bins = reference_sequence.query(14, 5, 8, 13);
    /// assert!(query_bins.is_empty());
    /// ```
    pub fn query(&self, min_shift: i32, depth: i32, start: i64, end: i64) -> Vec<&Bin> {
        let max_bin_id = Bin::max_id(depth);
        let mut region_bins = BitVec::from_elem(max_bin_id as usize, false);

        reg2bins(start, end, min_shift, depth, &mut region_bins);

        self.bins()
            .iter()
            .filter(|b| region_bins[b.id() as usize])
            .collect()
    }
}

// `CSIv1.pdf` (2020-07-21)
#[allow(clippy::many_single_char_names)]
fn reg2bins(beg: i64, end: i64, min_shift: i32, depth: i32, bins: &mut BitVec) {
    let mut l = 0;
    let mut t = 0;
    let mut s = min_shift + depth * 3;

    while l <= depth {
        let b = t + (beg >> s);
        let e = t + (end >> s);

        for i in b..=e {
            bins.set(i as usize, true);
        }

        s -= 3;
        t += 1 << (l * 3);
        l += 1;
    }
}
