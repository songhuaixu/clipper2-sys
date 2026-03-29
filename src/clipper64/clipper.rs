use std::fmt;

use crate::cxx_bridge::clipper2_sys_cxx;
use crate::paths_blob::{paths64_to_blob, PathsBlob64Iter};
use crate::poly_path::PolyCxxPreorderIter64;
use crate::{ClipType, FillRule, Paths64};

use super::LazyPaths64;

/// Result of [`Clipper64::execute`]: closed and open solutions as C++ blobs.
///
/// 平面布尔 [`Clipper64::execute`] 的结果：闭合解与开放解，内部为 C++ 扁平淡点。
/// Iterate or call [`Self::into_lazy`] to build `Path64` values lazily.
/// 可迭代或 [`into_lazy`](Self::into_lazy) 惰性得到 `Path64`。
#[derive(Clone, Debug)]
pub struct ClipSolution64 {
    closed: clipper2_sys_cxx::PathsBlob64,
    open: clipper2_sys_cxx::PathsBlob64,
}

impl ClipSolution64 {
    /// `true` if there is no closed output. / 无闭合输出。
    #[inline]
    pub fn closed_is_empty(&self) -> bool {
        self.closed.path_starts.len() < 2
    }

    /// `true` if there is no open output. / 无开放输出。
    #[inline]
    pub fn open_is_empty(&self) -> bool {
        self.open.path_starts.len() < 2
    }

    /// Iterator over closed paths. / 闭合路径迭代器。
    #[inline]
    pub fn iter_closed(&self) -> PathsBlob64Iter<'_> {
        PathsBlob64Iter::new(&self.closed)
    }

    /// Iterator over open paths. / 开放路径迭代器。
    #[inline]
    pub fn iter_open(&self) -> PathsBlob64Iter<'_> {
        PathsBlob64Iter::new(&self.open)
    }

    /// Materializes all closed paths. / 物化全部闭合路径。
    pub fn to_closed(&self) -> Paths64 {
        self.iter_closed().collect()
    }

    /// Materializes all open paths. / 物化全部开放路径。
    pub fn to_open(&self) -> Paths64 {
        self.iter_open().collect()
    }

    /// Splits into two lazy wrappers (no extra copy of blobs). / 拆成两个惰性包装。
    pub fn into_lazy(self) -> (LazyPaths64, LazyPaths64) {
        (
            LazyPaths64::from_blob(self.closed),
            LazyPaths64::from_blob(self.open),
        )
    }
}

/// Result of [`Clipper64::execute_tree`]: optional `PolyPath64` root + open paths.
///
/// 树形 [`execute_tree`](Clipper64::execute_tree) 结果：可选的多边形树根指针与开放路径。
/// The polygon tree must be consumed via [`Self::into_open_and_poly_preorder`] or dropped;
/// use [`Self::into_open_lazy`] to discard the tree.
/// 多边形树须通过前序迭代消费或析构；若只要开放解可用 [`into_open_lazy`](Self::into_open_lazy)。
pub struct ClipTreeSolution64 {
    poly_root: Option<usize>,
    open: clipper2_sys_cxx::PathsBlob64,
}

impl fmt::Debug for ClipTreeSolution64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClipTreeSolution64")
            .field("has_poly_root", &self.poly_root.map(|r| r != 0))
            .field("open_paths", &(self.open.path_starts.len().saturating_sub(1)))
            .finish()
    }
}

impl Drop for ClipTreeSolution64 {
    fn drop(&mut self) {
        if let Some(r) = self.poly_root.take() {
            if r != 0 {
                clipper2_sys_cxx::cxx_poly64_delete(r);
            }
        }
    }
}

impl ClipTreeSolution64 {
    /// `true` if a non-null polygon tree root exists. / 是否存在非空多边形树根。
    #[inline]
    pub fn has_poly_tree(&self) -> bool {
        matches!(self.poly_root, Some(r) if r != 0)
    }

    /// No open paths when `true`. / 无开放路径。
    #[inline]
    pub fn open_is_empty(&self) -> bool {
        self.open.path_starts.len() < 2
    }

