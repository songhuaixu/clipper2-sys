//!
//! # Examples cookbook / 示例合集
//!
//! ## `Clipper64`: union + lazy closed / 布尔并 + 惰性闭合解
//!
//! ```
//! use clipper2_sys::{
//!     ClipType, Clipper64, FillRule, Paths64, Point64,
//! };
//! # use clipper2_sys::Path64;
//! # fn square_i(x0: i64, y0: i64, s: i64) -> Path64 {
//! #     Path64::new(vec![Point64::new(x0, y0), Point64::new(x0 + s, y0),
//! #         Point64::new(x0 + s, y0 + s), Point64::new(x0, y0 + s)])
//! # }
//!
//! let mut clip = Clipper64::new();
//! clip.add_subject(&Paths64::new(vec![square_i(0, 0, 100)]));
//! clip.add_clip(&Paths64::new(vec![square_i(50, 50, 100)]));
//! let sol = clip.execute(ClipType::Union, FillRule::NonZero);
//! let (closed, _open) = sol.into_lazy();
//! assert!(!closed.is_empty());
//! ```
//!
//! ## `Clipper64`: iterate closed paths / 逐条遍历闭合路径
//!
//! ```
//! use clipper2_sys::{
//!     ClipType, Clipper64, FillRule, Path64, Paths64, Point64,
//! };
//! # fn square_i(x0: i64, y0: i64, s: i64) -> Path64 {
//! #     Path64::new(vec![Point64::new(x0, y0), Point64::new(x0 + s, y0),
//! #         Point64::new(x0 + s, y0 + s), Point64::new(x0, y0 + s)])
//! # }
//!
//! let mut clip = Clipper64::new();
//! clip.add_subject(&Paths64::new(vec![square_i(0, 0, 100)]));
//! clip.add_clip(&Paths64::new(vec![square_i(50, 50, 100)]));
//! let sol = clip.execute(ClipType::Union, FillRule::NonZero);
//! let all: Paths64 = sol.iter_closed().chain(sol.iter_open()).collect();
//! assert!(!all.is_empty());
//! ```
//!
//! ## `Clipper64`: `execute_tree` + preorder on `PolyPath` / 树形解与前序遍历
//!
//! ```
//! use clipper2_sys::{
//!     ClipType, Clipper64, FillRule, Path64, Paths64, Point64,
//! };
//! # fn square_i(x0: i64, y0: i64, s: i64) -> Path64 {
//! #     Path64::new(vec![Point64::new(x0, y0), Point64::new(x0 + s, y0),
//! #         Point64::new(x0 + s, y0 + s), Point64::new(x0, y0 + s)])
//! # }
//!
//! let mut clip = Clipper64::new();
//! clip.add_subject(&Paths64::new(vec![square_i(0, 0, 100)]));
//! clip.add_clip(&Paths64::new(vec![square_i(50, 50, 100)]));
//! let sol = clip.execute_tree(ClipType::Union, FillRule::NonZero);
//! let (_open_lazy, preorder) = sol.into_open_and_poly_preorder();
//! let n = preorder.count();
//! assert!(n > 0);
//! ```
//!
//! ## `ClipperD`: union / 双精度布尔并
//!
//! ```
//! use clipper2_sys::{
//!     ClipType, ClipperD, FillRule, PathD, PathsD, PointD,
//! };
//! # fn square_f(x0: f64, y0: f64, s: f64) -> PathD {
//! #     PathD::new(vec![PointD::new(x0, y0), PointD::new(x0 + s, y0),
//! #         PointD::new(x0 + s, y0 + s), PointD::new(x0, y0 + s)])
//! # }
//!
//! let mut clip = ClipperD::new(4);
//! clip.add_subject(&PathsD::new(vec![square_f(0.0, 0.0, 100.0)]));
//! clip.add_clip(&PathsD::new(vec![square_f(50.0, 50.0, 100.0)]));
//! let sol = clip.execute(ClipType::Union, FillRule::NonZero);
//! let (closed, _open) = sol.into_lazy();
//! assert!(!closed.is_empty());
//! ```
//!
//! ## `ClipperOffset`: inflate a square / 方形外扩
//!
//! ```
//! use clipper2_sys::{ClipperOffset, EndType, JoinType, Path64, Point64};
//!
//! let path = Path64::new(vec![
//!     Point64::new(0, 0),
//!     Point64::new(100, 0),
//!     Point64::new(100, 100),
//!     Point64::new(0, 100),
//! ]);
//! let mut co = ClipperOffset::new(2.0, 0.0, false, false);
//! co.add_path(&path, JoinType::MiterJoin, EndType::PolygonEnd);
//! let out = co.execute(10.0);
//! assert!(!out.is_empty());
//! ```
//!
//! ## `Path64`: area, point-in-polygon, translate, simplify / 面积、点包含、平移、简化
//!
//! ```
//! use clipper2_sys::{Path64, Point64, PointInPolygonResult};
//!
//! let p = Path64::new(vec![
//!     Point64::new(0, 0),
//!     Point64::new(10, 0),
//!     Point64::new(10, 10),
//!     Point64::new(0, 10),
//! ]);
//! assert!((p.area().abs() - 100.0).abs() < 1e-3);
//! assert!(matches!(
//!     p.point_in_polygon(Point64::new(5, 5)),
//!     PointInPolygonResult::Inside
//! ));
//! let t = p.translate(3, -2);
//! assert_eq!(t.get_point(0).x, 3);
//! let collinear = Path64::new(vec![
//!     Point64::new(0, 0),
//!     Point64::new(5, 0),
//!     Point64::new(10, 0),
//!     Point64::new(10, 10),
//! ]);
//! let simp = collinear.simplify(1.0, false);
//! assert!(simp.into_first_path().len() <= collinear.len());
//! ```
//!
//! ## `Paths64`: inflate (offset helper) / 多路径偏移
//!
//! ```
//! use clipper2_sys::{EndType, JoinType, Path64, Paths64, Point64};
//!
//! let paths = Paths64::new(vec![Path64::new(vec![
//!     Point64::new(0, 0),
//!     Point64::new(100, 0),
//!     Point64::new(100, 100),
//!     Point64::new(0, 100),
//! ])]);
//! let grown = paths.inflate(10.0, JoinType::MiterJoin, EndType::PolygonEnd, 2.0);
//! assert!(!grown.is_empty());
//! ```
//!
//! ## `Path64` / `Paths64`: Minkowski sum / 闵可夫斯基和
//!
//! ```
//! use clipper2_sys::{FillRule, Path64, Paths64, Point64};
//! # fn square_i(x0: i64, y0: i64, s: i64) -> Path64 {
//! #     Path64::new(vec![Point64::new(x0, y0), Point64::new(x0 + s, y0),
//! #         Point64::new(x0 + s, y0 + s), Point64::new(x0, y0 + s)])
//! # }
//!
//! let a = square_i(0, 0, 50);
//! let b = square_i(0, 0, 30);
//! let ms = a.minkowski_sum(&b, true);
//! assert!(!ms.is_empty());
//!
//! let many = Paths64::new(vec![square_i(0, 0, 40)]);
//! let ms2 = many.minkowski_sum(&b, true, FillRule::NonZero.into());
//! assert!(!ms2.is_empty());
//! ```
//!
//! ## `PathD` ↔ `Path64`: simplify and convert / 双精度简化与转整型
//!
//! ```
//! use clipper2_sys::{LazyPaths64, PathD, PointD};
//!
//! let p = PathD::new(vec![
//!     PointD::new(0.0, 0.0),
//!     PointD::new(5.0, 0.0),
//!     PointD::new(10.0, 0.0),
//!     PointD::new(10.0, 10.0),
//! ]);
//! let simplified = p.simplify(1.0, false);
//! let _: LazyPaths64 = p.to_path64();
//! assert!(simplified.into_first_path().len() <= p.len());
//! ```
//!
//! ## `ClipSolution`: `to_closed` / 物化全部闭合多边形
//!
//! ```
//! use clipper2_sys::{
//!     ClipType, Clipper64, FillRule, Path64, Paths64, Point64,
//! };
//! # fn square_i(x0: i64, y0: i64, s: i64) -> Path64 {
//! #     Path64::new(vec![Point64::new(x0, y0), Point64::new(x0 + s, y0),
//! #         Point64::new(x0 + s, y0 + s), Point64::new(x0, y0 + s)])
//! # }
//!
//! let mut clip = Clipper64::new();
//! clip.add_subject(&Paths64::new(vec![square_i(0, 0, 100)]));
//! clip.add_clip(&Paths64::new(vec![square_i(50, 50, 100)]));
//! let sol = clip.execute(ClipType::Union, FillRule::NonZero);
//! let closed: Paths64 = sol.to_closed();
//! assert!(!closed.is_empty());
//! ```
//!
//! ## `ClipTreeSolution`: open-only shortcut / 只要开放解
//!
//! ```
//! use clipper2_sys::{
//!     ClipType, Clipper64, FillRule, Path64, Paths64, Point64,
//! };
//! # fn square_i(x0: i64, y0: i64, s: i64) -> Path64 {
//! #     Path64::new(vec![Point64::new(x0, y0), Point64::new(x0 + s, y0),
//! #         Point64::new(x0 + s, y0 + s), Point64::new(x0, y0 + s)])
//! # }
//!
//! let mut clip = Clipper64::new();
//! clip.add_subject(&Paths64::new(vec![square_i(0, 0, 100)]));
//! clip.add_clip(&Paths64::new(vec![square_i(50, 50, 100)]));
//! let sol = clip.execute_tree(ClipType::Union, FillRule::NonZero);
//! let _open = sol.into_open_lazy();
//! ```
//!
//! # Module map / 模块结构
//!
//! - `clipper64` (logic in `src/clipper64/`, types re-exported here) — integer paths and
//!   [`Clipper64`]. / 整数路径与 [`Clipper64`]，实现位于 `clipper64` 子目录。
//! - `clipperd` — double paths and [`ClipperD`]. / 双精度路径与 [`ClipperD`]。
//! - [`ClipperOffset`] — path offset (inflate/deflate). / 路径偏移。

