//! VCF record info field key.

use crate::header::{info::Type, Number};

use std::{error, fmt, str::FromStr};

/// A VCF record info field key.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Key {
    // § 1.6.1 Fixed Fields (2021-01-13)
    /// Ancestral allele (`AA`).
    AncestralAllele,
    /// Allele count in genotypes, for each ALT allele, in the same order as listed (`AC`).
    AlleleCount,
    /// Total read depth for each allele (`AD`).
    TotalReadDepths,
    /// Read depth for each allele on the forward strand (`ADF`).
    ForwardStrandReadDepths,
    /// Read depth for each allele on the reverse strand (`ADR`).
    ReverseStrandReadDepths,
    /// Allele frequency for each ALT allele in the same order as listed (`AF`).
    AlleleFrequencies,
    /// Total number of alleles in called genotypes (`AN`).
    TotalAlleleCount,
    /// RMS base quality (`BQ`).
    BaseQuality,
    /// Cigar string describing how to align an alternate allele to the reference allele (`CIGAR`).
    Cigar,
    /// dbSNP membership (`DB`).
    IsInDbSnp,
    /// Combined depth across samples (`DP`).
    TotalDepth,
    // /// End position on CHROM (`END`).
    // EndPosition,
    /// HapMap2 membership (`H2`).
    IsInHapMap2,
    /// HapMap3 membership (`H3`).
    IsInHapMap3,
    /// RMS mapping quality (`MQ`).
    MappingQuality,
    /// Number of MAPQ == 0 reads (`MQ0`).
    ZeroMappingQualityCount,
    /// Number of samples with data (`NS`).
    SamplesWithDataCount,
    /// Strand bias (`SB`).
    StrandBias,
    /// Somatic mutation (`SOMATIC`).
    IsSomaticMutation,
    /// Validated by follow-up experiment (`VALIDATED`).
    IsValidated,
    /// 1000 Genomes membership (`1000G`).
    IsIn1000Genomes,

    // § 3 INFO keys used for structural variants (2021-01-13)
    /// Imprecise structural variation (`IMPRECISE`).
    IsImprecise,
    /// Indicates a novel structural variation (`NOVEL`).
    IsNovel,
    /// End position of the variant described in this record (`END`).
    EndPosition,
    /// Type of structural variant (`SVTYPE`).
    SvType,
    /// Difference in length between REF and ALT alleles (`SVLEN`).
    SvLengths,
    /// Confidence interval around POS for imprecise variants (`CIPOS`).
    PositionConfidenceIntervals,
    /// Confidence interval around END for imprecise variants (`CIEND`).
    EndConfidenceIntervals,
    /// Length of base pair identical micro-homology at event breakpoints (`HOMLEN`).
    MicrohomologyLengths,
    /// Sequence of base pair identical micro-homology at event breakpoints (`HOMSEQ`).
    MicrohomologySequences,
    /// ID of the assembled alternate allele in the assembly file (`BKPTID`).
    BreakpointIds,
    /// Mobile element info of the form NAME,START,END,POLARITY (`MEINFO`).
    MobileElementInfo,
    /// Mobile element transduction info of the form CHR,START,END,POLARITY (`METRANS`).
    MobileElementTransductionInfo,
    /// ID of this element in Database of Genomic Variation (`DBVID`).
    DbvId,
    /// ID of this element in DBVAR (`DBVARID`).
    DbVarId,
    /// ID of this element in DBRIP (`DBRIPID`).
    DbRipId,
    /// ID of mate breakends (`MATEID`).
    MateBreakendIds,
    /// ID of partner breakend (`PARID`).
    PartnerBreakendId,
    /// ID of event associated to breakend (`EVENT`).
    BreakendEventId,
    /// Confidence interval around the inserted material between breakends (`CILEN`).
    BreakendConfidenceIntervals,
    // /// Read Depth of segment containing breakend (`DP`).
    // BreakendReadDepth,
    /// Read Depth of adjacency (`DPADJ`).
    AdjacentReadDepths,
    /// Copy number of segment containing breakend (`CN`).
    BreakendCopyNumber,
    /// Copy number of adjacency (`CNADJ`).
    AdjacentCopyNumber,
    /// Confidence interval around copy number for the segment (`CICN`).
    CopyNumberConfidenceIntervals,
    /// Confidence interval around copy number for the adjacency (`CICNADJ`).
    AdjacentCopyNumberConfidenceIntervals,

    /// Any other non-reserved key.
    Other(String, Number, Type, String),
}

