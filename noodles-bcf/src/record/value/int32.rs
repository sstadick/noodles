#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Int32 {
    Value(i32),
    Missing,
    EndOfVector,
    Reserved(i32),
}

impl Int32 {
    /// The smallest value that can be represented by [`Self::Value`].
    pub const MIN_VALUE: i32 = i32::MIN + 8;

    /// The largest value that can be represented by [`Self::Value`].
    pub const MAX_VALUE: i32 = i32::MAX;
}

impl From<i32> for Int32 {
    fn from(value: i32) -> Self {
        match value as u32 {
            0x80000000 => Self::Missing,
            0x80000001 => Self::EndOfVector,
            0x80000002..=0x80000007 => Self::Reserved(value),
            _ => Self::Value(value),
        }
    }
}

impl From<Int32> for i32 {
    fn from(value: Int32) -> Self {
        match value {
            Int32::Missing => -2147483648,
            Int32::EndOfVector => -2147483647,
            Int32::Value(n) | Int32::Reserved(n) => n,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_i32_for_int32() {
        assert_eq!(Int32::from(0), Int32::Value(0));
        assert_eq!(Int32::from(-2147483648), Int32::Missing);
        assert_eq!(Int32::from(-2147483647), Int32::EndOfVector);
        assert_eq!(Int32::from(-2147483646), Int32::Reserved(-2147483646));
        assert_eq!(Int32::from(-2147483645), Int32::Reserved(-2147483645));
        assert_eq!(Int32::from(-2147483644), Int32::Reserved(-2147483644));
        assert_eq!(Int32::from(-2147483643), Int32::Reserved(-2147483643));
        assert_eq!(Int32::from(-2147483642), Int32::Reserved(-2147483642));
        assert_eq!(Int32::from(-2147483641), Int32::Reserved(-2147483641));
    }

    #[test]
    fn test_from_int32_for_i32() {
        assert_eq!(i32::from(Int32::Value(0)), 0);
        assert_eq!(i32::from(Int32::Missing), -2147483648);
        assert_eq!(i32::from(Int32::EndOfVector), -2147483647);
        assert_eq!(i32::from(Int32::Reserved(-2147483646)), -2147483646);
        assert_eq!(i32::from(Int32::Reserved(-2147483645)), -2147483645);
        assert_eq!(i32::from(Int32::Reserved(-2147483644)), -2147483644);
        assert_eq!(i32::from(Int32::Reserved(-2147483643)), -2147483643);
        assert_eq!(i32::from(Int32::Reserved(-2147483642)), -2147483642);
        assert_eq!(i32::from(Int32::Reserved(-2147483641)), -2147483641);
    }
}
