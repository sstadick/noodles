use std::io::{self, Write};

use byteorder::{LittleEndian, WriteBytesExt};
use noodles_bgzf::{self as bgzf, index::Chunk};

use super::{
    index::{self, reference_sequence::Bin, ReferenceSequence},
    Index, MAGIC_NUMBER,
};

const NUL: u8 = b'\x00';

/// A tabix writer.
pub struct Writer<W>
where
    W: Write,
{
    inner: bgzf::Writer<W>,
}

impl<W> Writer<W>
where
    W: Write,
{
    /// Creates a tabix writer.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    /// let writer = tabix::Writer::new(Vec::new());
    /// ```
    pub fn new(writer: W) -> Self {
        Self {
            inner: bgzf::Writer::new(writer),
        }
    }

    /// Returns a reference to the underlying writer.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_tabix as tabix;
    /// let writer = tabix::Writer::new(Vec::new());
    /// assert!(writer.get_ref().is_empty());
    /// ```
    pub fn get_ref(&self) -> &W {
        self.inner.get_ref()
    }

    /// Attempts to finish the output stream.
    ///
    /// This is typically only manually called if the underlying stream is needed before the writer
    /// is dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io;
    /// use noodles_tabix as tabix;
    /// let mut writer = tabix::Writer::new(Vec::new());
    /// writer.try_finish()?;
    /// # Ok::<(), io::Error>(())
    /// ```
    pub fn try_finish(&mut self) -> io::Result<()> {
        self.inner.try_finish()
    }

    /// Writes a tabix index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io;
    /// use noodles_tabix as tabix;
    /// let index = tabix::Index::default();
    /// let mut writer = tabix::Writer::new(Vec::new());
    /// writer.write_index(&index)?;
    /// # Ok::<(), io::Error>(())
    /// ```
    pub fn write_index(&mut self, index: &Index) -> io::Result<()> {
        write_magic(&mut self.inner)?;

        let n_ref = index.reference_sequences().len() as i32;
        self.inner.write_i32::<LittleEndian>(n_ref)?;

        write_header(&mut self.inner, index.header())?;

        // Add 1 for each trailing nul.
        let l_nm = index
            .reference_sequence_names()
            .iter()
            .map(|n| n.len() + 1)
            .sum::<usize>() as i32;
        self.inner.write_i32::<LittleEndian>(l_nm)?;

        for reference_sequence_name in index.reference_sequence_names() {
            self.inner.write_all(reference_sequence_name.as_bytes())?;
            self.inner.write_u8(NUL)?;
        }

        for reference_sequence in index.reference_sequences() {
            write_reference_sequence(&mut self.inner, reference_sequence)?;
        }

        if let Some(n_no_coor) = index.unmapped_read_count() {
            self.inner.write_u64::<LittleEndian>(n_no_coor)?;
        }

        Ok(())
    }
}

fn write_magic<W>(writer: &mut W) -> io::Result<()>
where
    W: Write,
{
    writer.write_all(MAGIC_NUMBER)
}

fn write_header<W>(writer: &mut W, header: &index::Header) -> io::Result<()>
where
    W: Write,
{
    let format = i32::from(header.format());
    writer.write_i32::<LittleEndian>(format)?;

    let col_seq = header.reference_sequence_name_index() as i32;
    writer.write_i32::<LittleEndian>(col_seq)?;

    let col_beg = header.start_position_index() as i32;
    writer.write_i32::<LittleEndian>(col_beg)?;

    let col_end = header
        .end_position_index()
        .map(|i| i as i32)
        .unwrap_or_default();
    writer.write_i32::<LittleEndian>(col_end)?;

    let meta = i32::from(header.line_comment_prefix());
    writer.write_i32::<LittleEndian>(meta)?;

    let skip = header.line_skip_count() as i32;
    writer.write_i32::<LittleEndian>(skip)?;

    Ok(())
}

