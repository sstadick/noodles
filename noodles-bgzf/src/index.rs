//! BGZF index structures.

mod chunk;

pub use self::chunk::Chunk;

use super::VirtualPosition;

/// Merges a list of chunks into a list of non-overlapping chunks.
///
/// This is the same as calling [`optimize_chunks`] with a `min_offset` of 0.
///
/// # Examples
///
/// ```
/// use noodles_bgzf::{self as bgzf, index::{merge_chunks, Chunk}};
///
/// let chunks = [
///     Chunk::new(bgzf::VirtualPosition::from(2), bgzf::VirtualPosition::from(3)),
///     Chunk::new(bgzf::VirtualPosition::from(5), bgzf::VirtualPosition::from(8)),
///     Chunk::new(bgzf::VirtualPosition::from(7), bgzf::VirtualPosition::from(13)),
///     Chunk::new(bgzf::VirtualPosition::from(21), bgzf::VirtualPosition::from(34)),
/// ];
///
/// let actual = merge_chunks(&chunks);
///
/// let expected = [
///     Chunk::new(bgzf::VirtualPosition::from(2), bgzf::VirtualPosition::from(3)),
///     Chunk::new(bgzf::VirtualPosition::from(5), bgzf::VirtualPosition::from(13)),
///     Chunk::new(bgzf::VirtualPosition::from(21), bgzf::VirtualPosition::from(34)),
/// ];
///
/// assert_eq!(actual, expected);
/// ```
pub fn merge_chunks(chunks: &[Chunk]) -> Vec<Chunk> {
    optimize_chunks(chunks, VirtualPosition::default())
}

/// Optimizes a list of chunks into a list of non-overlapping chunks.
///
/// Unlike [`merge_chunks`], `min_offset` (typically from the linear index) is given to remove
/// chunks that cannot be in the query.
///
/// # Examples
///
/// ```
/// use noodles_bgzf::{self as bgzf, index::{optimize_chunks, Chunk}};
///
/// let chunks = [
///     Chunk::new(bgzf::VirtualPosition::from(2), bgzf::VirtualPosition::from(3)),
///     Chunk::new(bgzf::VirtualPosition::from(5), bgzf::VirtualPosition::from(8)),
///     Chunk::new(bgzf::VirtualPosition::from(7), bgzf::VirtualPosition::from(13)),
///     Chunk::new(bgzf::VirtualPosition::from(21), bgzf::VirtualPosition::from(34)),
/// ];
/// let min_offset = bgzf::VirtualPosition::from(5);
///
/// let actual = optimize_chunks(&chunks, min_offset);
///
/// let expected = [
///     Chunk::new(bgzf::VirtualPosition::from(5), bgzf::VirtualPosition::from(13)),
///     Chunk::new(bgzf::VirtualPosition::from(21), bgzf::VirtualPosition::from(34)),
/// ];
///
/// assert_eq!(actual, expected);
/// ```
pub fn optimize_chunks(chunks: &[Chunk], min_offset: VirtualPosition) -> Vec<Chunk> {
    let mut chunks: Vec<_> = chunks
        .iter()
        .filter(|c| c.end() > min_offset)
        .copied()
        .collect();

    if chunks.is_empty() {
        return chunks;
    }

    chunks.sort_unstable_by_key(|c| c.start());

    // At worst, no chunks are merged, and the resulting list will be the same size as the input.
    let mut merged_chunks = Vec::with_capacity(chunks.len());

    // `chunks` is guaranteed to be non-empty.
    let mut current_chunk = chunks[0];

    for next_chunk in chunks.iter().skip(1) {
        if next_chunk.start() > current_chunk.end() {
            merged_chunks.push(current_chunk);
            current_chunk = *next_chunk;
        } else if current_chunk.end() < next_chunk.end() {
            current_chunk = Chunk::new(current_chunk.start(), next_chunk.end());
        }
    }

    merged_chunks.push(current_chunk);

    merged_chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_chunks() -> Vec<Chunk> {
        vec![
            Chunk::new(VirtualPosition::from(2), VirtualPosition::from(5)),
            Chunk::new(VirtualPosition::from(3), VirtualPosition::from(4)),
            Chunk::new(VirtualPosition::from(5), VirtualPosition::from(7)),
            Chunk::new(VirtualPosition::from(9), VirtualPosition::from(12)),
            Chunk::new(VirtualPosition::from(10), VirtualPosition::from(15)),
            Chunk::new(VirtualPosition::from(16), VirtualPosition::from(21)),
        ]
    }

    #[test]
    fn test_merge_chunks() {
        let chunks = build_chunks();
        let actual = merge_chunks(&chunks);

        let expected = [
            Chunk::new(VirtualPosition::from(2), VirtualPosition::from(7)),
            Chunk::new(VirtualPosition::from(9), VirtualPosition::from(15)),
            Chunk::new(VirtualPosition::from(16), VirtualPosition::from(21)),
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_merge_chunks_with_empty_list() {
        let chunks = Vec::new();
        let merged_chunks = merge_chunks(&chunks);
        assert!(merged_chunks.is_empty());
    }

    #[test]
    fn test_optimize_chunks() {
        let chunks = build_chunks();
        let actual = optimize_chunks(&chunks, VirtualPosition::from(10));

        let expected = [
            Chunk::new(VirtualPosition::from(9), VirtualPosition::from(15)),
            Chunk::new(VirtualPosition::from(16), VirtualPosition::from(21)),
        ];

        assert_eq!(actual, expected);
    }
}
