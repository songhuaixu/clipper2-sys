//! Conversions between flat `PathsBlob*` and Rust [`crate::Path64`]/[`crate::PathD`], plus iterators.
//!
//! `PathsBlob64` / `PathsBlobD` 与 Rust 路径类型互转及惰性迭代器。

use crate::cxx_bridge::clipper2_sys_cxx::{P64, PD, PathsBlob64, PathsBlobD};
use crate::{Path64, PathD, Paths64, PathsD};

/// Default decimal precision when no `ClipperD` instance sets it (`PathD` → `Path64`).
///
/// 无 `ClipperD` 实例时，`PathD` ↔ `Path64` 等转换使用的默认小数精度。
pub(crate) const DEFAULT_PATHD_PRECISION: i32 = 4;

/// Converts a flat point slice to [`Path64`] (memcpy). / 扁平淡点切片 → `Path64`。
#[inline]
pub(crate) fn path64_from_p64_slice(points: &[P64]) -> Path64 {
    Path64::from_vec(points.to_vec())
}

/// Converts a flat point slice to [`PathD`]. / 扁平淡点切片 → `PathD`。
#[inline]
pub(crate) fn pathd_from_pd_slice(points: &[PD]) -> PathD {
    PathD::from_vec(points.to_vec())
}

define_paths_blob_iter! {
    /// Iterator over each path inside a `PathsBlob64` (allocates one [`Path64`] per step).
    ///
    /// 遍历 `PathsBlob64` 中每条路径（每步分配一条 [`Path64`]）。
    pub PathsBlob64Iter,
    point = P64,
    path = Path64,
    blob = PathsBlob64,
    conv_slice = path64_from_p64_slice,
}

define_paths_blob_iter! {
    /// Iterator over paths in a `PathsBlobD`. / 遍历 `PathsBlobD` 中的路径。
    pub PathsBlobDIter,
    point = PD,
    path = PathD,
    blob = PathsBlobD,
    conv_slice = pathd_from_pd_slice,
}

/// Single-path `Path64` → cxx blob (`path_starts = [0, n]`). / 单路径 → cxx blob。
#[inline]
pub(crate) fn path64_to_blob(p: &Path64) -> PathsBlob64 {
    let n = p.0.len();
    PathsBlob64 {
        points: p.0.clone(),
        path_starts: vec![0, n],
    }
}

/// Many `Path64` → one blob. / 多路径扁平化。
pub(crate) fn paths64_to_blob(p: &Paths64) -> PathsBlob64 {
    let n_paths = p.0.len();
    let total_pts: usize = p.0.iter().map(|path| path.0.len()).sum();
    let mut points = Vec::with_capacity(total_pts);
    let mut path_starts = Vec::with_capacity(n_paths + 1);
    path_starts.push(0);
    for path in &p.0 {
        points.extend_from_slice(&path.0);
        path_starts.push(points.len());
    }
    PathsBlob64 { points, path_starts }
}

/// First polygon in `PolyPath` blob → one [`Path64`]. / 多边形树单轮廓 blob → `Path64`。
pub(crate) fn path64_from_single_paths_blob(b: PathsBlob64) -> Path64 {
    if b.path_starts.len() < 2 {
        return Path64::default();
    }
    let a = b.path_starts[0];
    let e = b.path_starts[1];
    path64_from_p64_slice(&b.points[a..e])
}

/// Materialize all paths from a blob. / 由 blob 物化全部路径。
pub(crate) fn paths64_from_blob(b: PathsBlob64) -> Paths64 {
    if b.path_starts.len() < 2 {
        return Paths64::default();
    }
    let n_paths = b.path_starts.len() - 1;
    let mut paths = Vec::with_capacity(n_paths);
    for w in b.path_starts.windows(2) {
        let (a, e) = (w[0], w[1]);
        paths.push(path64_from_p64_slice(&b.points[a..e]));
    }
    Paths64::from_vec(paths)
}

#[inline]
pub(crate) fn pathd_to_blob(p: &PathD) -> PathsBlobD {
    let n = p.0.len();
    PathsBlobD {
        points: p.0.clone(),
        path_starts: vec![0, n],
    }
}

