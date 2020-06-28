use super::{Attributes, Phase, Record, Strand, NULL_FIELD};

/// A GFF record builder.
#[derive(Debug)]
pub struct Builder {
    reference_sequence_name: String,
    source: String,
    feature: String,
    start: i32,
    end: i32,
    score: Option<f32>,
    strand: Strand,
    phase: Option<Phase>,
    attributes: Attributes,
}

impl Builder {
    /// Creates a GFF record builder.
    ///
    /// Typically, [`gff::Record::builder`] is used instead of calling
    /// [`gff::record::Builder::new`].
    ///
    /// [`gff::Record::builder`]: struct.Record.html#method.builder
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff as gff;
    /// let builder = gff::Record::builder();
    /// ```
    pub fn new() -> Self {
        Builder::default()
    }

    /// Sets a GFF record reference sequence name.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff as gff;
    ///
    /// let record = gff::Record::builder()
    ///     .set_reference_sequence_name(String::from("sq0"))
    ///     .build();
    ///
    /// assert_eq!(record.reference_sequence_name(), "sq0");
    /// ```
    pub fn set_reference_sequence_name(mut self, reference_sequence_name: String) -> Self {
        self.reference_sequence_name = reference_sequence_name;
        self
    }

    /// Sets a GFF record source.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff as gff;
    ///
    /// let record = gff::Record::builder()
    ///     .set_source(String::from("NOODLES"))
    ///     .build();
    ///
    /// assert_eq!(record.source(), "NOODLES");
    /// ```
    pub fn set_source(mut self, source: String) -> Self {
        self.source = source;
        self
    }

    /// Sets a GFF record feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff as gff;
    ///
    /// let record = gff::Record::builder()
    ///     .set_feature(String::from("gene"))
    ///     .build();
    ///
    /// assert_eq!(record.feature(), "gene");
    /// ```
    pub fn set_feature(mut self, feature: String) -> Self {
        self.feature = feature;
        self
    }

    /// Sets a GFF record start position.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff as gff;
    /// let record = gff::Record::builder().set_start(8).build();
    /// assert_eq!(record.start(), 8);
    /// ```
    pub fn set_start(mut self, start: i32) -> Self {
        self.start = start;
        self
    }

    /// Sets a GFF record end position.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff as gff;
    /// let record = gff::Record::builder().set_end(13).build();
    /// assert_eq!(record.end(), 13);
    /// ```
    pub fn set_end(mut self, end: i32) -> Self {
        self.end = end;
        self
    }

    /// Sets a GFF record score.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff as gff;
    /// let record = gff::Record::builder().set_score(21.0).build();
    /// assert_eq!(record.score(), Some(21.0));
    /// ```
    pub fn set_score(mut self, score: f32) -> Self {
        self.score = Some(score);
        self
    }

    /// Sets a GFF record strand.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff::{self as gff, record::Strand};
    ///
    /// let record = gff::Record::builder()
    ///     .set_strand(Strand::Forward)
    ///     .build();
    ///
    /// assert_eq!(record.strand(), Strand::Forward);
    /// ```
    pub fn set_strand(mut self, strand: Strand) -> Self {
        self.strand = strand;
        self
    }

    /// Sets a GFF record phase.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff::{self as gff, record::Phase};
    /// let record = gff::Record::builder().set_phase(Phase::Zero).build();
    /// assert_eq!(record.phase(), Some(Phase::Zero));
    /// ```
    pub fn set_phase(mut self, phase: Phase) -> Self {
        self.phase = Some(phase);
        self
    }

    /// Sets GFF record attributes.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_gff::{
    ///     self as gff,
    ///     record::{attributes::Entry, Attributes},
    /// };
    ///
    /// let attributes = Attributes::from(vec![
    ///     Entry::new(String::from("gene_id"), String::from("ndls0")),
    /// ]);
    ///
    /// let record = gff::Record::builder()
    ///     .set_attributes(attributes.clone())
    ///     .build();
    ///
    /// assert_eq!(record.attributes(), &attributes);
    /// ```
    pub fn set_attributes(mut self, attributes: Attributes) -> Self {
        self.attributes = attributes;
        self
    }

    /// Builds a GFF record.
    ///
    /// # Example
    ///
    /// ```
    /// use noodles_gff as gff;
    /// let record = gff::Record::builder().build();
    /// ```
    pub fn build(self) -> Record {
        Record {
            reference_sequence_name: self.reference_sequence_name,
            source: self.source,
            feature: self.feature,
            start: self.start,
            end: self.end,
            score: self.score,
            strand: self.strand,
            phase: self.phase,
            attributes: self.attributes,
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Builder {
            reference_sequence_name: NULL_FIELD.into(),
            source: NULL_FIELD.into(),
            feature: NULL_FIELD.into(),
            start: 1,
            end: 1,
            score: None,
            strand: Strand::default(),
            phase: None,
            attributes: Attributes::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::record::attributes::Entry;

    use super::*;

    #[test]
    fn test_default() {
        let record = Builder::default().build();

        assert_eq!(record.reference_sequence_name(), ".");
        assert_eq!(record.source(), ".");
        assert_eq!(record.feature(), ".");
        assert_eq!(record.start(), 1);
        assert_eq!(record.end(), 1);
        assert!(record.score().is_none());
        assert_eq!(record.strand(), Strand::default());
        assert!(record.phase().is_none());
        assert!(record.attributes().entries().is_empty());
    }

    #[test]
    fn test_build() {
        let attributes = Attributes::from(vec![Entry::new(
            String::from("gene_id"),
            String::from("ndls0"),
        )]);

        let record = Builder::new()
            .set_reference_sequence_name(String::from("sq0"))
            .set_source(String::from("NOODLES"))
            .set_feature(String::from("CDS"))
            .set_start(8)
            .set_end(13)
            .set_score(21.0)
            .set_strand(Strand::Forward)
            .set_phase(Phase::Zero)
            .set_attributes(attributes.clone())
            .build();

        assert_eq!(record.reference_sequence_name(), "sq0");
        assert_eq!(record.source(), "NOODLES");
        assert_eq!(record.feature(), "CDS");
        assert_eq!(record.start(), 8);
        assert_eq!(record.end(), 13);
        assert_eq!(record.score(), Some(21.0));
        assert_eq!(record.strand(), Strand::Forward);
        assert_eq!(record.phase(), Some(Phase::Zero));
        assert_eq!(record.attributes(), &attributes);
    }
}