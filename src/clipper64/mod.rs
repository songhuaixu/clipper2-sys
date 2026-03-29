//! Integer grid paths (`Point64`, `Path64`, `Paths64`) and [`Clipper64`].
//!
//! 整数网格坐标路径与布尔裁剪主入口 [`Clipper64`]。

mod clipper;
pub use clipper::*;

mod path;
pub use path::*;

mod paths;
pub use paths::*;

use crate::cxx_bridge::clipper2_sys_cxx;
use crate::cxx_bridge::clipper2_sys_cxx::ClipperFillRule;
use crate::paths_blob::{path64_to_blob, paths64_to_blob};
use crate::{EndType, JoinType, LazyPathsD, PointInPolygonResult};

/// Single point on the integer Clipper2 grid (`Point64`); same layout as cxx `P64`.
///
/// Clipper2 整数网格上的点，布局与 cxx `P64` 一致。
pub type Point64 = crate::cxx_bridge::clipper2_sys_cxx::P64;

impl Point64 {
    /// Creates a point from coordinates. / 由坐标构造点。
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Path64 {
    /// Simplifies the path (RDP-style via Clipper). `is_open_path` marks open polylines.
    ///
    /// 简化路径（Clipper 实现）。`is_open_path` 表示开放折线。
    pub fn simplify(&self, epsilon: f64, is_open_path: bool) -> LazyPaths64 {
        let out = clipper2_sys_cxx::cxx_path64_simplify(
            &path64_to_blob(self),
            epsilon,
            is_open_path,
        );
        LazyPaths64::from_blob(out)
    }

    /// Tests where `point` lies relative to this path. / 判断 `point` 与路径的位置关系。
    pub fn point_in_polygon(&self, point: Point64) -> PointInPolygonResult {
        clipper2_sys_cxx::cxx_point_in_path64(&path64_to_blob(self), point.x, point.y).into()
    }

    /// Converts to double paths (`LazyPathsD`) for `ClipperD` workflows.
    ///
    /// 转为双精度路径，便于走 `ClipperD` 流程。
    pub fn to_pathd(&self) -> LazyPathsD {
        let b = clipper2_sys_cxx::cxx_path64_to_pathd(&path64_to_blob(self));
        LazyPathsD::from_blob(b)
    }
}

impl Paths64 {
    /// Simplifies every path. / 对每条路径做简化。
    pub fn simplify(&self, epsilon: f64, is_open_path: bool) -> LazyPaths64 {
        let out = clipper2_sys_cxx::cxx_paths64_simplify(
            &paths64_to_blob(self),
            epsilon,
            is_open_path,
        );
        LazyPaths64::from_blob(out)
    }

    /// Offsets (inflates/deflates) all paths by `delta` using join/end rules.
    ///
    /// 按连接/端点规则对全部路径做 `delta` 偏移（膨胀或收缩）。
    pub fn inflate(
        &self,
        delta: f64,
        join_type: JoinType,
        end_type: EndType,
        miter_limit: f64,
    ) -> LazyPaths64 {
        let out = clipper2_sys_cxx::cxx_paths64_inflate(
            &paths64_to_blob(self),
            delta,
            join_type.into(),
            end_type.into(),
            miter_limit,
        );
        LazyPaths64::from_blob(out)
    }

    /// Converts all paths to `PathD` blobs. / 将全部路径转为双精度。
    pub fn to_pathsd(&self) -> LazyPathsD {
        let b = clipper2_sys_cxx::cxx_paths64_to_pathsd(&paths64_to_blob(self));
        LazyPathsD::from_blob(b)
    }
}

impl Path64 {
    /// Minkowski sum with `pattern`; `is_closed` toggles closed semantics.
    ///
    /// 与 `pattern` 的闵可夫斯基和。
    pub fn minkowski_sum(&self, pattern: &Path64, is_closed: bool) -> LazyPaths64 {
        LazyPaths64::from_blob(clipper2_sys_cxx::cxx_path64_minkowski_sum(
            &path64_to_blob(pattern),
            &path64_to_blob(self),
            is_closed,
        ))
    }

    /// Minkowski difference. / 闵可夫斯基差。
    pub fn minkowski_diff(&self, pattern: &Path64, is_closed: bool) -> LazyPaths64 {
        LazyPaths64::from_blob(clipper2_sys_cxx::cxx_path64_minkowski_diff(
            &path64_to_blob(pattern),
            &path64_to_blob(self),
            is_closed,
        ))
    }
}

impl Paths64 {
    /// Minkowski sum against each path with fill rule. / 对多条路径做闵可夫斯基和（含填充规则）。
    pub fn minkowski_sum(
        &self,
        pattern: &Path64,
        is_closed: bool,
        fillrule: ClipperFillRule,
    ) -> LazyPaths64 {
        LazyPaths64::from_blob(clipper2_sys_cxx::cxx_paths64_minkowski_sum(
            &path64_to_blob(pattern),
            &paths64_to_blob(self),
            is_closed,
            fillrule,
        ))
    }

    /// Minkowski difference with fill rule. / 闵可夫斯基差（含填充规则）。
    pub fn minkowski_diff(
        &self,
        pattern: &Path64,
        is_closed: bool,
        fillrule: ClipperFillRule,
    ) -> LazyPaths64 {
        LazyPaths64::from_blob(clipper2_sys_cxx::cxx_paths64_minkowski_diff(
            &path64_to_blob(pattern),
            &paths64_to_blob(self),
            is_closed,
            fillrule,
        ))
    }
}

impl Path64 {
    /// Signed area of a closed path. / 闭合路径的有向面积。
    pub fn area(&self) -> f64 {
        clipper2_sys_cxx::cxx_path64_area(&path64_to_blob(self))
    }
}

impl Paths64 {
    /// Sum of signed areas. / 各路径有向面积之和。
    pub fn area(&self) -> f64 {
        clipper2_sys_cxx::cxx_paths64_area(&paths64_to_blob(self))
    }
}

#[cfg(test)]
mod tests {
    use super::{Path64, Point64, PointInPolygonResult};

    #[test]
    fn path64_area_unit_square() {
        let p = Path64::new(vec![
            Point64::new(0, 0),
            Point64::new(10, 0),
            Point64::new(10, 10),
            Point64::new(0, 10),
        ]);
        assert!((p.area().abs() - 100.0).abs() < 1e-6);
    }

    #[test]
    fn point_in_polygon_center_inside_square() {
        let p = Path64::new(vec![
            Point64::new(0, 0),
            Point64::new(10, 0),
            Point64::new(10, 10),
            Point64::new(0, 10),
        ]);
        let r = p.point_in_polygon(Point64::new(5, 5));
        assert!(matches!(r, PointInPolygonResult::Inside));
    }
}