#[allow(dead_code)]
mod cxx_bridge;

#[macro_use]
mod macros;

mod paths_blob;
pub use paths_blob::{PathsBlob64Iter, PathsBlobDIter};

mod poly_path;
pub use poly_path::{PolyCxxNode64, PolyCxxNodeD, PolyCxxPreorderIter64, PolyCxxPreorderIterD};

mod offset;
pub use offset::*;

mod clipper64;
pub use clipper64::*;

mod clipperd;
pub use clipperd::*;

/// Polygon fill rule (non-zero, even-odd, …), maps to Clipper2 `FillRule`.
///
/// 多边形填充规则，对应 Clipper2 `FillRule`。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FillRule {
    /// Even-odd rule. / 偶奇规则。
    EvenOdd,
    /// Non-zero winding rule. / 非零环绕规则。
    NonZero,
    /// Positive winding only. / 仅正向环绕。
    Positive,
    /// Negative winding only. / 仅负向环绕。
    Negative,
}

/// Boolean clip operation between subject and clip paths.
///
/// Subject 与 Clip 之间的裁剪运算类型。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClipType {
    /// No clipping (placeholder). / 无裁剪（占位）。
    None,
    /// Intersection. / 交集。
    Intersection,
    /// Union. / 并集。
    Union,
    /// Difference (subject minus clip). / 差集（Subject 减 Clip）。
    Difference,
    /// Exclusive OR. / 异或。
    Xor,
}

