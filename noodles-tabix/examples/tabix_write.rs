//! Builds and writes a tabix index file from sample BED data.
//!
//! This writes the output to stdout.

use std::io::{self, BufRead, Write};

use noodles_bgzf::{self as bgzf, index::Chunk};
use noodles_tabix as tabix;

const SEPARATOR: char = '\t';

static BED_DATA: &[u8] = b"\
sq0\t8\t13
sq0\t121393\t196418
";

fn compress_data(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut writer = bgzf::Writer::new(Vec::new());
    writer.write_all(&data)?;
    writer.finish()
}

fn parse_record(s: &str) -> io::Result<(&str, i32, i32)> {
    let mut components = s.splitn(3, SEPARATOR);

    let reference_sequence_name = components
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;

    let start = components
        .next()
        .and_then(|t| t.parse().ok())
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;

    let end = components
        .next()
        .and_then(|t| t.parse().ok())
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;

    Ok((reference_sequence_name, start, end))
}

fn main() -> io::Result<()> {
    let data = compress_data(BED_DATA)?;
    let mut reader = bgzf::Reader::new(&data[..]);

    let mut indexer = tabix::Index::indexer();
    indexer.set_header(tabix::index::header::Builder::bed().build());

    let mut buf = String::new();
    let mut start_position = reader.virtual_position();

    loop {
        buf.clear();

        match reader.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        let end_position = reader.virtual_position();
        let chunk = Chunk::new(start_position, end_position);

        let (reference_sequence_name, start, end) = parse_record(buf.trim_end())?;
        indexer.add_record(reference_sequence_name, start, end, chunk);

        start_position = end_position;
    }

    let index = indexer.build();

    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = tabix::Writer::new(handle);

    writer.write_index(&index)?;

    Ok(())
}