pub(crate) fn pathsd_to_blob(p: &PathsD) -> PathsBlobD {
    let n_paths = p.0.len();
    let total_pts: usize = p.0.iter().map(|path| path.0.len()).sum();
    let mut points = Vec::with_capacity(total_pts);
    let mut path_starts = Vec::with_capacity(n_paths + 1);
    path_starts.push(0);
    for path in &p.0 {
        points.extend_from_slice(&path.0);
        path_starts.push(points.len());
    }
    PathsBlobD { points, path_starts }
}

pub(crate) fn pathd_from_single_paths_blob(b: PathsBlobD) -> PathD {
    if b.path_starts.len() < 2 {
        return PathD::default();
    }
    let a = b.path_starts[0];
    let e = b.path_starts[1];
    pathd_from_pd_slice(&b.points[a..e])
}

pub(crate) fn pathsd_from_blob(b: PathsBlobD) -> PathsD {
    if b.path_starts.len() < 2 {
        return PathsD::default();
    }
    let n_paths = b.path_starts.len() - 1;
    let mut paths = Vec::with_capacity(n_paths);
    for w in b.path_starts.windows(2) {
        let (a, e) = (w[0], w[1]);
        paths.push(pathd_from_pd_slice(&b.points[a..e]));
    }
    PathsD::from_vec(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Path64, PathD, Paths64, PathsD, Point64, PointD};

    #[test]
    fn path64_single_blob_roundtrip() {
        let p = Path64::new(vec![
            Point64::new(0, 0),
            Point64::new(10, 0),
            Point64::new(10, 10),
        ]);
        let blob = path64_to_blob(&p);
        let q = path64_from_single_paths_blob(blob);
        assert_eq!(q.len(), p.len());
        assert_eq!(q.get_point(0), p.get_point(0));
        assert_eq!(q.get_point(2), p.get_point(2));
    }

    #[test]
    fn paths64_multi_blob_roundtrip() {
        let a = Path64::new(vec![Point64::new(0, 0), Point64::new(1, 0)]);
        let b = Path64::new(vec![
            Point64::new(5, 5),
            Point64::new(6, 5),
            Point64::new(6, 6),
        ]);
        let paths = Paths64::new(vec![a, b]);
        let blob = paths64_to_blob(&paths);
        let out = paths64_from_blob(blob);
        assert_eq!(out.len(), 2);
        assert_eq!(out.path(0).len(), 2);
        assert_eq!(out.path(1).len(), 3);
    }

    #[test]
    fn paths64_from_blob_short_starts_is_empty() {
        let b = PathsBlob64 {
            points: vec![],
            path_starts: vec![0],
        };
        assert!(paths64_from_blob(b).is_empty());
        let b2 = PathsBlob64 {
            points: vec![],
            path_starts: vec![],
        };
        assert!(paths64_from_blob(b2).is_empty());
    }

    #[test]
    fn path64_from_single_paths_blob_short_starts_is_default() {
        let b = PathsBlob64 {
            points: vec![],
            path_starts: vec![0],
        };
        assert!(path64_from_single_paths_blob(b).is_empty());
    }

    #[test]
    fn paths_blob64_iter_exact_len_and_fused() {
        let paths = Paths64::new(vec![
            Path64::new(vec![Point64::new(0, 0), Point64::new(1, 0)]),
            Path64::new(vec![Point64::new(2, 2)]),
        ]);
        let blob = paths64_to_blob(&paths);
        let mut it = PathsBlob64Iter::new(&blob);
        assert_eq!(it.len(), 2);
        assert_eq!(it.next().map(|p| p.len()), Some(2));
        assert_eq!(it.len(), 1);
        assert_eq!(it.next().map(|p| p.len()), Some(1));
        assert!(it.next().is_none());
        assert!(it.next().is_none());
    }

    #[test]
    fn pathd_single_and_paths_roundtrip() {
        let p = PathD::new(vec![
            PointD::new(0.0, 0.0),
            PointD::new(3.0, 0.0),
            PointD::new(3.0, 4.0),
        ]);
        let blob = pathd_to_blob(&p);
        let q = pathd_from_single_paths_blob(blob);
        assert_eq!(q.len(), 3);

        let paths = PathsD::new(vec![p]);
        let blob2 = pathsd_to_blob(&paths);
        let out = pathsd_from_blob(blob2);
        assert_eq!(out.len(), 1);
        assert_eq!(out.path(0).len(), 3);
    }
}
