//! Shared `macro_rules!` for path geometry, blob iteration, lazy path holders, and poly-tree preorder.
//!
//! 整型 / 双精度路径几何、`PathsBlob*` 迭代、惰性路径包装、多边形树前序等的内部宏（`#[macro_use]` 加载）。

/// Lazy iterator: one [`Path*`] per segment `[path_starts[i], path_starts[i+1])`.
///
/// 惰性迭代：`path_starts` 与 `points` 布局与 C++ 一致，每步产出一条路径。
macro_rules! define_paths_blob_iter {
    (
        $(#[$iter_meta:meta])*
        $vis:vis $Iter:ident,
        point = $Pt:ty,
        path = $Path:ty,
        blob = $Blob:ty,
        conv_slice = $conv:path,
    ) => {
        $(#[$iter_meta])*
        #[derive(Clone, Copy)]
        $vis struct $Iter<'a> {
            points: &'a [$Pt],
            path_starts: &'a [usize],
            idx: usize,
        }

        impl<'a> $Iter<'a> {
            /// Wraps a borrowed blob. / 借用 blob 构造迭代器。
            pub(crate) fn new(blob: &'a $Blob) -> Self {
                Self {
                    points: &blob.points,
                    path_starts: &blob.path_starts,
                    idx: 0,
                }
            }
        }

        impl<'a> Iterator for $Iter<'a> {
            type Item = $Path;

            fn next(&mut self) -> Option<Self::Item> {
                let i = self.idx;
                if i + 1 >= self.path_starts.len() {
                    return None;
                }
                let a = self.path_starts[i];
                let e = self.path_starts[i + 1];
                self.idx += 1;
                Some($conv(&self.points[a..e]))
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let total = self.path_starts.len().saturating_sub(1);
                let left = total.saturating_sub(self.idx);
                (left, Some(left))
            }
        }

        impl<'a> ExactSizeIterator for $Iter<'a> {}

        impl<'a> std::iter::FusedIterator for $Iter<'a> {}
    };
}

/// Expands a lazy holder around a `PathsBlob*` plus `IntoIterator`.
///
/// 在 `PathsBlob*` 外封一层惰性包装并实现 `IntoIterator`。
macro_rules! define_lazy_paths {
    (
        $(#[$lazy_meta:meta])*
        $Lazy:ident,
        blob = $Blob:ty,
        paths = $Paths:ty,
        path = $Path:ty,
        iter = $Iter:ident,
        from_blob = $from_blob:path,
    ) => {
        $(#[$lazy_meta])*
        #[derive(Clone, Debug, Default)]
        pub struct $Lazy {
            blob: $Blob,
        }

        impl $Lazy {
            /// Wraps an owned blob (crate-internal). / 拥有 blob（crate 内部构造）。
            pub(crate) fn from_blob(blob: $Blob) -> Self {
                Self { blob }
            }

            /// Borrowing iterator over paths. / 借用迭代每条路径。
            #[inline]
            pub fn iter(&self) -> $Iter<'_> {
                $Iter::new(&self.blob)
            }

            /// True if there is no path segment. / 无任何路径分段。
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.blob.path_starts.len() < 2
            }

            /// Consumes and materializes all paths. / 消耗并物化全部路径。
            pub fn into_paths(self) -> $Paths {
                $from_blob(self.blob)
            }

            /// Clones-by-rebuilding all paths. / 通过迭代收集副本。
            pub fn to_paths(&self) -> $Paths {
                self.iter().collect()
            }

            /// First path or empty (common for single-path ops). / 取首条路径（单路径运算常用）。
            pub fn into_first_path(self) -> $Path {
                self.iter().next().unwrap_or_default()
            }
        }

        impl<'a> IntoIterator for &'a $Lazy {
            type Item = $Path;
            type IntoIter = $Iter<'a>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }
    };
}

macro_rules! define_poly_cxx_preorder_64 {
    () => {
        /// One node while traversing a C++ `PolyPath64` in preorder.
        ///
        /// 前序遍历 C++ `PolyPath64` 时的一个结点。
        #[derive(Clone, Debug)]
        pub struct PolyCxxNode64 {
            /// Depth from tree root (0 = root). / 相对根的深度。
            pub depth: usize,
            /// Hole flag reported by Clipper. / Clipper 报告的洞标志。
            pub is_hole: bool,
            /// Single contour for this node. / 该结点对应单条轮廓。
            pub path: crate::Path64,
        }

        /// Owns a native root; yields [`PolyCxxNode64`] in preorder; frees via `cxx_poly64_delete` on drop.
        ///
        /// 拥有 C++ 根指针，前序产出结点；`Drop` 时 `cxx_poly64_delete`。
        pub struct PolyCxxPreorderIter64 {
            stack: Vec<(usize, usize)>,
            root_to_free: Option<usize>,
        }

        impl PolyCxxPreorderIter64 {
            /// `root` is opaque C++ pointer as `usize`; `0` means empty. / `root` 为 C++ 指针数值，`0` 表示空。
            pub(crate) fn new(root: usize) -> Self {
                if root == 0 {
                    return Self {
                        stack: Vec::new(),
                        root_to_free: None,
                    };
                }
                Self {
                    stack: vec![(root, 0)],
                    root_to_free: Some(root),
                }
            }
        }

        impl Iterator for PolyCxxPreorderIter64 {
            type Item = PolyCxxNode64;

            fn next(&mut self) -> Option<Self::Item> {
                use crate::paths_blob::path64_from_single_paths_blob;
                let (n, depth) = self.stack.pop()?;
                let is_hole = crate::cxx_bridge::clipper2_sys_cxx::cxx_poly64_is_hole(n);
                let blob = crate::cxx_bridge::clipper2_sys_cxx::cxx_poly64_polygon(n);
                let path = path64_from_single_paths_blob(blob);
                let nc = crate::cxx_bridge::clipper2_sys_cxx::cxx_poly64_child_count(n) as usize;
                for i in (0..nc).rev() {
                    let c = crate::cxx_bridge::clipper2_sys_cxx::cxx_poly64_child_at(n, i);
                    self.stack.push((c, depth + 1));
                }
                Some(PolyCxxNode64 {
                    depth,
                    is_hole,
                    path,
                })
            }
        }

        impl Drop for PolyCxxPreorderIter64 {
            fn drop(&mut self) {
                if let Some(r) = self.root_to_free.take() {
                    if r != 0 {
                        crate::cxx_bridge::clipper2_sys_cxx::cxx_poly64_delete(r);
                    }
                }
            }
        }
    };
}

macro_rules! define_poly_cxx_preorder_d {
    () => {
        /// Node during preorder on `PolyPathD`. / `PolyPathD` 前序结点。
        #[derive(Clone, Debug)]
        pub struct PolyCxxNodeD {
            /// Depth from root. / 深度。
            pub depth: usize,
            /// Hole flag. / 是否洞。
            pub is_hole: bool,
            /// Scale from `PolyPathD`. / `PolyPathD` 的 scale。
            pub scale: f64,
            /// Contour geometry. / 轮廓。
            pub path: crate::PathD,
        }

        /// Preorder iterator with `cxx_polyd_delete` on drop. / 前序迭代，`Drop` 时 `cxx_polyd_delete`。
        pub struct PolyCxxPreorderIterD {
            stack: Vec<(usize, usize)>,
            root_to_free: Option<usize>,
        }

        impl PolyCxxPreorderIterD {
            /// See [`PolyCxxPreorderIter64::new`]. / 同 64 版 `new` 语义。
            pub(crate) fn new(root: usize) -> Self {
                if root == 0 {
                    return Self {
                        stack: Vec::new(),
                        root_to_free: None,
                    };
                }
                Self {
                    stack: vec![(root, 0)],
                    root_to_free: Some(root),
                }
            }
        }

        impl Iterator for PolyCxxPreorderIterD {
            type Item = PolyCxxNodeD;

            fn next(&mut self) -> Option<Self::Item> {
                use crate::paths_blob::pathd_from_single_paths_blob;
                let (n, depth) = self.stack.pop()?;
                let is_hole = crate::cxx_bridge::clipper2_sys_cxx::cxx_polyd_is_hole(n);
                let scale = crate::cxx_bridge::clipper2_sys_cxx::cxx_polyd_scale(n);
                let blob = crate::cxx_bridge::clipper2_sys_cxx::cxx_polyd_polygon(n);
                let path = pathd_from_single_paths_blob(blob);
                let nc = crate::cxx_bridge::clipper2_sys_cxx::cxx_polyd_child_count(n) as usize;
                for i in (0..nc).rev() {
                    let c = crate::cxx_bridge::clipper2_sys_cxx::cxx_polyd_child_at(n, i);
                    self.stack.push((c, depth + 1));
                }
                Some(PolyCxxNodeD {
                    depth,
                    is_hole,
                    scale,
                    path,
                })
            }
        }

        impl Drop for PolyCxxPreorderIterD {
            fn drop(&mut self) {
                if let Some(r) = self.root_to_free.take() {
                    if r != 0 {
                        crate::cxx_bridge::clipper2_sys_cxx::cxx_polyd_delete(r);
                    }
                }
            }
        }
    };
}

/// Implements geometry helpers on `Path64` or `PathD` (`translate` / `scale` differ by coordinate type).
///
/// 为 `Path64` 或 `PathD` 生成几何方法（平移 / 缩放随坐标类型不同）。
macro_rules! impl_path_geom {
    (int64, $Path:ident, $Point:ident) => {
        impl $Path {
            /// From owned `Vec` of points. / 由点向量构造。
            #[inline]
            pub fn from_vec(points: Vec<$Point>) -> Self {
                Self(points)
            }

            /// From anything that becomes `Vec` (e.g. `vec![…]`). / 由 `Into<Vec>` 构造。
            #[inline]
            pub fn new(points: impl Into<Vec<$Point>>) -> Self {
                Self(points.into())
            }

            #[inline]
            pub fn as_slice(&self) -> &[$Point] {
                &self.0
            }

            #[inline]
            pub fn as_mut_slice(&mut self) -> &mut [$Point] {
                &mut self.0
            }

            #[inline]
            pub fn iter(&self) -> std::slice::Iter<'_, $Point> {
                self.0.iter()
            }

            #[inline]
            pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, $Point> {
                self.0.iter_mut()
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            /// Copy of vertex `index`. / 拷贝下标 `index` 的顶点。
            #[inline]
            pub fn get_point(&self, index: usize) -> $Point {
                self.0[index]
            }

            pub fn add_point(&mut self, point: $Point) {
                self.0.push(point)
            }

            /// Translate by integer deltas. / 整数平移。
            pub fn translate(&self, dx: i64, dy: i64) -> Self {
                let n = self.0.len();
                let mut new_points = Vec::with_capacity(n);
                new_points.extend(self.0.iter().map(|p| $Point {
                    x: p.x + dx,
                    y: p.y + dy,
                }));
                Self(new_points)
            }

            /// Scale coordinates (`sx`/`sy` — `0` treated as `1`). / 缩放（0 视为 1）。
            pub fn scale(&self, sx: f64, sy: f64) -> Self {
                let mut _sx = sx;
                if _sx == 0. {
                    _sx = 1.;
                }
                let mut _sy = sy;
                if _sy == 0. {
                    _sy = 1.;
                }
                let n = self.0.len();
                let mut new_points = Vec::with_capacity(n);
                new_points.extend(self.0.iter().map(|p| $Point {
                    x: ((p.x as f64) * _sx) as i64,
                    y: ((p.y as f64) * _sy) as i64,
                }));
                Self(new_points)
            }
        }

        impl<'a> IntoIterator for &'a $Path {
            type Item = &'a $Point;
            type IntoIter = std::slice::Iter<'a, $Point>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl IntoIterator for $Path {
            type Item = $Point;
            type IntoIter = std::vec::IntoIter<$Point>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }
    };
    (float, $Path:ident, $Point:ident) => {
        impl $Path {
            /// See int64 `Path64` methods (float variants). / 与 `Path64` 方法对偶（浮点版）。
            #[inline]
            pub fn from_vec(points: Vec<$Point>) -> Self {
                Self(points)
            }

            #[inline]
            pub fn new(points: impl Into<Vec<$Point>>) -> Self {
                Self(points.into())
            }

            #[inline]
            pub fn as_slice(&self) -> &[$Point] {
                &self.0
            }

            #[inline]
            pub fn as_mut_slice(&mut self) -> &mut [$Point] {
                &mut self.0
            }

            #[inline]
            pub fn iter(&self) -> std::slice::Iter<'_, $Point> {
                self.0.iter()
            }

            #[inline]
            pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, $Point> {
                self.0.iter_mut()
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            #[inline]
            pub fn get_point(&self, index: usize) -> $Point {
                self.0[index]
            }

            pub fn add_point(&mut self, point: $Point) {
                self.0.push(point)
            }

            pub fn translate(&self, dx: f64, dy: f64) -> Self {
                let n = self.0.len();
                let mut new_points = Vec::with_capacity(n);
                new_points.extend(self.0.iter().map(|p| $Point {
                    x: p.x + dx,
                    y: p.y + dy,
                }));
                Self(new_points)
            }

            pub fn scale(&self, sx: f64, sy: f64) -> Self {
                let mut _sx = sx;
                if _sx == 0. {
                    _sx = 1.;
                }
                let mut _sy = sy;
                if _sy == 0. {
                    _sy = 1.;
                }
                let n = self.0.len();
                let mut new_points = Vec::with_capacity(n);
                new_points.extend(self.0.iter().map(|p| $Point {
                    x: p.x * _sx,
                    y: p.y * _sy,
                }));
                Self(new_points)
            }
        }

        impl<'a> IntoIterator for &'a $Path {
            type Item = &'a $Point;
            type IntoIter = std::slice::Iter<'a, $Point>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.iter()
            }
        }

        impl IntoIterator for $Path {
            type Item = $Point;
            type IntoIter = std::vec::IntoIter<$Point>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }
    };
}

/// Multi-path container: forwards `translate` / `scale` to each path.
///
/// 多路径容器：将 `translate` / `scale` 逐条委托给单路径。
macro_rules! impl_paths_collection {
    (int64, $Paths:ident, $Path:ty) => {
        impl $Paths {
            /// Internal: wrap `Vec`. / 内部：包装 `Vec`。
            #[inline]
            pub(crate) fn from_vec(paths: Vec<$Path>) -> Self {
                Self(paths)
            }

            /// New collection from any `Into<Vec>`. / 由 `Into<Vec>` 构造。
            #[inline]
            pub fn new(paths: impl Into<Vec<$Path>>) -> Self {
                Self(paths.into())
            }

            /// Slice of all paths. / 全部路径切片。
            #[inline]
            pub fn as_slice(&self) -> &[$Path] {
                &self.0
            }

            /// Borrow one path by index. / 按下标借用路径。
            #[inline]
            pub fn path(&self, index: usize) -> &$Path {
                &self.0[index]
            }

            /// True if there are zero paths. / 是否无路径。
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            /// Path count. / 路径条数。
            pub fn len(&self) -> usize {
                self.0.len()
            }

            /// Push one path. / 追加一条路径。
            pub fn add_path(&mut self, path: $Path) {
                self.0.push(path)
            }

            /// Extend with many paths. / 追加多条。
            pub fn add_paths(&mut self, paths: impl IntoIterator<Item = $Path>) {
                self.0.extend(paths)
            }

            /// Clone all paths into a vector. / 克隆为 `Vec`。
            pub fn to_vec(&self) -> Vec<$Path> {
                self.0.clone()
            }

            /// Per-path translate with `i64` deltas. / 逐路径平移（`i64`）。
            pub fn translate(&self, dx: i64, dy: i64) -> Self {
                let n = self.0.len();
                let mut new_paths = Vec::with_capacity(n);
                new_paths.extend(self.0.iter().map(|p| p.translate(dx, dy)));
                Self(new_paths)
            }

            /// Per-path scale (`f64` factors, 0 → 1). / 逐路径缩放。
            pub fn scale(&self, sx: f64, sy: f64) -> Self {
                let n = self.0.len();
                let mut new_paths = Vec::with_capacity(n);
                new_paths.extend(self.0.iter().map(|p| p.scale(sx, sy)));
                Self(new_paths)
            }
        }

        impl FromIterator<$Path> for $Paths {
            /// Collect iterator of paths. / 从迭代器收集。
            fn from_iter<T: IntoIterator<Item = $Path>>(iter: T) -> Self {
                Self(iter.into_iter().collect())
            }
        }
    };
    (float, $Paths:ident, $Path:ty) => {
        impl $Paths {
            #[inline]
            pub(crate) fn from_vec(paths: Vec<$Path>) -> Self {
                Self(paths)
            }

            #[inline]
            pub fn new(paths: impl Into<Vec<$Path>>) -> Self {
                Self(paths.into())
            }

            #[inline]
            pub fn as_slice(&self) -> &[$Path] {
                &self.0
            }

            #[inline]
            pub fn path(&self, index: usize) -> &$Path {
                &self.0[index]
            }

            /// See int64 arm. / 同整型版各方法语义。
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn add_path(&mut self, path: $Path) {
                self.0.push(path)
            }

            pub fn add_paths(&mut self, paths: impl IntoIterator<Item = $Path>) {
                self.0.extend(paths)
            }

            pub fn to_vec(&self) -> Vec<$Path> {
                self.0.clone()
            }

            /// Per-path translate (`f64`). / 逐路径平移。
            pub fn translate(&self, dx: f64, dy: f64) -> Self {
                let n = self.0.len();
                let mut new_paths = Vec::with_capacity(n);
                new_paths.extend(self.0.iter().map(|p| p.translate(dx, dy)));
                Self(new_paths)
            }

            pub fn scale(&self, sx: f64, sy: f64) -> Self {
                let n = self.0.len();
                let mut new_paths = Vec::with_capacity(n);
                new_paths.extend(self.0.iter().map(|p| p.scale(sx, sy)));
                Self(new_paths)
            }
        }

        impl FromIterator<$Path> for $Paths {
            fn from_iter<T: IntoIterator<Item = $Path>>(iter: T) -> Self {
                Self(iter.into_iter().collect())
            }
        }
    };
}
