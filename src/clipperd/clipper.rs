use std::fmt;

use crate::cxx_bridge::clipper2_sys_cxx;
use crate::paths_blob::{pathsd_to_blob, PathsBlobDIter};
use crate::poly_path::PolyCxxPreorderIterD;
use crate::{ClipType, FillRule, PathsD};

use super::LazyPathsD;

/// Result of [`ClipperD::execute`]. / [`ClipperD::execute`] 的返回值。
#[derive(Clone, Debug)]
pub struct ClipSolutionD {
    closed: clipper2_sys_cxx::PathsBlobD,
    open: clipper2_sys_cxx::PathsBlobD,
}

impl ClipSolutionD {
    /// No closed solution. / 无闭合解。
    #[inline]
    pub fn closed_is_empty(&self) -> bool {
        self.closed.path_starts.len() < 2
    }

    /// No open solution. / 无开放解。
    #[inline]
    pub fn open_is_empty(&self) -> bool {
        self.open.path_starts.len() < 2
    }

    /// Iterator over closed paths. / 闭合路径迭代器。
    #[inline]
    pub fn iter_closed(&self) -> PathsBlobDIter<'_> {
        PathsBlobDIter::new(&self.closed)
    }

    /// Iterator over open paths. / 开放路径迭代器。
    #[inline]
    pub fn iter_open(&self) -> PathsBlobDIter<'_> {
        PathsBlobDIter::new(&self.open)
    }

    /// Collect closed paths. / 收集闭合路径。
    pub fn to_closed(&self) -> PathsD {
        self.iter_closed().collect()
    }

    /// Collect open paths. / 收集开放路径。
    pub fn to_open(&self) -> PathsD {
        self.iter_open().collect()
    }

    /// Split into lazy holders. / 拆分为惰性包装。
    pub fn into_lazy(self) -> (LazyPathsD, LazyPathsD) {
        (
            LazyPathsD::from_blob(self.closed),
            LazyPathsD::from_blob(self.open),
        )
    }
}

/// Result of [`ClipperD::execute_tree`]. / [`ClipperD::execute_tree`] 的返回值。
pub struct ClipTreeSolutionD {
    poly_root: Option<usize>,
    open: clipper2_sys_cxx::PathsBlobD,
}

impl fmt::Debug for ClipTreeSolutionD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClipTreeSolutionD")
            .field("has_poly_root", &self.poly_root.map(|r| r != 0))
            .field("open_paths", &(self.open.path_starts.len().saturating_sub(1)))
            .finish()
    }
}

impl Drop for ClipTreeSolutionD {
    fn drop(&mut self) {
        if let Some(r) = self.poly_root.take() {
            if r != 0 {
                clipper2_sys_cxx::cxx_polyd_delete(r);
            }
        }
    }
}

impl ClipTreeSolutionD {
    /// `true` if a polygon-tree root is present. / 是否存在多边形树根。
    #[inline]
    pub fn has_poly_tree(&self) -> bool {
        matches!(self.poly_root, Some(r) if r != 0)
    }

    /// No open paths. / 无开放路径。
    #[inline]
    pub fn open_is_empty(&self) -> bool {
        self.open.path_starts.len() < 2
    }

    /// Open-path iterator. / 开放路径迭代器。
    #[inline]
    pub fn iter_open(&self) -> PathsBlobDIter<'_> {
        PathsBlobDIter::new(&self.open)
    }

    /// Collect open paths. / 收集开放路径。
    pub fn to_open(&self) -> PathsD {
        self.iter_open().collect()
    }

    /// Keep only open solution; free polygon tree. / 只要开放解并释放树。
    pub fn into_open_lazy(mut self) -> LazyPathsD {
        if let Some(r) = self.poly_root.take() {
            if r != 0 {
                clipper2_sys_cxx::cxx_polyd_delete(r);
            }
        }
        LazyPathsD::from_blob(std::mem::take(&mut self.open))
    }

    /// Lazy open paths + preorder over C++ `PolyPathD`. / 开放路径 + `PolyPathD` 前序迭代。
    pub fn into_open_and_poly_preorder(mut self) -> (LazyPathsD, PolyCxxPreorderIterD) {
        let root = self.poly_root.take().unwrap_or(0);
        let open = std::mem::take(&mut self.open);
        let iter = PolyCxxPreorderIterD::new(root);
        (LazyPathsD::from_blob(open), iter)
    }
}

/// Double-precision Clipper2 engine (`ClipperD`); `precision` controls decimal places.
///
/// 双精度布尔裁剪引擎；`precision` 为小数精度位数。
pub struct ClipperD {
    inner: cxx::UniquePtr<clipper2_sys_cxx::ClipperDBox>,
}

impl ClipperD {
    /// New clipper with decimal precision (e.g. `4`). / 指定小数精度构造。
    pub fn new(precision: i32) -> Self {
        Self {
            inner: clipper2_sys_cxx::cxx_clipperd_new(precision),
        }
    }

    /// Sets `PreserveCollinear`. / 设置保留共线点。
    pub fn set_preserve_collinear(&mut self, value: bool) {
        clipper2_sys_cxx::cxx_clipperd_set_preserve_collinear(self.inner.pin_mut(), value);
    }