impl Key {
    /// Returns the cardinality of the info field value.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{header::Number, record::info::field::Key};
    /// assert_eq!(Key::AlleleCount.number(), Number::A);
    /// ```
    pub fn number(&self) -> Number {
        match self {
            Self::AncestralAllele => Number::Count(1),
            Self::AlleleCount => Number::A,
            Self::TotalReadDepths => Number::R,
            Self::ForwardStrandReadDepths => Number::R,
            Self::ReverseStrandReadDepths => Number::R,
            Self::AlleleFrequencies => Number::A,
            Self::TotalAlleleCount => Number::Count(1),
            Self::BaseQuality => Number::Count(1),
            Self::Cigar => Number::A,
            Self::IsInDbSnp => Number::Count(0),
            Self::TotalDepth => Number::Count(1),
            // Self::EndPosition => Number::Count(1),
            Self::IsInHapMap2 => Number::Count(0),
            Self::IsInHapMap3 => Number::Count(0),
            Self::MappingQuality => Number::Count(1),
            Self::ZeroMappingQualityCount => Number::Count(1),
            Self::SamplesWithDataCount => Number::Count(1),
            Self::StrandBias => Number::Count(4),
            Self::IsSomaticMutation => Number::Count(0),
            Self::IsValidated => Number::Count(0),
            Self::IsIn1000Genomes => Number::Count(0),

            Self::IsImprecise => Number::Count(0),
            Self::IsNovel => Number::Count(0),
            Self::EndPosition => Number::Count(1),
            Self::SvType => Number::Count(1),
            Self::SvLengths => Number::Unknown,
            Self::PositionConfidenceIntervals => Number::Count(2),
            Self::EndConfidenceIntervals => Number::Count(2),
            Self::MicrohomologyLengths => Number::Unknown,
            Self::MicrohomologySequences => Number::Unknown,
            Self::BreakpointIds => Number::Unknown,
            Self::MobileElementInfo => Number::Count(4),
            Self::MobileElementTransductionInfo => Number::Count(4),
            Self::DbvId => Number::Count(1),
            Self::DbVarId => Number::Count(1),
            Self::DbRipId => Number::Count(1),
            Self::MateBreakendIds => Number::Unknown,
            Self::PartnerBreakendId => Number::Count(1),
            Self::BreakendEventId => Number::Count(1),
            Self::BreakendConfidenceIntervals => Number::Count(2),
            // Self::BreakendReadDepth => Number::Count(1),
            Self::AdjacentReadDepths => Number::Unknown,
            Self::BreakendCopyNumber => Number::Count(1),
            Self::AdjacentCopyNumber => Number::Unknown,
            Self::CopyNumberConfidenceIntervals => Number::Count(2),
            Self::AdjacentCopyNumberConfidenceIntervals => Number::Unknown,

            Self::Other(_, number, _, _) => *number,
        }
    }

    /// Returns the type of the info field value.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{header::info::Type, record::info::field::Key};
    /// assert_eq!(Key::AlleleCount.ty(), Type::Integer);
    /// ```
    pub fn ty(&self) -> Type {
        match self {
            Self::AncestralAllele => Type::String,
            Self::AlleleCount => Type::Integer,
            Self::TotalReadDepths => Type::Integer,
            Self::ForwardStrandReadDepths => Type::Integer,
            Self::ReverseStrandReadDepths => Type::Integer,
            Self::AlleleFrequencies => Type::Float,
            Self::TotalAlleleCount => Type::Integer,
            Self::BaseQuality => Type::Float,
            Self::Cigar => Type::String,
            Self::IsInDbSnp => Type::Flag,
            Self::TotalDepth => Type::Integer,
            // Self::EndPosition => Type::Integer,
            Self::IsInHapMap2 => Type::Flag,
            Self::IsInHapMap3 => Type::Flag,
            Self::MappingQuality => Type::Float,
            Self::ZeroMappingQualityCount => Type::Integer,
            Self::SamplesWithDataCount => Type::Integer,
            Self::StrandBias => Type::Integer,
            Self::IsSomaticMutation => Type::Flag,
            Self::IsValidated => Type::Flag,
            Self::IsIn1000Genomes => Type::Flag,

            Self::IsImprecise => Type::Flag,
            Self::IsNovel => Type::Flag,
            Self::EndPosition => Type::Integer,
            Self::SvType => Type::String,
            Self::SvLengths => Type::Integer,
            Self::PositionConfidenceIntervals => Type::Integer,
            Self::EndConfidenceIntervals => Type::Integer,
            Self::MicrohomologyLengths => Type::Integer,
            Self::MicrohomologySequences => Type::String,
            Self::BreakpointIds => Type::String,
            Self::MobileElementInfo => Type::String,
            Self::MobileElementTransductionInfo => Type::String,
            Self::DbvId => Type::String,
            Self::DbVarId => Type::String,
            Self::DbRipId => Type::String,
            Self::MateBreakendIds => Type::String,
            Self::PartnerBreakendId => Type::String,
            Self::BreakendEventId => Type::String,
            Self::BreakendConfidenceIntervals => Type::Integer,
            // Self::BreakendReadDepth => Type::Integer,
            Self::AdjacentReadDepths => Type::Integer,
            Self::BreakendCopyNumber => Type::Integer,
            Self::AdjacentCopyNumber => Type::Integer,
            Self::CopyNumberConfidenceIntervals => Type::Integer,
            Self::AdjacentCopyNumberConfidenceIntervals => Type::Integer,

            Self::Other(_, _, ty, _) => *ty,
        }
    }

