use std::collections::HashMap;

use super::{AlternativeNames, Md5Checksum, MoleculeTopology, ReferenceSequence, Tag};

/// A SAM header reference sequence builder.
#[derive(Debug, Default)]
pub struct Builder {
    name: Option<String>,
    len: Option<i32>,
    alternative_locus: Option<String>,
    alternative_names: Option<AlternativeNames>,
    assembly_id: Option<String>,
    description: Option<String>,
    md5_checksum: Option<Md5Checksum>,
    species: Option<String>,
    molecule_topology: Option<MoleculeTopology>,
    uri: Option<String>,
    fields: HashMap<Tag, String>,
}

impl Builder {
    /// Sets a reference sequence name.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    ///
    /// let reference_sequence = ReferenceSequence::builder()
    ///     .set_name("sq0")
    ///     .set_length(13)
    ///     .build();
    ///
    /// assert_eq!(reference_sequence.name(), "sq0");
    /// ```
    pub fn set_name<I>(mut self, name: I) -> Self
    where
        I: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    /// Sets a reference sequence length.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    ///
    /// let reference_sequence = ReferenceSequence::builder()
    ///     .set_name("sq0")
    ///     .set_length(13)
    ///     .build();
    ///
    /// assert_eq!(reference_sequence.len(), 13);
    /// ```
    pub fn set_length(mut self, len: i32) -> Self {
        self.len = Some(len);
        self
    }

    /// Sets an alternative locus.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    ///
    /// let reference_sequence = ReferenceSequence::builder()
    ///     .set_name("sq0")
    ///     .set_length(13)
    ///     .set_alternative_locus("sq0_alt")
    ///     .build();
    ///
    /// assert_eq!(reference_sequence.alternative_locus(), Some("sq0_alt"));
    /// ```
    pub fn set_alternative_locus<I>(mut self, alternative_locus: I) -> Self
    where
        I: Into<String>,
    {
        self.alternative_locus = Some(alternative_locus.into());
        self
    }

    /// Sets alternative names.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::{
    ///     reference_sequence::{alternative_names, AlternativeNames},
    ///     ReferenceSequence,
    /// };
    ///
    /// let alternative_names: AlternativeNames = "0,SQ.0".parse()?;
    ///
    /// let reference_sequence = ReferenceSequence::builder()
    ///     .set_name("sq0")
    ///     .set_length(13)
    ///     .set_alternative_names(alternative_names.clone())
    ///     .build();
    ///
    /// assert_eq!(reference_sequence.alternative_names(), Some(&alternative_names));
    /// # Ok::<(), alternative_names::ParseError>(())
    /// ```
    pub fn set_alternative_names(mut self, alternative_names: AlternativeNames) -> Self {
        self.alternative_names = Some(alternative_names);
        self
    }

    /// Sets a genome assembly ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    ///
    /// let reference_sequence = ReferenceSequence::builder()
    ///     .set_name("sq0")
    ///     .set_length(13)
    ///     .set_assembly_id("ref")
    ///     .build();
    ///
    /// assert_eq!(reference_sequence.assembly_id(), Some("ref"));
    /// ```
    pub fn set_assembly_id<I>(mut self, assembly_id: I) -> Self
    where
        I: Into<String>,
    {
        self.assembly_id = Some(assembly_id.into());
        self
    }

    /// Sets a description.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    ///
    /// let reference_sequence = ReferenceSequence::builder()
    ///     .set_name("sq0")
    ///     .set_length(13)
    ///     .set_description("noodles")
    ///     .build();
    ///
    /// assert_eq!(reference_sequence.description(), Some("noodles"));
    /// ```
    pub fn set_description<I>(mut self, description: I) -> Self
    where
        I: Into<String>,
    {
        self.description = Some(description.into());
        self
    }