    /// Open-path iterator. / 开放路径迭代器。
    #[inline]
    pub fn iter_open(&self) -> PathsBlob64Iter<'_> {
        PathsBlob64Iter::new(&self.open)
    }

    /// Collects open paths. / 收集开放路径。
    pub fn to_open(&self) -> Paths64 {
        self.iter_open().collect()
    }

    /// Frees the polygon tree and returns lazy open paths only. / 释放树，仅返回开放路径惰性包装。
    pub fn into_open_lazy(mut self) -> LazyPaths64 {
        if let Some(r) = self.poly_root.take() {
            if r != 0 {
                clipper2_sys_cxx::cxx_poly64_delete(r);
            }
        }
        LazyPaths64::from_blob(std::mem::take(&mut self.open))
    }

    /// Lazy open paths + preorder iterator over the C++ `PolyPath64` tree (frees tree on drop).
    ///
    /// 开放路径惰性包装 + C++ `PolyPath64` 前序迭代器（迭代器 `Drop` 时释放整棵子树）。
    pub fn into_open_and_poly_preorder(mut self) -> (LazyPaths64, PolyCxxPreorderIter64) {
        let root = self.poly_root.take().unwrap_or(0);
        let open = std::mem::take(&mut self.open);
        let iter = PolyCxxPreorderIter64::new(root);
        (LazyPaths64::from_blob(open), iter)
    }
}

/// Integer-coordinate Clipper2 boolean engine (`Clipper64`).
///
/// Clipper2 整数布尔裁剪引擎。
pub struct Clipper64 {
    inner: cxx::UniquePtr<clipper2_sys_cxx::Clipper64Box>,
}

impl Clipper64 {
    /// Creates an empty clipper. / 创建空裁剪器。
    pub fn new() -> Self {
        Self {
            inner: clipper2_sys_cxx::cxx_clipper64_new(),
        }
    }

    /// Sets `PreserveCollinear`. / 设置保留共线点。
    pub fn set_preserve_collinear(&mut self, value: bool) {
        clipper2_sys_cxx::cxx_clipper64_set_preserve_collinear(self.inner.pin_mut(), value);
    }

    /// Gets `PreserveCollinear`. / 读取保留共线点。
    pub fn get_preserve_collinear(&self) -> bool {
        clipper2_sys_cxx::cxx_clipper64_get_preserve_collinear(&self.inner)
    }

    /// Sets `ReverseSolution`. / 设置反转解方向。
    pub fn set_reverse_solution(&mut self, value: bool) {
        clipper2_sys_cxx::cxx_clipper64_set_reverse_solution(self.inner.pin_mut(), value);
    }

    /// Gets `ReverseSolution`. / 读取反转解。
    pub fn get_reverse_solution(&self) -> bool {
        clipper2_sys_cxx::cxx_clipper64_get_reverse_solution(&self.inner)
    }

    /// Clears all subjects and clips. / 清空所有 subject 与 clip。
    pub fn clear(&mut self) {
        clipper2_sys_cxx::cxx_clipper64_clear(self.inner.pin_mut());
    }

    /// Adds open subject paths. / 添加开放 subject。
    pub fn add_open_subject(&mut self, open_subject: &Paths64) {
        clipper2_sys_cxx::cxx_clipper64_add_open_subject(
            self.inner.pin_mut(),
            &paths64_to_blob(open_subject),
        );
    }

    /// Adds closed subject paths. / 添加闭合 subject。
    pub fn add_subject(&mut self, subject: &Paths64) {
        clipper2_sys_cxx::cxx_clipper64_add_subject(self.inner.pin_mut(), &paths64_to_blob(subject));
    }

    /// Adds clip paths. / 添加裁剪区域。
    pub fn add_clip(&mut self, clip: &Paths64) {
        clipper2_sys_cxx::cxx_clipper64_add_clip(self.inner.pin_mut(), &paths64_to_blob(clip));
    }

    /// Runs planar clipping, returning closed/open paths.
    ///
    /// 执行平面裁剪，返回闭合/开放路径解。
    pub fn execute(&mut self, clip_type: ClipType, fill_rule: FillRule) -> ClipSolution64 {
        let out = clipper2_sys_cxx::cxx_clipper64_execute(
            self.inner.pin_mut(),
            clip_type.into(),
            fill_rule.into(),
        );
        ClipSolution64 {
            closed: out.closed,
            open: out.open,
        }
    }