    /// Returns the description of the info field.
    ///
    /// # Examples
    ///
    /// ```
    /// use noodles_vcf::{header::Number, record::info::field::Key};
    ///
    /// assert_eq!(
    ///     Key::AlleleCount.description(),
    ///     "Allele count in genotypes, for each ALT allele, in the same order as listed",
    /// );
    /// ```
    pub fn description(&self) -> &str {
        match self {
            Self::AncestralAllele => "Ancestral allele",
            Self::AlleleCount => {
                "Allele count in genotypes, for each ALT allele, in the same order as listed"
            }
            Self::TotalReadDepths => "Total read depth for each allele",
            Self::ForwardStrandReadDepths => "Read depth for each allele on the forward strand",
            Self::ReverseStrandReadDepths => "Read depth for each allele on the reverse strand",
            Self::AlleleFrequencies => {
                "Allele frequency for each ALT allele in the same order as listed"
            }
            Self::TotalAlleleCount => "Total number of alleles in called genotypes",
            Self::BaseQuality => "RMS base quality",
            Self::Cigar => {
                "Cigar string describing how to align an alternate allele to the reference allele"
            }
            Self::IsInDbSnp => "dbSNP membership",
            Self::TotalDepth => "Combined depth across samples",
            // Self::EndPosition => "End position on CHROM",
            Self::IsInHapMap2 => "HapMap2 membership",
            Self::IsInHapMap3 => "HapMap3 membership",
            Self::MappingQuality => "RMS mapping quality",
            Self::ZeroMappingQualityCount => "Number of MAPQ == 0 reads",
            Self::SamplesWithDataCount => "Number of samples with data",
            Self::StrandBias => "Strand bias",
            Self::IsSomaticMutation => "Somatic mutation",
            Self::IsValidated => "Validated by follow-up experiment",
            Self::IsIn1000Genomes => "1000 Genomes membership",

            Self::IsImprecise => "Imprecise structural variation",
            Self::IsNovel => "Indicates a novel structural variation",
            Self::EndPosition => "End position of the variant described in this record",
            Self::SvType => "Type of structural variant",
            Self::SvLengths => "Difference in length between REF and ALT alleles",
            Self::PositionConfidenceIntervals => {
                "Confidence interval around POS for imprecise variants"
            }
            Self::EndConfidenceIntervals => "Confidence interval around END for imprecise variants",
            Self::MicrohomologyLengths => {
                "Length of base pair identical micro-homology at event breakpoints"
            }
            Self::MicrohomologySequences => {
                "Sequence of base pair identical micro-homology at event breakpoints"
            }
            Self::BreakpointIds => "ID of the assembled alternate allele in the assembly file",
            Self::MobileElementInfo => "Mobile element info of the form NAME,START,END,POLARITY",
            Self::MobileElementTransductionInfo => {
                "Mobile element transduction info of the form CHR,START,END,POLARITY"
            }
            Self::DbvId => "ID of this element in Database of Genomic Variation",
            Self::DbVarId => "ID of this element in DBVAR",
            Self::DbRipId => "ID of this element in DBRIP",
            Self::MateBreakendIds => "ID of mate breakends",
            Self::PartnerBreakendId => "ID of partner breakend",
            Self::BreakendEventId => "ID of event associated to breakend",
            Self::BreakendConfidenceIntervals => {
                "Confidence interval around the inserted material between breakends"
            }
            // Self::BreakendReadDepth => "Read Depth of segment containing breakend",
            Self::AdjacentReadDepths => "Read Depth of adjacency",
            Self::BreakendCopyNumber => "Copy number of segment containing breakend",
            Self::AdjacentCopyNumber => "Copy number of adjacency",
            Self::CopyNumberConfidenceIntervals => {
                "Confidence interval around copy number for the segment"
            }
            Self::AdjacentCopyNumberConfidenceIntervals => {
                "Confidence interval around copy number for the adjacency"
            }

            Self::Other(_, _, _, description) => description,
        }
    }
}

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        match self {
            Self::AncestralAllele => "AA",
            Self::AlleleCount => "AC",
            Self::TotalReadDepths => "AD",
            Self::ForwardStrandReadDepths => "ADF",
            Self::ReverseStrandReadDepths => "ADR",
            Self::AlleleFrequencies => "AF",
            Self::TotalAlleleCount => "AN",
            Self::BaseQuality => "BQ",
            Self::Cigar => "CIGAR",
            Self::IsInDbSnp => "DB",
            Self::TotalDepth => "DP",
            // Self::EndPosition => "END",
            Self::IsInHapMap2 => "H2",
            Self::IsInHapMap3 => "H3",
            Self::MappingQuality => "MQ",
            Self::ZeroMappingQualityCount => "MQ0",
            Self::SamplesWithDataCount => "NS",
            Self::StrandBias => "SB",
            Self::IsSomaticMutation => "SOMATIC",
            Self::IsValidated => "VALIDATED",
            Self::IsIn1000Genomes => "1000G",

            Self::IsImprecise => "IMPRECISE",
            Self::IsNovel => "NOVEL",
            Self::EndPosition => "END",
            Self::SvType => "SVTYPE",
            Self::SvLengths => "SVLEN",
            Self::PositionConfidenceIntervals => "CIPOS",
            Self::EndConfidenceIntervals => "CIEND",
            Self::MicrohomologyLengths => "HOMLEN",
            Self::MicrohomologySequences => "HOMSEQ",
            Self::BreakpointIds => "BKPTID",
            Self::MobileElementInfo => "MEINFO",
            Self::MobileElementTransductionInfo => "METRANS",
            Self::DbvId => "DGVID",
            Self::DbVarId => "DBVARID",
            Self::DbRipId => "DBRIPID",
            Self::MateBreakendIds => "MATEID",
            Self::PartnerBreakendId => "PARID",
            Self::BreakendEventId => "EVENT",
            Self::BreakendConfidenceIntervals => "CILEN",
            // Self::BreakendReadDepth => "DP",
            Self::AdjacentReadDepths => "DPADJ",
            Self::BreakendCopyNumber => "CN",
            Self::AdjacentCopyNumber => "CNADJ",
            Self::CopyNumberConfidenceIntervals => "CICN",
            Self::AdjacentCopyNumberConfidenceIntervals => "CICNADJ",

            Self::Other(key, ..) => key,
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

/// An error returned when a raw VCF record info field key fails to parse.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// The input is empty.
    Empty,
    /// The input is invalid.
    Invalid,
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("empty input"),
            Self::Invalid => f.write_str("invalid input"),
        }
    }
}

