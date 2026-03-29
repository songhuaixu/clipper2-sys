//! Double-precision paths (`PointD`, `PathD`, `PathsD`) and [`ClipperD`].
//!
//! 双精度坐标路径与 [`ClipperD`]。

mod clipper;
pub use clipper::*;

mod path;
pub use path::*;

mod paths;
pub use paths::*;

use crate::cxx_bridge::clipper2_sys_cxx;
use crate::cxx_bridge::clipper2_sys_cxx::ClipperFillRule;
use crate::paths_blob::{pathd_to_blob, pathsd_to_blob, DEFAULT_PATHD_PRECISION};
use crate::{EndType, JoinType, LazyPaths64, PointInPolygonResult};

/// Double-precision point (`PointD`); same layout as cxx `PD`.
///
/// 双精度点，布局与 cxx `PD` 一致。
pub type PointD = crate::cxx_bridge::clipper2_sys_cxx::PD;

impl PointD {
    /// Creates a point. / 构造点。
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl PathD {
    /// Simplifies this path. / 简化路径。
    pub fn simplify(&self, epsilon: f64, is_open_path: bool) -> LazyPathsD {
        let out = clipper2_sys_cxx::cxx_pathd_simplify(&pathd_to_blob(self), epsilon, is_open_path);
        LazyPathsD::from_blob(out)
    }

    /// Point-in-polygon test. / 点在多边形内外判定。
    pub fn point_in_polygon(&self, point: PointD) -> PointInPolygonResult {
        clipper2_sys_cxx::cxx_point_in_pathd(&pathd_to_blob(self), point.x, point.y).into()
    }

    /// Quantizes to `Path64` (default precision). / 量化为 `Path64`（默认精度）。
    pub fn to_path64(&self) -> LazyPaths64 {
        let b =
            clipper2_sys_cxx::cxx_pathd_to_path64(&pathd_to_blob(self), DEFAULT_PATHD_PRECISION);
        LazyPaths64::from_blob(b)
    }
}

impl PathsD {
    /// Simplifies all paths. / 简化所有路径。
    pub fn simplify(&self, epsilon: f64, is_open_path: bool) -> LazyPathsD {
        let out =
            clipper2_sys_cxx::cxx_pathsd_simplify(&pathsd_to_blob(self), epsilon, is_open_path);
        LazyPathsD::from_blob(out)
    }

    /// Offsets all paths; `precision` is ClipperD decimal precision.
    ///
    /// 偏移所有路径；`precision` 为 ClipperD 小数精度。
    pub fn inflate(
        &self,
        delta: f64,
        join_type: JoinType,
        end_type: EndType,
        miter_limit: f64,
        precision: i32,
    ) -> LazyPathsD {
        let out = clipper2_sys_cxx::cxx_pathsd_inflate(
            &pathsd_to_blob(self),
            delta,
            join_type.into(),
            end_type.into(),
            miter_limit,
            precision,
        );
        LazyPathsD::from_blob(out)
    }

    /// Converts to integer paths. / 转为整数路径集合。
    pub fn to_paths64(&self) -> LazyPaths64 {
        let b =
            clipper2_sys_cxx::cxx_pathsd_to_paths64(&pathsd_to_blob(self), DEFAULT_PATHD_PRECISION);
        LazyPaths64::from_blob(b)
    }
}

impl PathD {
    /// Minkowski sum in double space. / 双精度闵可夫斯基和。
    pub fn minkowski_sum(&self, pattern: &PathD, is_closed: bool, precision: i32) -> LazyPathsD {
        LazyPathsD::from_blob(clipper2_sys_cxx::cxx_pathd_minkowski_sum(
            &pathd_to_blob(pattern),
            &pathd_to_blob(self),
            is_closed,
            precision,
        ))
    }

    /// Minkowski difference. / 闵可夫斯基差。
    pub fn minkowski_diff(&self, pattern: &PathD, is_closed: bool, precision: i32) -> LazyPathsD {
        LazyPathsD::from_blob(clipper2_sys_cxx::cxx_pathd_minkowski_diff(
            &pathd_to_blob(pattern),
            &pathd_to_blob(self),
            is_closed,
            precision,
        ))
    }
}

impl PathsD {
    /// Minkowski sum with fill rule. / 闵可夫斯基和（填充规则）。
    pub fn minkowski_sum(
        &self,
        pattern: &PathD,
        is_closed: bool,
        precision: i32,
        fillrule: ClipperFillRule,
    ) -> LazyPathsD {
        LazyPathsD::from_blob(clipper2_sys_cxx::cxx_pathsd_minkowski_sum(
            &pathd_to_blob(pattern),
            &pathsd_to_blob(self),
            is_closed,
            precision,
            fillrule,
        ))
    }

    /// Minkowski difference with fill rule. / 闵可夫斯基差（填充规则）。
    pub fn minkowski_diff(
        &self,
        pattern: &PathD,
        is_closed: bool,
        precision: i32,
        fillrule: ClipperFillRule,
    ) -> LazyPathsD {
        LazyPathsD::from_blob(clipper2_sys_cxx::cxx_pathsd_minkowski_diff(
            &pathd_to_blob(pattern),
            &pathsd_to_blob(self),
            is_closed,
            precision,
            fillrule,
        ))
    }
}

impl PathD {
    /// Signed area. / 有向面积。
    pub fn area(&self) -> f64 {
        clipper2_sys_cxx::cxx_pathd_area(&pathd_to_blob(self))
    }
}

impl PathsD {
    /// Sum of signed areas. / 有向面积之和。
    pub fn area(&self) -> f64 {
        clipper2_sys_cxx::cxx_pathsd_area(&pathsd_to_blob(self))
    }
}

#[cfg(test)]
mod tests {
    use super::{PathD, PointD};

    #[test]
    fn pathd_area_unit_square() {
        let p = PathD::new(vec![
            PointD::new(0.0, 0.0),
            PointD::new(10.0, 0.0),
            PointD::new(10.0, 10.0),
            PointD::new(0.0, 10.0),
        ]);
        assert!((p.area().abs() - 100.0).abs() < 1e-6);
    }

    #[test]
    fn pathd_simplify_reduces_collinear() {
        let p = PathD::new(vec![
            PointD::new(0.0, 0.0),
            PointD::new(5.0, 0.0),
            PointD::new(10.0, 0.0),
            PointD::new(10.0, 10.0),
            PointD::new(0.0, 10.0),
        ]);
        let s = p.simplify(1.0, false);
        assert!(s.into_first_path().len() <= p.len());
    }
}
