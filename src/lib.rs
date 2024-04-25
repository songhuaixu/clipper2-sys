mod clipper2;
pub(crate) use clipper2::*;

mod clipper_offset;
pub use clipper_offset::*;

mod double;
pub use double::*;

mod int64;
pub use int64::*;

#[cfg(test)]
mod tests;

pub(crate) unsafe fn malloc(size: usize) -> *mut std::os::raw::c_void {
    libc::malloc(size)
}

pub(crate) unsafe fn free(p: *mut std::os::raw::c_void) {
    libc::free(p)
}

#[derive(Clone, Copy)]
pub enum FillRule {
    EvenOdd,
    NonZero,
    Positive,
    Negative,
}

#[derive(Clone, Copy)]
pub enum ClipType {
    None,
    Intersection,
    Union,
    Difference,
    Xor,
}

#[derive(Clone, Copy)]
pub enum JoinType {
    SquaerJoin,
    RoundJoin,
    MiterJoin,
}

#[derive(Clone, Copy)]
pub enum EndType {
    PolygonEnd,
    JoinedEnd,
    SquaerEnd,
    RoundEnd,
}

impl From<ClipType> for ClipperClipType {
    fn from(value: ClipType) -> Self {
        match value {
            ClipType::None => ClipperClipType_NONE,
            ClipType::Intersection => ClipperClipType_INTERSECTION,
            ClipType::Union => ClipperClipType_UNION,
            ClipType::Difference => ClipperClipType_DIFFERENCE,
            ClipType::Xor => ClipperClipType_XOR,
        }
    }
}

impl From<FillRule> for ClipperFillRule {
    fn from(value: FillRule) -> Self {
        match value {
            FillRule::EvenOdd => ClipperFillRule_EVEN_ODD,
            FillRule::NonZero => ClipperFillRule_NON_ZERO,
            FillRule::Positive => ClipperFillRule_POSITIVE,
            FillRule::Negative => ClipperFillRule_NEGATIVE,
        }
    }
}

impl From<JoinType> for ClipperJoinType {
    fn from(value: JoinType) -> Self {
        match value {
            JoinType::SquaerJoin => ClipperJoinType_SQUARE_JOIN,
            JoinType::RoundJoin => ClipperJoinType_ROUND_JOIN,
            JoinType::MiterJoin => ClipperJoinType_MITER_JOIN,
        }
    }
}

impl From<EndType> for ClipperEndType {
    fn from(value: EndType) -> Self {
        match value {
            EndType::PolygonEnd => ClipperEndType_POLYGON_END,
            EndType::JoinedEnd => ClipperEndType_JOINED_END,
            EndType::SquaerEnd => ClipperEndType_SQUARE_END,
            EndType::RoundEnd => ClipperEndType_ROUND_END,
        }
    }
}