impl FromStr for Key {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseError::Empty);
        }

        match s {
            "AA" => Ok(Self::AncestralAllele),
            "AC" => Ok(Self::AlleleCount),
            "AD" => Ok(Self::TotalReadDepths),
            "ADF" => Ok(Self::ForwardStrandReadDepths),
            "ADR" => Ok(Self::ReverseStrandReadDepths),
            "AF" => Ok(Self::AlleleFrequencies),
            "AN" => Ok(Self::TotalAlleleCount),
            "BQ" => Ok(Self::BaseQuality),
            "CIGAR" => Ok(Self::Cigar),
            "DB" => Ok(Self::IsInDbSnp),
            "DP" => Ok(Self::TotalDepth),
            // "END" => Ok(Self::EndPosition),
            "H2" => Ok(Self::IsInHapMap2),
            "H3" => Ok(Self::IsInHapMap3),
            "MQ" => Ok(Self::MappingQuality),
            "MQ0" => Ok(Self::ZeroMappingQualityCount),
            "NS" => Ok(Self::SamplesWithDataCount),
            "SB" => Ok(Self::StrandBias),
            "SOMATIC" => Ok(Self::IsSomaticMutation),
            "VALIDATED" => Ok(Self::IsValidated),
            "1000G" => Ok(Self::IsIn1000Genomes),

            "IMPRECISE" => Ok(Self::IsImprecise),
            "NOVEL" => Ok(Self::IsNovel),
            "END" => Ok(Self::EndPosition),
            "SVTYPE" => Ok(Self::SvType),
            "SVLEN" => Ok(Self::SvLengths),
            "CIPOS" => Ok(Self::PositionConfidenceIntervals),
            "CIEND" => Ok(Self::EndConfidenceIntervals),
            "HOMLEN" => Ok(Self::MicrohomologyLengths),
            "HOMSEQ" => Ok(Self::MicrohomologySequences),
            "BKPTID" => Ok(Self::BreakpointIds),
            "MEINFO" => Ok(Self::MobileElementInfo),
            "METRANS" => Ok(Self::MobileElementTransductionInfo),
            "DGVID" => Ok(Self::DbvId),
            "DBVARID" => Ok(Self::DbVarId),
            "DBRIPID" => Ok(Self::DbRipId),
            "MATEID" => Ok(Self::MateBreakendIds),
            "PARID" => Ok(Self::PartnerBreakendId),
            "EVENT" => Ok(Self::BreakendEventId),
            "CILEN" => Ok(Self::BreakendConfidenceIntervals),
            // "DP" => Ok(Self::BreakendReadDepth),
            "DPADJ" => Ok(Self::AdjacentReadDepths),
            "CN" => Ok(Self::BreakendCopyNumber),
            "CNADJ" => Ok(Self::AdjacentCopyNumber),
            "CICN" => Ok(Self::CopyNumberConfidenceIntervals),
            "CICNADJ" => Ok(Self::AdjacentCopyNumberConfidenceIntervals),

            _ => {
                if is_valid_name(s) {
                    Ok(Self::Other(
                        s.into(),
                        Number::Count(1),
                        Type::String,
                        String::default(),
                    ))
                } else {
                    Err(ParseError::Invalid)
                }
            }
        }
    }
}

// § 1.6.1 Fixed fields
fn is_valid_name_char(c: char) -> bool {
    matches!(c, '0'..='9' | 'A'..='Z' | 'a'..='z' | '_' | '.')
}