/// Vertex join style for offsets and offset-like operations.
///
/// 顶点连接样式，用于偏移等运算。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum JoinType {
    /// Square corner join. / 方角连接。
    SquareJoin,
    /// Round join. / 圆角连接。
    RoundJoin,
    /// Miter join. / 尖角连接。
    MiterJoin,
}

/// End cap / polygon closure mode for open paths in offsetting.
///
/// 开放路径在偏移时的端点与闭合模式。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EndType {
    /// Closed polygon. / 闭合多边形。
    PolygonEnd,
    /// Open path with joined ends. / 开放路径且端点相连。
    JoinedEnd,
    /// Square end cap. / 平头端点。
    SquareEnd,
    /// Round end cap. / 圆头端点。
    RoundEnd,
}

/// Point-in-polygon classification result.
///
/// 点在多边形内外的判定结果。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PointInPolygonResult {
    /// Point lies on an edge. / 点在边上。
    On,
    /// Point strictly inside. / 点在内部。
    Inside,
    /// Point strictly outside. / 点在外部。
    Outside,
}

/// Maps our [`ClipType`] to the cxx bridge enum. / [`ClipType`] → cxx 枚举。
impl From<ClipType> for crate::cxx_bridge::clipper2_sys_cxx::ClipperClipType {
    fn from(value: ClipType) -> Self {
        use crate::cxx_bridge::clipper2_sys_cxx::ClipperClipType;
        match value {
            ClipType::None => ClipperClipType::NoClip,
            ClipType::Intersection => ClipperClipType::Intersection,
            ClipType::Union => ClipperClipType::Union,
            ClipType::Difference => ClipperClipType::Difference,
            ClipType::Xor => ClipperClipType::Xor,
        }
    }
}