    /// Runs clipping with hierarchy (`PolyTree`) on the closed side.
    ///
    /// 执行裁剪并在闭合侧保留层次（`PolyTree`），开放侧仍为扁平淡点。
    pub fn execute_tree(&mut self, clip_type: ClipType, fill_rule: FillRule) -> ClipTreeSolution64 {
        let te = clipper2_sys_cxx::cxx_clipper64_execute_tree(
            self.inner.pin_mut(),
            clip_type.into(),
            fill_rule.into(),
        );
        ClipTreeSolution64 {
            poly_root: if te.root == 0 { None } else { Some(te.root) },
            open: te.open,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClipType, Clipper64, FillRule, Path64, Paths64, Point64};

    fn square(x0: i64, y0: i64, s: i64) -> Path64 {
        Path64::new(vec![
            Point64::new(x0, y0),
            Point64::new(x0 + s, y0),
            Point64::new(x0 + s, y0 + s),
            Point64::new(x0, y0 + s),
        ])
    }

    #[test]
    fn union_overlapping_squares_non_empty() {
        let mut c = Clipper64::new();
        c.add_subject(&Paths64::new(vec![square(0, 0, 100)]));
        c.add_clip(&Paths64::new(vec![square(50, 50, 100)]));
        let sol = c.execute(ClipType::Union, FillRule::NonZero);
        assert!(!sol.closed_is_empty() || !sol.open_is_empty());
    }

    #[test]
    fn execute_iter_closed_collect_matches_into_lazy() {
        let mut c = Clipper64::new();
        c.add_subject(&Paths64::new(vec![square(0, 0, 100)]));
        c.add_clip(&Paths64::new(vec![square(50, 50, 100)]));
        let sol = c.execute(ClipType::Union, FillRule::NonZero);
        let a: Paths64 = sol.iter_closed().chain(sol.iter_open()).collect();
        let (closed, open) = sol.into_lazy();
        let mut b = closed.into_paths();
        b.0.extend(open.into_paths().0);
        assert_eq!(a.len(), b.len());
    }

    #[test]
    fn execute_tree_poly_preorder_count_stable() {
        let mut c = Clipper64::new();
        c.add_subject(&Paths64::new(vec![square(0, 0, 100)]));
        c.add_clip(&Paths64::new(vec![square(50, 50, 100)]));
        let sol = c.execute_tree(ClipType::Union, FillRule::NonZero);
        let (_, iter_a) = sol.into_open_and_poly_preorder();
        let n_a = iter_a.count();

        let mut c2 = Clipper64::new();
        c2.add_subject(&Paths64::new(vec![square(0, 0, 100)]));
        c2.add_clip(&Paths64::new(vec![square(50, 50, 100)]));
        let sol2 = c2.execute_tree(ClipType::Union, FillRule::NonZero);
        let (_, iter_b) = sol2.into_open_and_poly_preorder();
        let n_b = iter_b.count();

        assert_eq!(n_a, n_b);
    }

    #[test]
    fn execute_tree_union_has_poly_or_open() {
        let mut c = Clipper64::new();
        c.add_subject(&Paths64::new(vec![square(0, 0, 100)]));
        c.add_clip(&Paths64::new(vec![square(50, 50, 100)]));
        let sol = c.execute_tree(ClipType::Union, FillRule::NonZero);
        assert!(sol.has_poly_tree() || !sol.open_is_empty());
    }

    #[test]
    fn intersection_overlapping_squares_non_empty() {
        let mut c = Clipper64::new();
        c.add_subject(&Paths64::new(vec![square(0, 0, 100)]));
        c.add_clip(&Paths64::new(vec![square(50, 50, 100)]));
        let sol = c.execute(ClipType::Intersection, FillRule::NonZero);
        assert!(!sol.closed_is_empty() || !sol.open_is_empty());
    }

    #[test]
    fn difference_frame_non_empty() {
        let mut c = Clipper64::new();
        c.add_subject(&Paths64::new(vec![square(0, 0, 100)]));
        c.add_clip(&Paths64::new(vec![square(25, 25, 50)]));
        let sol = c.execute(ClipType::Difference, FillRule::NonZero);
        assert!(!sol.closed_is_empty() || !sol.open_is_empty());
    }

    #[test]
    fn xor_overlapping_squares_non_empty() {
        let mut c = Clipper64::new();
        c.add_subject(&Paths64::new(vec![square(0, 0, 100)]));
        c.add_clip(&Paths64::new(vec![square(50, 50, 100)]));
        let sol = c.execute(ClipType::Xor, FillRule::NonZero);
        assert!(!sol.closed_is_empty() || !sol.open_is_empty());
    }

    #[test]
    fn union_even_odd_fill_non_empty() {
        let mut c = Clipper64::new();
        c.add_subject(&Paths64::new(vec![square(0, 0, 100)]));
        c.add_clip(&Paths64::new(vec![square(50, 50, 100)]));
        let sol = c.execute(ClipType::Union, FillRule::EvenOdd);
        assert!(!sol.closed_is_empty() || !sol.open_is_empty());
    }

    #[test]
    fn open_subject_line_intersection_with_clip_non_empty() {
        let mut c = Clipper64::new();
        let line = Path64::new(vec![
            Point64::new(-10, 50),
            Point64::new(110, 50),
        ]);
        c.add_open_subject(&Paths64::new(vec![line]));
        c.add_clip(&Paths64::new(vec![square(0, 0, 100)]));
        let sol = c.execute(ClipType::Intersection, FillRule::NonZero);
        assert!(!sol.open_is_empty() || !sol.closed_is_empty());
    }
}
