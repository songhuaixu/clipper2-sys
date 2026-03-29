//! C++/Rust boundary: [`cxx::bridge`] shared types and `extern "C++"` into Clipper2Lib.
//!
//! C++/Rust 边界：`cxx::bridge` 共享类型与直连 Clipper2Lib 的 `extern "C++"`。
//!
//! Paths are stored flat as [`PathsBlob64`] / [`PathsBlobD`]: `path_starts.len() == path_count + 1`,
//! path `i` is `points[path_starts[i]..path_starts[i+1]]`.
//!
//! 路径用扁平淡点表编码：`path_starts.len() == 路径条数 + 1`，第 `i` 条为 `points[path_starts[i]..path_starts[i+1]]`。

#[cxx::bridge]
pub(crate) mod clipper2_sys_cxx {
    /// Grid point for integer Clipper2 math; same layout as C++ `Point64`.
    ///
    /// 整数网格点，与 C++ `Point64` 布局一致。
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct P64 {
        /// X in user integer units. / X。
        pub x: i64,
        /// Y in user integer units. / Y。
        pub y: i64,
    }

    /// Double-precision point; same layout as C++ `PointD`.
    ///
    /// 双精度点，与 C++ `PointD` 一致。
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct PD {
        pub x: f64,
        pub y: f64,
    }

    /// Flat multi-path buffer (64-bit coordinates).
    ///
    /// 扁平淡点多路径缓冲（64 位坐标）。
    #[derive(Clone, Debug, Default)]
    pub struct PathsBlob64 {
        /// All points concatenated. / 全部顶点顺接。
        pub points: Vec<P64>,
        /// Prefix sums into `points`; length = paths + 1. / 各路径在 `points` 中的起止前缀，长度 = 路径数+1。
        pub path_starts: Vec<usize>,
    }

    /// Flat multi-path buffer (`f64` coordinates).
    ///
    /// 扁平淡点缓冲（`f64` 坐标）。
    #[derive(Clone, Debug, Default)]
    pub struct PathsBlobD {
        pub points: Vec<PD>,
        pub path_starts: Vec<usize>,
    }

    /// Closed + open blobs from `Clipper64::Execute`.
    ///
    /// `Clipper64::Execute` 的闭合/开放结果。
    pub struct Exec64 {
        pub closed: PathsBlob64,
        pub open: PathsBlob64,
    }

    /// Closed + open blobs from `ClipperD::Execute`. / `ClipperD::Execute` 结果。
    pub struct ExecD {
        pub closed: PathsBlobD,
        pub open: PathsBlobD,
    }

    /// Tree execute: opaque `PolyPath` root + open blob (`Clipper64`).
    ///
    /// 树形执行：不透明 `PolyPath` 根 + 开放路径（`Clipper64`）。
    pub struct TreeExec64 {
        /// Opaque C++ pointer as `usize`. / C++ 根指针数值。
        pub root: usize,
        pub open: PathsBlob64,
    }

    /// Tree execute for `ClipperD`. / `ClipperD` 树形执行。
    pub struct TreeExecD {
        pub root: usize,
        pub open: PathsBlobD,
    }

