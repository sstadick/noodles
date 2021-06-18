#![warn(missing_docs)]

//! **noodles-fasta** handles and reading and writing of the FASTA format.
//!
//! FASTA is a text format with no formal specification and only has de facto rules. It typically
//! consists of a list of records, each with a definition on the first line and a sequence in the
//! following lines.
//!
//! The definition starts with a `>` (greater than) character, and directly after it is the
//! reference sequence name. Optionally, whitespace may be used a delimiter for an extra
//! description or metadata of the sequence. For example,
//!
//! ```text
//!  reference sequence name
//!  | |
//! >sq0 LN:13
//!      |   |
//!      description
//! ```
//!
//! The sequence is effectively a byte array of characters representing a base. It is typically
//! hard wrapped at an arbitrary width. For example, the following makes up the sequence
//! `ACGTNACTGG`.
//!
//! ```text
//! ACGT
//! NACT
//! GG
//! ```
//!
//! # Examples
//!
//! ## Read all records in a FASTA file
//!
//! ```no_run
//! # use std::{fs::File, io::{self, BufReader}};
//! use noodles_fasta as fasta;
//!
//! let mut reader = File::open("reference.fa")
//!     .map(BufReader::new)
//!     .map(fasta::Reader::new)?;
//!
//! for result in reader.records() {
//!     let record = result?;
//!
//!     println!(
//!         "{}\t{}",
//!         record.reference_sequence_name(),
//!         record.sequence().len()
//!     );
//! }
//! # Ok::<(), io::Error>(())
//! ```

pub mod fai;
mod indexer;
pub mod reader;
pub mod record;
mod writer;

pub use self::{reader::Reader, record::Record, writer::Writer};

use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
};

use self::indexer::Indexer;

/// Indexes a FASTA file.
///
/// # Examples
///
/// ```no_run
/// # use std::io;
/// use noodles_fasta as fasta;
/// let index = fasta::index("reference.fa")?;
/// # Ok::<(), io::Error>(())
/// ```
pub fn index<P>(src: P) -> io::Result<fai::Index>
where
    P: AsRef<Path>,
{
    let mut indexer = File::open(src).map(BufReader::new).map(Indexer::new)?;
    let mut index = Vec::new();

    while let Some(i) = indexer.index_record()? {
        index.push(i);
    }

    Ok(index)
}