    /// Sets an MD5 checksum.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::{reference_sequence::Md5Checksum, ReferenceSequence};
    ///
    /// let reference_sequence = ReferenceSequence::builder()
    ///     .set_name("sq0")
    ///     .set_length(13)
    ///     .set_md5_checksum(Md5Checksum::from([
    ///         0xd7, 0xeb, 0xa3, 0x11, 0x42, 0x1b, 0xbc, 0x9d,
    ///         0x3a, 0xda, 0x44, 0x70, 0x9d, 0xd6, 0x15, 0x34,
    ///     ]))
    ///     .build();
    ///
    /// assert_eq!(reference_sequence.md5_checksum(), Some(Md5Checksum::from([
    ///     0xd7, 0xeb, 0xa3, 0x11, 0x42, 0x1b, 0xbc, 0x9d,
    ///     0x3a, 0xda, 0x44, 0x70, 0x9d, 0xd6, 0x15, 0x34,
    /// ])));
    /// ```
    pub fn set_md5_checksum(mut self, md5_checksum: Md5Checksum) -> Self {
        self.md5_checksum = Some(md5_checksum);
        self
    }

    /// Sets a species.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    ///
    /// let reference_sequence = ReferenceSequence::builder()
    ///     .set_name("sq0")
    ///     .set_length(13)
    ///     .set_species("human")
    ///     .build();
    ///
    /// assert_eq!(reference_sequence.species(), Some("human"));
    /// ```
    pub fn set_species<I>(mut self, species: I) -> Self
    where
        I: Into<String>,
    {
        self.species = Some(species.into());
        self
    }

    /// Sets a molecule topology.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::{reference_sequence::MoleculeTopology, ReferenceSequence};
    ///
    /// let reference_sequence = ReferenceSequence::builder()
    ///     .set_name("sq0")
    ///     .set_length(13)
    ///     .set_molecule_topology(MoleculeTopology::Linear)
    ///     .build();
    ///
    /// assert_eq!(reference_sequence.molecule_topology(), Some(MoleculeTopology::Linear));
    /// ```
    pub fn set_molecule_topology(mut self, molecule_topology: MoleculeTopology) -> Self {
        self.molecule_topology = Some(molecule_topology);
        self
    }

    /// Sets a URI.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    ///
    /// let reference_sequence = ReferenceSequence::builder()
    ///     .set_name("sq0")
    ///     .set_length(13)
    ///     .set_uri("file:///tmp/ref.fasta")
    ///     .build();
    ///
    /// assert_eq!(reference_sequence.uri(), Some("file:///tmp/ref.fasta"));
    /// ```
    pub fn set_uri<I>(mut self, uri: I) -> Self
    where
        I: Into<String>,
    {
        self.uri = Some(uri.into());
        self
    }

    /// Inserts a tag-raw value pair.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::{reference_sequence::Tag, ReferenceSequence};
    ///
    /// let zn = Tag::Other(String::from("zn"));
    ///
    /// let reference_sequence = ReferenceSequence::builder()
    ///     .set_name("sq0")
    ///     .set_length(13)
    ///     .insert(zn.clone(), String::from("noodles"))
    ///     .build();
    ///
    /// assert_eq!(
    ///     reference_sequence.fields().get(&zn),
    ///     Some(&String::from("noodles"))
    /// );
    /// ```
    pub fn insert<I>(mut self, tag: Tag, value: I) -> Self
    where
        I: Into<String>,
    {
        self.fields.insert(tag, value.into());
        self
    }

    /// Builds a reference sequence.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_sam::header::ReferenceSequence;
    /// let reference_sequence = ReferenceSequence::builder()
    ///     .set_name("sq0")
    ///     .set_length(13)
    ///     .build();
    /// ```
    pub fn build(self) -> ReferenceSequence {
        ReferenceSequence {
            name: self.name.expect("missing name"),
            len: self.len.expect("missing len"),
            alternative_locus: self.alternative_locus,
            alternative_names: self.alternative_names,
            assembly_id: self.assembly_id,
            description: self.description,
            md5_checksum: self.md5_checksum,
            species: self.species,
            molecule_topology: self.molecule_topology,
            uri: self.uri,
            fields: self.fields,
        }
    }
}