/// Maps our [`FillRule`] to the cxx bridge enum. / [`FillRule`] → cxx。
impl From<FillRule> for crate::cxx_bridge::clipper2_sys_cxx::ClipperFillRule {
    fn from(value: FillRule) -> Self {
        use crate::cxx_bridge::clipper2_sys_cxx::ClipperFillRule;
        match value {
            FillRule::EvenOdd => ClipperFillRule::EvenOdd,
            FillRule::NonZero => ClipperFillRule::NonZero,
            FillRule::Positive => ClipperFillRule::Positive,
            FillRule::Negative => ClipperFillRule::Negative,
        }
    }
}

/// Maps our [`JoinType`] to the cxx bridge enum. / [`JoinType`] → cxx。
impl From<JoinType> for crate::cxx_bridge::clipper2_sys_cxx::ClipperJoinType {
    fn from(value: JoinType) -> Self {
        use crate::cxx_bridge::clipper2_sys_cxx::ClipperJoinType;
        match value {
            JoinType::SquareJoin => ClipperJoinType::Square,
            JoinType::RoundJoin => ClipperJoinType::Round,
            JoinType::MiterJoin => ClipperJoinType::Miter,
        }
    }
}

/// Maps our [`EndType`] to the cxx bridge enum. / [`EndType`] → cxx。
impl From<EndType> for crate::cxx_bridge::clipper2_sys_cxx::ClipperEndType {
    fn from(value: EndType) -> Self {
        use crate::cxx_bridge::clipper2_sys_cxx::ClipperEndType;
        match value {
            EndType::PolygonEnd => ClipperEndType::Polygon,
            EndType::JoinedEnd => ClipperEndType::Joined,
            EndType::SquareEnd => ClipperEndType::Square,
            EndType::RoundEnd => ClipperEndType::Round,
        }
    }
}

/// Maps cxx point-in-polygon result to [`PointInPolygonResult`]. / cxx 判定结果 → [`PointInPolygonResult`]。
impl From<crate::cxx_bridge::clipper2_sys_cxx::ClipperPointInPolygonResult> for PointInPolygonResult {
    fn from(value: crate::cxx_bridge::clipper2_sys_cxx::ClipperPointInPolygonResult) -> Self {
        use crate::cxx_bridge::clipper2_sys_cxx::ClipperPointInPolygonResult;
        match value {
            ClipperPointInPolygonResult::IsOn => PointInPolygonResult::On,
            ClipperPointInPolygonResult::IsInside => PointInPolygonResult::Inside,
            ClipperPointInPolygonResult::IsOutside => PointInPolygonResult::Outside,
            _ => PointInPolygonResult::Outside,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cxx_bridge::clipper2_sys_cxx::{
        ClipperClipType, ClipperFillRule, ClipperPointInPolygonResult,
    };

    #[test]
    fn fill_rule_into_clipper_matches_even_odd() {
        let v: ClipperFillRule = FillRule::EvenOdd.into();
        assert_eq!(v, ClipperFillRule::EvenOdd);
    }

    #[test]
    fn clip_type_into_clipper_matches_union() {
        let v: ClipperClipType = ClipType::Union.into();
        assert_eq!(v, ClipperClipType::Union);
    }

    #[test]
    fn point_in_polygon_result_from_clipper() {
        let r: PointInPolygonResult = ClipperPointInPolygonResult::IsInside.into();
        assert!(matches!(r, PointInPolygonResult::Inside));
    }
}