    /// Clipper2 `ClipType` discrimination (`NoClip` = 0 … `Xor` = 4).
    ///
    /// 与 `Clipper2Lib::ClipType` 数值一致。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum ClipperClipType {
        NoClip = 0,
        Intersection = 1,
        Union = 2,
        Difference = 3,
        Xor = 4,
    }

    /// Clipper2 `FillRule` values. / `Clipper2Lib::FillRule` 取值。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum ClipperFillRule {
        EvenOdd = 0,
        NonZero = 1,
        Positive = 2,
        Negative = 3,
    }

    /// Join type indices as used by this bridge (`join_from_u32`). / 本桥 C++ 侧约定的连接类型编号。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum ClipperJoinType {
        Square = 0,
        Round = 1,
        Miter = 2,
    }

    /// Clipper2 `EndType` (`Polygon` = 0 … `Round` = 4). / 与 `EndType` 一致。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum ClipperEndType {
        Polygon = 0,
        Joined = 1,
        Butt = 2,
        Square = 3,
        Round = 4,
    }

    /// Raw `PointInPolygon` result from Clipper2. / `PointInPolygon` 原始返回值。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u32)]
    pub enum ClipperPointInPolygonResult {
        IsOn = 0,
        IsInside = 1,
        IsOutside = 2,
    }

    // Each `fn` mirrors `cpp/clipper2_sys_bridge.hpp` / 下列函数与 `cpp/clipper2_sys_bridge.hpp` 一一对应。
    unsafe extern "C++" {
        include!("cpp/clipper2_sys_bridge.hpp");

        /// 编译与链接探针；不应为 0。
        fn clipper2_sys_cxx_link_marker() -> u32;

        /// 与 Clipper2 C++ 运行时一致的堆分配（`std::malloc`）。
        fn clipper2_sys_malloc(n: usize) -> usize;
        fn clipper2_sys_free(p: usize);

        type Clipper64Box;
        fn cxx_clipper64_new() -> UniquePtr<Clipper64Box>;
        fn cxx_clipper64_set_preserve_collinear(c: Pin<&mut Clipper64Box>, v: bool);
        fn cxx_clipper64_get_preserve_collinear(c: &Clipper64Box) -> bool;
        fn cxx_clipper64_set_reverse_solution(c: Pin<&mut Clipper64Box>, v: bool);
        fn cxx_clipper64_get_reverse_solution(c: &Clipper64Box) -> bool;
        fn cxx_clipper64_clear(c: Pin<&mut Clipper64Box>);
        fn cxx_clipper64_add_subject(c: Pin<&mut Clipper64Box>, paths: &PathsBlob64);
        fn cxx_clipper64_add_open_subject(c: Pin<&mut Clipper64Box>, paths: &PathsBlob64);
        fn cxx_clipper64_add_clip(c: Pin<&mut Clipper64Box>, paths: &PathsBlob64);
        fn cxx_clipper64_execute(c: Pin<&mut Clipper64Box>, ct: ClipperClipType, fr: ClipperFillRule) -> Exec64;
        fn cxx_clipper64_execute_tree(
            c: Pin<&mut Clipper64Box>,
            ct: ClipperClipType,
            fr: ClipperFillRule,
        ) -> TreeExec64;

        type ClipperDBox;
        fn cxx_clipperd_new(precision: i32) -> UniquePtr<ClipperDBox>;
        fn cxx_clipperd_set_preserve_collinear(c: Pin<&mut ClipperDBox>, v: bool);
        fn cxx_clipperd_get_preserve_collinear(c: &ClipperDBox) -> bool;
        fn cxx_clipperd_set_reverse_solution(c: Pin<&mut ClipperDBox>, v: bool);
        fn cxx_clipperd_get_reverse_solution(c: &ClipperDBox) -> bool;
        fn cxx_clipperd_clear(c: Pin<&mut ClipperDBox>);
        fn cxx_clipperd_add_subject(c: Pin<&mut ClipperDBox>, paths: &PathsBlobD);
        fn cxx_clipperd_add_open_subject(c: Pin<&mut ClipperDBox>, paths: &PathsBlobD);
        fn cxx_clipperd_add_clip(c: Pin<&mut ClipperDBox>, paths: &PathsBlobD);
        fn cxx_clipperd_execute(c: Pin<&mut ClipperDBox>, ct: ClipperClipType, fr: ClipperFillRule) -> ExecD;
        fn cxx_clipperd_execute_tree(
            c: Pin<&mut ClipperDBox>,
            ct: ClipperClipType,
            fr: ClipperFillRule,
        ) -> TreeExecD;

        type ClipperOffsetBox;
        fn cxx_clipper_offset_new(
            miter_limit: f64,
            arc_tolerance: f64,
            preserve_collinear: bool,
            reverse_solution: bool,
        ) -> UniquePtr<ClipperOffsetBox>;
        fn cxx_clipper_offset_set_miter_limit(c: Pin<&mut ClipperOffsetBox>, v: f64);
        fn cxx_clipper_offset_get_miter_limit(c: &ClipperOffsetBox) -> f64;
        fn cxx_clipper_offset_set_arc_tolerance(c: Pin<&mut ClipperOffsetBox>, v: f64);
        fn cxx_clipper_offset_get_arc_tolerance(c: &ClipperOffsetBox) -> f64;
        fn cxx_clipper_offset_set_preserve_collinear(c: Pin<&mut ClipperOffsetBox>, v: bool);
        fn cxx_clipper_offset_get_preserve_collinear(c: &ClipperOffsetBox) -> bool;
        fn cxx_clipper_offset_set_reverse_solution(c: Pin<&mut ClipperOffsetBox>, v: bool);
        fn cxx_clipper_offset_get_reverse_solution(c: &ClipperOffsetBox) -> bool;
        fn cxx_clipper_offset_error_code(c: &ClipperOffsetBox) -> i32;
        fn cxx_clipper_offset_clear(c: Pin<&mut ClipperOffsetBox>);
        fn cxx_clipper_offset_add_path(
            c: Pin<&mut ClipperOffsetBox>,
            path: &PathsBlob64,
            join_type: ClipperJoinType,
            end_type: ClipperEndType,
        );
        fn cxx_clipper_offset_add_paths(
            c: Pin<&mut ClipperOffsetBox>,
            paths: &PathsBlob64,
            join_type: ClipperJoinType,
            end_type: ClipperEndType,
        );
        fn cxx_clipper_offset_execute(c: Pin<&mut ClipperOffsetBox>, delta: f64) -> PathsBlob64;

        fn cxx_path64_simplify(blob: &PathsBlob64, epsilon: f64, is_open: bool) -> PathsBlob64;
        fn cxx_paths64_simplify(blob: &PathsBlob64, epsilon: f64, is_open: bool) -> PathsBlob64;
        fn cxx_pathd_simplify(blob: &PathsBlobD, epsilon: f64, is_open: bool) -> PathsBlobD;
        fn cxx_pathsd_simplify(blob: &PathsBlobD, epsilon: f64, is_open: bool) -> PathsBlobD;

        fn cxx_path64_to_pathd(blob: &PathsBlob64) -> PathsBlobD;
        fn cxx_pathd_to_path64(blob: &PathsBlobD, precision: i32) -> PathsBlob64;
        fn cxx_paths64_to_pathsd(blob: &PathsBlob64) -> PathsBlobD;
        fn cxx_pathsd_to_paths64(blob: &PathsBlobD, precision: i32) -> PathsBlob64;

        fn cxx_point_in_path64(blob: &PathsBlob64, x: i64, y: i64) -> ClipperPointInPolygonResult;
        fn cxx_point_in_pathd(blob: &PathsBlobD, x: f64, y: f64) -> ClipperPointInPolygonResult;

        fn cxx_path64_area(blob: &PathsBlob64) -> f64;
        fn cxx_paths64_area(blob: &PathsBlob64) -> f64;
        fn cxx_pathd_area(blob: &PathsBlobD) -> f64;
        fn cxx_pathsd_area(blob: &PathsBlobD) -> f64;

        fn cxx_path64_minkowski_sum(a: &PathsBlob64, b: &PathsBlob64, closed: bool) -> PathsBlob64;
        fn cxx_path64_minkowski_diff(a: &PathsBlob64, b: &PathsBlob64, closed: bool) -> PathsBlob64;
        fn cxx_paths64_minkowski_sum(
            pattern: &PathsBlob64,
            paths: &PathsBlob64,
            closed: bool,
            fr: ClipperFillRule,
        ) -> PathsBlob64;
        fn cxx_paths64_minkowski_diff(
            pattern: &PathsBlob64,
            paths: &PathsBlob64,
            closed: bool,
            fr: ClipperFillRule,
        ) -> PathsBlob64;

        fn cxx_pathd_minkowski_sum(
            a: &PathsBlobD,
            b: &PathsBlobD,
            closed: bool,
            precision: i32,
        ) -> PathsBlobD;
        fn cxx_pathd_minkowski_diff(
            a: &PathsBlobD,
            b: &PathsBlobD,
            closed: bool,
            precision: i32,
        ) -> PathsBlobD;
        fn cxx_pathsd_minkowski_sum(
            pattern: &PathsBlobD,
            paths: &PathsBlobD,
            closed: bool,
            precision: i32,
            fr: ClipperFillRule,
        ) -> PathsBlobD;
        fn cxx_pathsd_minkowski_diff(
            pattern: &PathsBlobD,
            paths: &PathsBlobD,
            closed: bool,
            precision: i32,
            fr: ClipperFillRule,
        ) -> PathsBlobD;

        fn cxx_paths64_inflate(
            paths: &PathsBlob64,
            delta: f64,
            join_type: ClipperJoinType,
            end_type: ClipperEndType,
            miter_limit: f64,
        ) -> PathsBlob64;
        fn cxx_pathsd_inflate(
            paths: &PathsBlobD,
            delta: f64,
            join_type: ClipperJoinType,
            end_type: ClipperEndType,
            miter_limit: f64,
            precision: i32,
        ) -> PathsBlobD;

        /// `PolyPath64` 指针（由 `execute_tree` 返回）；用 [`cxx_poly64_delete`] 释放整棵子树。
        fn cxx_poly64_is_hole(p: usize) -> bool;
        /// 单条轮廓路径的扁平编码（`path_starts.len() == 2`）。
        fn cxx_poly64_polygon(p: usize) -> PathsBlob64;
        fn cxx_poly64_child_count(p: usize) -> usize;
        fn cxx_poly64_child_at(p: usize, i: usize) -> usize;
        fn cxx_poly64_delete(p: usize);

        fn cxx_polyd_scale(p: usize) -> f64;
        fn cxx_polyd_is_hole(p: usize) -> bool;
        fn cxx_polyd_polygon(p: usize) -> PathsBlobD;
        fn cxx_polyd_child_count(p: usize) -> usize;
        fn cxx_polyd_child_at(p: usize, i: usize) -> usize;
        fn cxx_polyd_delete(p: usize);
    }
}