pub fn write_reference_sequence<W>(writer: &mut W, reference: &ReferenceSequence) -> io::Result<()>
where
    W: Write,
{
    let mut n_bin = reference.bins().len() as i32;

    if reference.metadata().is_some() {
        n_bin += 1;
    }

    writer.write_i32::<LittleEndian>(n_bin)?;

    for bin in reference.bins() {
        write_bin(writer, bin)?;
    }

    if let Some(metadata) = reference.metadata() {
        let bin = Bin::from(metadata.clone());
        write_bin(writer, &bin)?;
    }

    let n_intv = reference.intervals().len() as i32;
    writer.write_i32::<LittleEndian>(n_intv)?;

    for interval in reference.intervals() {
        let ioff = u64::from(*interval);
        writer.write_u64::<LittleEndian>(ioff)?;
    }

    Ok(())
}

pub fn write_bin<W>(writer: &mut W, bin: &Bin) -> io::Result<()>
where
    W: Write,
{
    writer.write_u32::<LittleEndian>(bin.id())?;

    let n_chunk = bin.chunks().len() as i32;
    writer.write_i32::<LittleEndian>(n_chunk)?;

    for chunk in bin.chunks() {
        write_chunk(writer, chunk)?;
    }

    Ok(())
}

fn write_chunk<W>(writer: &mut W, chunk: &Chunk) -> io::Result<()>
where
    W: Write,
{
    let cnk_beg = u64::from(chunk.start());
    writer.write_u64::<LittleEndian>(cnk_beg)?;

    let cnk_end = u64::from(chunk.end());
    writer.write_u64::<LittleEndian>(cnk_end)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::{BufWriter, Read};

    use noodles_bgzf as bgzf;

    use super::*;

    #[test]
    fn test_write_index() -> io::Result<()> {
        let chunks = vec![Chunk::new(
            bgzf::VirtualPosition::from(509268599425),
            bgzf::VirtualPosition::from(509268599570),
        )];
        let bins = vec![Bin::new(16385, chunks)];
        let intervals = vec![bgzf::VirtualPosition::from(337)];
        let references = vec![ReferenceSequence::new(bins, intervals, None)];

        let index = Index::builder()
            .set_reference_sequence_names(vec![String::from("sq0"), String::from("sq1")])
            .set_reference_sequences(references)
            .build();

        let mut actual_writer = Writer::new(Vec::new());
        actual_writer.write_index(&index)?;

        actual_writer.try_finish()?;

        let mut expected_writer = BufWriter::new(Vec::new());
        // magic
        expected_writer.write_all(MAGIC_NUMBER)?;
        // n_ref
        expected_writer.write_i32::<LittleEndian>(1)?;
        // format
        expected_writer.write_i32::<LittleEndian>(0)?;
        // col_seq
        expected_writer.write_i32::<LittleEndian>(1)?;
        // col_beg
        expected_writer.write_i32::<LittleEndian>(4)?;
        // col_end
        expected_writer.write_i32::<LittleEndian>(5)?;
        // meta
        expected_writer.write_i32::<LittleEndian>(i32::from(b'#'))?;
        // skip
        expected_writer.write_i32::<LittleEndian>(0)?;
        // l_nm
        expected_writer.write_i32::<LittleEndian>(8)?;
        // names
        expected_writer.write_all(b"sq0\x00sq1\x00")?;
        // n_bin
        expected_writer.write_u32::<LittleEndian>(1)?;
        // bin
        expected_writer.write_u32::<LittleEndian>(16385)?;
        // n_chunk
        expected_writer.write_u32::<LittleEndian>(1)?;
        // chunk_beg
        expected_writer.write_u64::<LittleEndian>(509268599425)?;
        // chunk_end
        expected_writer.write_u64::<LittleEndian>(509268599570)?;
        // n_intv
        expected_writer.write_u32::<LittleEndian>(1)?;
        // ioffset
        expected_writer.write_u64::<LittleEndian>(337)?;
        expected_writer.flush()?;

        let mut reader = bgzf::Reader::new(actual_writer.get_ref().as_slice());
        let mut actual = Vec::new();
        reader.read_to_end(&mut actual)?;

        let expected = expected_writer.get_ref();

        assert_eq!(&actual, expected);

        Ok(())
    }
}