fn is_valid_name(s: &str) -> bool {
    let mut chars = s.chars();

    if let Some(c) = chars.next() {
        if !matches!(c, 'A'..='Z' | 'a'..='z' | '_') {
            return false;
        }
    }

    chars.all(is_valid_name_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        assert_eq!(Key::AncestralAllele.number(), Number::Count(1));
        assert_eq!(Key::AlleleCount.number(), Number::A);
        assert_eq!(Key::TotalReadDepths.number(), Number::R);
        assert_eq!(Key::ForwardStrandReadDepths.number(), Number::R);
        assert_eq!(Key::ReverseStrandReadDepths.number(), Number::R);
        assert_eq!(Key::AlleleFrequencies.number(), Number::A);
        assert_eq!(Key::TotalAlleleCount.number(), Number::Count(1));
        assert_eq!(Key::BaseQuality.number(), Number::Count(1));
        assert_eq!(Key::Cigar.number(), Number::A);
        assert_eq!(Key::IsInDbSnp.number(), Number::Count(0));
        assert_eq!(Key::TotalDepth.number(), Number::Count(1));
        // assert_eq!(Key::EndPosition.number(), Number::Count(1));
        assert_eq!(Key::IsInHapMap2.number(), Number::Count(0));
        assert_eq!(Key::IsInHapMap3.number(), Number::Count(0));
        assert_eq!(Key::MappingQuality.number(), Number::Count(1));
        assert_eq!(Key::ZeroMappingQualityCount.number(), Number::Count(1));
        assert_eq!(Key::SamplesWithDataCount.number(), Number::Count(1));
        assert_eq!(Key::StrandBias.number(), Number::Count(4));
        assert_eq!(Key::IsSomaticMutation.number(), Number::Count(0));
        assert_eq!(Key::IsValidated.number(), Number::Count(0));
        assert_eq!(Key::IsIn1000Genomes.number(), Number::Count(0));

        assert_eq!(Key::IsImprecise.number(), Number::Count(0));
        assert_eq!(Key::IsNovel.number(), Number::Count(0));
        assert_eq!(Key::EndPosition.number(), Number::Count(1));
        assert_eq!(Key::SvType.number(), Number::Count(1));
        assert_eq!(Key::SvLengths.number(), Number::Unknown);
        assert_eq!(Key::PositionConfidenceIntervals.number(), Number::Count(2));
        assert_eq!(Key::EndConfidenceIntervals.number(), Number::Count(2));
        assert_eq!(Key::MicrohomologyLengths.number(), Number::Unknown);
        assert_eq!(Key::MicrohomologySequences.number(), Number::Unknown);
        assert_eq!(Key::BreakpointIds.number(), Number::Unknown);
        assert_eq!(Key::MobileElementInfo.number(), Number::Count(4));
        assert_eq!(
            Key::MobileElementTransductionInfo.number(),
            Number::Count(4)
        );
        assert_eq!(Key::DbvId.number(), Number::Count(1));
        assert_eq!(Key::DbVarId.number(), Number::Count(1));
        assert_eq!(Key::DbRipId.number(), Number::Count(1));
        assert_eq!(Key::MateBreakendIds.number(), Number::Unknown);
        assert_eq!(Key::PartnerBreakendId.number(), Number::Count(1));
        assert_eq!(Key::BreakendEventId.number(), Number::Count(1));
        assert_eq!(Key::BreakendConfidenceIntervals.number(), Number::Count(2));
        // assert_eq!(Key::BreakendReadDepth.number(), Number::Count(1));
        assert_eq!(Key::AdjacentReadDepths.number(), Number::Unknown);
        assert_eq!(Key::BreakendCopyNumber.number(), Number::Count(1));
        assert_eq!(Key::AdjacentCopyNumber.number(), Number::Unknown);
        assert_eq!(
            Key::CopyNumberConfidenceIntervals.number(),
            Number::Count(2)
        );
        assert_eq!(
            Key::AdjacentCopyNumberConfidenceIntervals.number(),
            Number::Unknown
        );

        assert_eq!(
            Key::Other(
                String::from("NDLS"),
                Number::Count(1),
                Type::String,
                String::default()
            )
            .number(),
            Number::Count(1)
        );
    }

    #[test]
    fn test_ty() {
        assert_eq!(Key::AncestralAllele.ty(), Type::String);
        assert_eq!(Key::AlleleCount.ty(), Type::Integer);
        assert_eq!(Key::TotalReadDepths.ty(), Type::Integer);
        assert_eq!(Key::ForwardStrandReadDepths.ty(), Type::Integer);
        assert_eq!(Key::ReverseStrandReadDepths.ty(), Type::Integer);
        assert_eq!(Key::AlleleFrequencies.ty(), Type::Float);
        assert_eq!(Key::TotalAlleleCount.ty(), Type::Integer);
        assert_eq!(Key::BaseQuality.ty(), Type::Float);
        assert_eq!(Key::Cigar.ty(), Type::String);
        assert_eq!(Key::IsInDbSnp.ty(), Type::Flag);
        assert_eq!(Key::TotalDepth.ty(), Type::Integer);
        // assert_eq!(Key::EndPosition.ty(), Type::Integer);
        assert_eq!(Key::IsInHapMap2.ty(), Type::Flag);
        assert_eq!(Key::IsInHapMap3.ty(), Type::Flag);
        assert_eq!(Key::MappingQuality.ty(), Type::Float);
        assert_eq!(Key::ZeroMappingQualityCount.ty(), Type::Integer);
        assert_eq!(Key::SamplesWithDataCount.ty(), Type::Integer);
        assert_eq!(Key::StrandBias.ty(), Type::Integer);
        assert_eq!(Key::IsSomaticMutation.ty(), Type::Flag);
        assert_eq!(Key::IsValidated.ty(), Type::Flag);
        assert_eq!(Key::IsIn1000Genomes.ty(), Type::Flag);

        assert_eq!(Key::IsImprecise.ty(), Type::Flag);
        assert_eq!(Key::IsNovel.ty(), Type::Flag);
        assert_eq!(Key::EndPosition.ty(), Type::Integer);
        assert_eq!(Key::SvType.ty(), Type::String);
        assert_eq!(Key::SvLengths.ty(), Type::Integer);
        assert_eq!(Key::PositionConfidenceIntervals.ty(), Type::Integer);
        assert_eq!(Key::EndConfidenceIntervals.ty(), Type::Integer);
        assert_eq!(Key::MicrohomologyLengths.ty(), Type::Integer);
        assert_eq!(Key::MicrohomologySequences.ty(), Type::String);
        assert_eq!(Key::BreakpointIds.ty(), Type::String);
        assert_eq!(Key::MobileElementInfo.ty(), Type::String);
        assert_eq!(Key::MobileElementTransductionInfo.ty(), Type::String);
        assert_eq!(Key::DbvId.ty(), Type::String);
        assert_eq!(Key::DbVarId.ty(), Type::String);
        assert_eq!(Key::DbRipId.ty(), Type::String);
        assert_eq!(Key::MateBreakendIds.ty(), Type::String);
        assert_eq!(Key::PartnerBreakendId.ty(), Type::String);
        assert_eq!(Key::BreakendEventId.ty(), Type::String);
        assert_eq!(Key::BreakendConfidenceIntervals.ty(), Type::Integer);
        // assert_eq!(Key::BreakendReadDepth.ty(), Type::Integer);
        assert_eq!(Key::AdjacentReadDepths.ty(), Type::Integer);
        assert_eq!(Key::BreakendCopyNumber.ty(), Type::Integer);
        assert_eq!(Key::AdjacentCopyNumber.ty(), Type::Integer);
        assert_eq!(Key::CopyNumberConfidenceIntervals.ty(), Type::Integer);
        assert_eq!(
            Key::AdjacentCopyNumberConfidenceIntervals.ty(),
            Type::Integer
        );

        assert_eq!(
            Key::Other(
                String::from("NDLS"),
                Number::Count(1),
                Type::String,
                String::default()
            )
            .ty(),
            Type::String
        );
    }

    #[test]
    fn test_description() {
        assert_eq!(Key::AncestralAllele.description(), "Ancestral allele");
        assert_eq!(
            Key::AlleleCount.description(),
            "Allele count in genotypes, for each ALT allele, in the same order as listed"
        );
        assert_eq!(
            Key::TotalReadDepths.description(),
            "Total read depth for each allele"
        );
        assert_eq!(
            Key::ForwardStrandReadDepths.description(),
            "Read depth for each allele on the forward strand"
        );
        assert_eq!(
            Key::ReverseStrandReadDepths.description(),
            "Read depth for each allele on the reverse strand"
        );
        assert_eq!(
            Key::AlleleFrequencies.description(),
            "Allele frequency for each ALT allele in the same order as listed"
        );
        assert_eq!(
            Key::TotalAlleleCount.description(),
            "Total number of alleles in called genotypes"
        );
        assert_eq!(Key::BaseQuality.description(), "RMS base quality");
        assert_eq!(
            Key::Cigar.description(),
            "Cigar string describing how to align an alternate allele to the reference allele"
        );
        assert_eq!(Key::IsInDbSnp.description(), "dbSNP membership");
        assert_eq!(
            Key::TotalDepth.description(),
            "Combined depth across samples"
        );
        // Self::EndPosition.description(), "End position on CHROM");
        assert_eq!(Key::IsInHapMap2.description(), "HapMap2 membership");
        assert_eq!(Key::IsInHapMap3.description(), "HapMap3 membership");
        assert_eq!(Key::MappingQuality.description(), "RMS mapping quality");
        assert_eq!(
            Key::ZeroMappingQualityCount.description(),
            "Number of MAPQ == 0 reads"
        );
        assert_eq!(
            Key::SamplesWithDataCount.description(),
            "Number of samples with data"
        );
        assert_eq!(Key::StrandBias.description(), "Strand bias");
        assert_eq!(Key::IsSomaticMutation.description(), "Somatic mutation");
        assert_eq!(
            Key::IsValidated.description(),
            "Validated by follow-up experiment"
        );
        assert_eq!(
            Key::IsIn1000Genomes.description(),
            "1000 Genomes membership"
        );

        assert_eq!(
            Key::IsImprecise.description(),
            "Imprecise structural variation"
        );
        assert_eq!(
            Key::IsNovel.description(),
            "Indicates a novel structural variation"
        );
        assert_eq!(
            Key::EndPosition.description(),
            "End position of the variant described in this record"
        );
        assert_eq!(Key::SvType.description(), "Type of structural variant");
        assert_eq!(
            Key::SvLengths.description(),
            "Difference in length between REF and ALT alleles"
        );
        assert_eq!(
            Key::PositionConfidenceIntervals.description(),
            "Confidence interval around POS for imprecise variants"
        );
        assert_eq!(
            Key::EndConfidenceIntervals.description(),
            "Confidence interval around END for imprecise variants"
        );
        assert_eq!(
            Key::MicrohomologyLengths.description(),
            "Length of base pair identical micro-homology at event breakpoints"
        );
        assert_eq!(
            Key::MicrohomologySequences.description(),
            "Sequence of base pair identical micro-homology at event breakpoints"
        );
        assert_eq!(
            Key::BreakpointIds.description(),
            "ID of the assembled alternate allele in the assembly file"
        );
        assert_eq!(
            Key::MobileElementInfo.description(),
            "Mobile element info of the form NAME,START,END,POLARITY"
        );
        assert_eq!(
            Key::MobileElementTransductionInfo.description(),
            "Mobile element transduction info of the form CHR,START,END,POLARITY"
        );
        assert_eq!(
            Key::DbvId.description(),
            "ID of this element in Database of Genomic Variation"
        );
        assert_eq!(Key::DbVarId.description(), "ID of this element in DBVAR");
        assert_eq!(Key::DbRipId.description(), "ID of this element in DBRIP");
        assert_eq!(Key::MateBreakendIds.description(), "ID of mate breakends");
        assert_eq!(
            Key::PartnerBreakendId.description(),
            "ID of partner breakend"
        );
        assert_eq!(
            Key::BreakendEventId.description(),
            "ID of event associated to breakend"
        );
        assert_eq!(
            Key::BreakendConfidenceIntervals.description(),
            "Confidence interval around the inserted material between breakends"
        );
        // Self::BreakendReadDepth.description(), "Read Depth of segment containing breakend");
        assert_eq!(
            Key::AdjacentReadDepths.description(),
            "Read Depth of adjacency"
        );
        assert_eq!(
            Key::BreakendCopyNumber.description(),
            "Copy number of segment containing breakend"
        );
        assert_eq!(
            Key::AdjacentCopyNumber.description(),
            "Copy number of adjacency"
        );
        assert_eq!(
            Key::CopyNumberConfidenceIntervals.description(),
            "Confidence interval around copy number for the segment"
        );
        assert_eq!(
            Key::AdjacentCopyNumberConfidenceIntervals.description(),
            "Confidence interval around copy number for the adjacency"
        );

        assert_eq!(
            Key::Other(
                String::from("NDLS"),
                Number::Count(1),
                Type::String,
                String::from("noodles"),
            )
            .description(),
            "noodles"
        );
    }

    #[test]
    fn test_fmt() {
        assert_eq!(Key::AncestralAllele.to_string(), "AA");
        assert_eq!(Key::AlleleCount.to_string(), "AC");
        assert_eq!(Key::TotalReadDepths.to_string(), "AD");
        assert_eq!(Key::ForwardStrandReadDepths.to_string(), "ADF");
        assert_eq!(Key::ReverseStrandReadDepths.to_string(), "ADR");
        assert_eq!(Key::AlleleFrequencies.to_string(), "AF");
        assert_eq!(Key::TotalAlleleCount.to_string(), "AN");
        assert_eq!(Key::BaseQuality.to_string(), "BQ");
        assert_eq!(Key::Cigar.to_string(), "CIGAR");
        assert_eq!(Key::IsInDbSnp.to_string(), "DB");
        assert_eq!(Key::TotalDepth.to_string(), "DP");
        // assert_eq!(Key::EndPosition.to_string(), "END");
        assert_eq!(Key::IsInHapMap2.to_string(), "H2");
        assert_eq!(Key::IsInHapMap3.to_string(), "H3");
        assert_eq!(Key::MappingQuality.to_string(), "MQ");
        assert_eq!(Key::ZeroMappingQualityCount.to_string(), "MQ0");
        assert_eq!(Key::SamplesWithDataCount.to_string(), "NS");
        assert_eq!(Key::StrandBias.to_string(), "SB");
        assert_eq!(Key::IsSomaticMutation.to_string(), "SOMATIC");
        assert_eq!(Key::IsValidated.to_string(), "VALIDATED");
        assert_eq!(Key::IsIn1000Genomes.to_string(), "1000G");

        assert_eq!(Key::IsImprecise.to_string(), "IMPRECISE");
        assert_eq!(Key::IsNovel.to_string(), "NOVEL");
        assert_eq!(Key::EndPosition.to_string(), "END");
        assert_eq!(Key::SvType.to_string(), "SVTYPE");
        assert_eq!(Key::SvLengths.to_string(), "SVLEN");
        assert_eq!(Key::PositionConfidenceIntervals.to_string(), "CIPOS");
        assert_eq!(Key::EndConfidenceIntervals.to_string(), "CIEND");
        assert_eq!(Key::MicrohomologyLengths.to_string(), "HOMLEN");
        assert_eq!(Key::MicrohomologySequences.to_string(), "HOMSEQ");
        assert_eq!(Key::BreakpointIds.to_string(), "BKPTID");
        assert_eq!(Key::MobileElementInfo.to_string(), "MEINFO");
        assert_eq!(Key::MobileElementTransductionInfo.to_string(), "METRANS");
        assert_eq!(Key::DbvId.to_string(), "DGVID");
        assert_eq!(Key::DbVarId.to_string(), "DBVARID");
        assert_eq!(Key::DbRipId.to_string(), "DBRIPID");
        assert_eq!(Key::MateBreakendIds.to_string(), "MATEID");
        assert_eq!(Key::PartnerBreakendId.to_string(), "PARID");
        assert_eq!(Key::BreakendEventId.to_string(), "EVENT");
        assert_eq!(Key::BreakendConfidenceIntervals.to_string(), "CILEN");
        // assert_eq!(Key::BreakendReadDepth.to_string(), "DP");
        assert_eq!(Key::AdjacentReadDepths.to_string(), "DPADJ");
        assert_eq!(Key::BreakendCopyNumber.to_string(), "CN");
        assert_eq!(Key::AdjacentCopyNumber.to_string(), "CNADJ");
        assert_eq!(Key::CopyNumberConfidenceIntervals.to_string(), "CICN");
        assert_eq!(
            Key::AdjacentCopyNumberConfidenceIntervals.to_string(),
            "CICNADJ"
        );

        assert_eq!(
            Key::Other(
                String::from("NDLS"),
                Number::Count(1),
                Type::String,
                String::default()
            )
            .to_string(),
            "NDLS"
        );
    }

    #[test]
    fn test_from_str() {
        assert_eq!("AA".parse(), Ok(Key::AncestralAllele));
        assert_eq!("AC".parse(), Ok(Key::AlleleCount));
        assert_eq!("AD".parse(), Ok(Key::TotalReadDepths));
        assert_eq!("ADF".parse(), Ok(Key::ForwardStrandReadDepths));
        assert_eq!("ADR".parse(), Ok(Key::ReverseStrandReadDepths));
        assert_eq!("AF".parse(), Ok(Key::AlleleFrequencies));
        assert_eq!("AN".parse(), Ok(Key::TotalAlleleCount));
        assert_eq!("BQ".parse(), Ok(Key::BaseQuality));
        assert_eq!("CIGAR".parse(), Ok(Key::Cigar));
        assert_eq!("DB".parse(), Ok(Key::IsInDbSnp));
        assert_eq!("DP".parse(), Ok(Key::TotalDepth));
        // assert_eq!("END".parse(), Ok(Key::EndPosition));
        assert_eq!("H2".parse(), Ok(Key::IsInHapMap2));
        assert_eq!("H3".parse(), Ok(Key::IsInHapMap3));
        assert_eq!("MQ".parse(), Ok(Key::MappingQuality));
        assert_eq!("MQ0".parse(), Ok(Key::ZeroMappingQualityCount));
        assert_eq!("NS".parse(), Ok(Key::SamplesWithDataCount));
        assert_eq!("SB".parse(), Ok(Key::StrandBias));
        assert_eq!("SOMATIC".parse(), Ok(Key::IsSomaticMutation));
        assert_eq!("VALIDATED".parse(), Ok(Key::IsValidated));
        assert_eq!("1000G".parse(), Ok(Key::IsIn1000Genomes));

        assert_eq!("IMPRECISE".parse(), Ok(Key::IsImprecise));
        assert_eq!("NOVEL".parse(), Ok(Key::IsNovel));
        assert_eq!("END".parse(), Ok(Key::EndPosition));
        assert_eq!("SVTYPE".parse(), Ok(Key::SvType));
        assert_eq!("SVLEN".parse(), Ok(Key::SvLengths));
        assert_eq!("CIPOS".parse(), Ok(Key::PositionConfidenceIntervals));
        assert_eq!("CIEND".parse(), Ok(Key::EndConfidenceIntervals));
        assert_eq!("HOMLEN".parse(), Ok(Key::MicrohomologyLengths));
        assert_eq!("HOMSEQ".parse(), Ok(Key::MicrohomologySequences));
        assert_eq!("BKPTID".parse(), Ok(Key::BreakpointIds));
        assert_eq!("MEINFO".parse(), Ok(Key::MobileElementInfo));
        assert_eq!("METRANS".parse(), Ok(Key::MobileElementTransductionInfo));
        assert_eq!("DGVID".parse(), Ok(Key::DbvId));
        assert_eq!("DBVARID".parse(), Ok(Key::DbVarId));
        assert_eq!("DBRIPID".parse(), Ok(Key::DbRipId));
        assert_eq!("MATEID".parse(), Ok(Key::MateBreakendIds));
        assert_eq!("PARID".parse(), Ok(Key::PartnerBreakendId));
        assert_eq!("EVENT".parse(), Ok(Key::BreakendEventId));
        assert_eq!("CILEN".parse(), Ok(Key::BreakendConfidenceIntervals));
        // assert_eq!("DP".parse(), Ok(Key::BreakendReadDepth));
        assert_eq!("DPADJ".parse(), Ok(Key::AdjacentReadDepths));
        assert_eq!("CN".parse(), Ok(Key::BreakendCopyNumber));
        assert_eq!("CNADJ".parse(), Ok(Key::AdjacentCopyNumber));
        assert_eq!("CICN".parse(), Ok(Key::CopyNumberConfidenceIntervals));
        assert_eq!(
            "CICNADJ".parse(),
            Ok(Key::AdjacentCopyNumberConfidenceIntervals)
        );

        assert_eq!(
            "NDLS".parse(),
            Ok(Key::Other(
                String::from("NDLS"),
                Number::Count(1),
                Type::String,
                String::default(),
            ))
        );

        assert_eq!("".parse::<Key>(), Err(ParseError::Empty));
        assert_eq!("8D".parse::<Key>(), Err(ParseError::Invalid));
        assert_eq!(".N".parse::<Key>(), Err(ParseError::Invalid));
        assert_eq!("A!".parse::<Key>(), Err(ParseError::Invalid));
    }
}