    /// Gets `PreserveCollinear`. / 读取保留共线点。
    pub fn get_preserve_collinear(&self) -> bool {
        clipper2_sys_cxx::cxx_clipperd_get_preserve_collinear(&self.inner)
    }

    /// Sets `ReverseSolution`. / 设置反转解。
    pub fn set_reverse_solution(&mut self, value: bool) {
        clipper2_sys_cxx::cxx_clipperd_set_reverse_solution(self.inner.pin_mut(), value);
    }

    /// Gets `ReverseSolution`. / 读取反转解。
    pub fn get_reverse_solution(&self) -> bool {
        clipper2_sys_cxx::cxx_clipperd_get_reverse_solution(&self.inner)
    }

    /// Clears all paths. / 清空路径。
    pub fn clear(&mut self) {
        clipper2_sys_cxx::cxx_clipperd_clear(self.inner.pin_mut());
    }

    /// Adds open subjects. / 添加开放 subject。
    pub fn add_open_subject(&mut self, open_subject: &PathsD) {
        clipper2_sys_cxx::cxx_clipperd_add_open_subject(
            self.inner.pin_mut(),
            &pathsd_to_blob(open_subject),
        );
    }

    /// Adds closed subjects. / 添加闭合 subject。
    pub fn add_subject(&mut self, subject: &PathsD) {
        clipper2_sys_cxx::cxx_clipperd_add_subject(self.inner.pin_mut(), &pathsd_to_blob(subject));
    }

    /// Adds clip paths. / 添加 clip。
    pub fn add_clip(&mut self, clip: &PathsD) {
        clipper2_sys_cxx::cxx_clipperd_add_clip(self.inner.pin_mut(), &pathsd_to_blob(clip));
    }

    /// Planar clip; same idea as [`crate::Clipper64::execute`]. / 平面裁剪，语义同 [`crate::Clipper64::execute`]。
    pub fn execute(&mut self, clip_type: ClipType, fill_rule: FillRule) -> ClipSolutionD {
        let out = clipper2_sys_cxx::cxx_clipperd_execute(
            self.inner.pin_mut(),
            clip_type.into(),
            fill_rule.into(),
        );
        ClipSolutionD {
            closed: out.closed,
            open: out.open,
        }
    }

    /// Hierarchical clip. / 带层次结构的裁剪。
    pub fn execute_tree(&mut self, clip_type: ClipType, fill_rule: FillRule) -> ClipTreeSolutionD {
        let te = clipper2_sys_cxx::cxx_clipperd_execute_tree(
            self.inner.pin_mut(),
            clip_type.into(),
            fill_rule.into(),
        );
        ClipTreeSolutionD {
            poly_root: if te.root == 0 { None } else { Some(te.root) },
            open: te.open,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClipType, ClipperD, FillRule, PathD, PathsD, PointD};

    fn square(x0: f64, y0: f64, s: f64) -> PathD {
        PathD::new(vec![
            PointD::new(x0, y0),
            PointD::new(x0 + s, y0),
            PointD::new(x0 + s, y0 + s),
            PointD::new(x0, y0 + s),
        ])
    }

    #[test]
    fn union_overlapping_squares_non_empty() {
        let mut c = ClipperD::new(4);
        c.add_subject(&PathsD::new(vec![square(0.0, 0.0, 100.0)]));
        c.add_clip(&PathsD::new(vec![square(50.0, 50.0, 100.0)]));
        let sol = c.execute(ClipType::Union, FillRule::NonZero);
        assert!(!sol.closed_is_empty() || !sol.open_is_empty());
    }

    #[test]
    fn execute_tree_poly_preorder_d_count_stable() {
        let mut c = ClipperD::new(4);
        c.add_subject(&PathsD::new(vec![square(0.0, 0.0, 100.0)]));
        c.add_clip(&PathsD::new(vec![square(50.0, 50.0, 100.0)]));
        let sol = c.execute_tree(ClipType::Union, FillRule::NonZero);
        let (_, iter_a) = sol.into_open_and_poly_preorder();
        let n_a = iter_a.count();

        let mut c2 = ClipperD::new(4);
        c2.add_subject(&PathsD::new(vec![square(0.0, 0.0, 100.0)]));
        c2.add_clip(&PathsD::new(vec![square(50.0, 50.0, 100.0)]));
        let sol2 = c2.execute_tree(ClipType::Union, FillRule::NonZero);
        let (_, iter_b) = sol2.into_open_and_poly_preorder();
        let n_b = iter_b.count();

        assert_eq!(n_a, n_b);
    }

    #[test]
    fn execute_tree_union_d_has_poly_or_open() {
        let mut c = ClipperD::new(4);
        c.add_subject(&PathsD::new(vec![square(0.0, 0.0, 100.0)]));
        c.add_clip(&PathsD::new(vec![square(50.0, 50.0, 100.0)]));
        let sol = c.execute_tree(ClipType::Union, FillRule::NonZero);
        assert!(sol.has_poly_tree() || !sol.open_is_empty());
    }
}