/// Allocates `size` bytes using the same allocator as Clipper2 C++ (`malloc`).
///
/// 使用与 C++ 侧一致的分配器分配（`malloc`）。
pub(crate) unsafe fn cxx_malloc(size: usize) -> *mut std::os::raw::c_void {
    clipper2_sys_cxx::clipper2_sys_malloc(size) as *mut std::os::raw::c_void
}

/// Frees memory allocated by [`cxx_malloc`]. / 释放 [`cxx_malloc`] 分配的内存。
pub(crate) unsafe fn cxx_free(p: *mut std::os::raw::c_void) {
    clipper2_sys_cxx::clipper2_sys_free(p as usize);
}

#[cfg(test)]
mod tests {
    #[test]
    fn cxx_link_marker_nonzero() {
        assert_ne!(super::clipper2_sys_cxx::clipper2_sys_cxx_link_marker(), 0);
    }

    #[test]
    fn cxx_malloc_roundtrip() {
        unsafe {
            let p = super::cxx_malloc(32);
            assert!(!p.is_null());
            super::cxx_free(p);
        }
    }

    #[test]
    fn cxx_path64_area_smoke() {
        use super::clipper2_sys_cxx::{cxx_path64_area, PathsBlob64, P64};
        let b = PathsBlob64 {
            points: vec![
                P64 { x: 0, y: 0 },
                P64 { x: 10, y: 0 },
                P64 { x: 10, y: 10 },
                P64 { x: 0, y: 10 },
            ],
            path_starts: vec![0, 4],
        };
        assert!((cxx_path64_area(&b).abs() - 100.0).abs() < 1e-6);
    }
}
