use crate::cxx_bridge::clipper2_sys_cxx;
use crate::paths_blob::{path64_to_blob, paths64_to_blob};
use crate::{EndType, JoinType, LazyPaths64, Path64, Paths64};

/// Clipper2 path offsetter: inflate or deflate polygons/polylines (integer `Path64`).
///
/// Clipper2 路径偏移器：对 `Path64` 做膨胀或收缩。
pub struct ClipperOffset {
    inner: cxx::UniquePtr<clipper2_sys_cxx::ClipperOffsetBox>,
}

impl ClipperOffset {
    /// Creates an offset engine (`miter_limit`, `arc_tolerance`, flags).
    ///
    /// 创建偏移引擎（斜接上限、圆弧容差、布尔选项）。
    pub fn new(
        miter_limit: f64,
        arc_tolerance: f64,
        preserve_collinear: bool,
        reverse_solution: bool,
    ) -> Self {
        Self {
            inner: clipper2_sys_cxx::cxx_clipper_offset_new(
                miter_limit,
                arc_tolerance,
                preserve_collinear,
                reverse_solution,
            ),
        }
    }

    /// Adds one path with join and end types. / 添加单条路径及连接/端点类型。
    pub fn add_path(&mut self, path: &Path64, join_type: JoinType, end_type: EndType) {
        clipper2_sys_cxx::cxx_clipper_offset_add_path(
            self.inner.pin_mut(),
            &path64_to_blob(path),
            join_type.into(),
            end_type.into(),
        );
    }

    /// Adds many paths. / 添加多条路径。
    pub fn add_paths(&mut self, paths: &Paths64, join_type: JoinType, end_type: EndType) {
        clipper2_sys_cxx::cxx_clipper_offset_add_paths(
            self.inner.pin_mut(),
            &paths64_to_blob(paths),
            join_type.into(),
            end_type.into(),
        );
    }

    /// Runs offset by signed `delta` (positive = outward for closed paths). / 按有符号 `delta` 执行偏移。
    pub fn execute(&mut self, delta: f64) -> LazyPaths64 {
        let out = clipper2_sys_cxx::cxx_clipper_offset_execute(self.inner.pin_mut(), delta);
        LazyPaths64::from_blob(out)
    }

    /// Clears queued paths. / 清空已添加路径。
    pub fn clear(&mut self) {
        clipper2_sys_cxx::cxx_clipper_offset_clear(self.inner.pin_mut());
    }

    /// Miter limit. / 斜接上限。
    pub fn get_miter_limit(&self) -> f64 {
        clipper2_sys_cxx::cxx_clipper_offset_get_miter_limit(&self.inner)
    }

    /// Sets miter limit. / 设置斜接上限。
    pub fn set_miter_limit(&mut self, miter_limit: f64) {
        clipper2_sys_cxx::cxx_clipper_offset_set_miter_limit(self.inner.pin_mut(), miter_limit);
    }

    /// Arc tolerance for rounding. / 圆角弧形容差。
    pub fn get_arc_tolerance(&self) -> f64 {
        clipper2_sys_cxx::cxx_clipper_offset_get_arc_tolerance(&self.inner)
    }

    /// Sets arc tolerance. / 设置弧形容差。
    pub fn set_arc_tolerance(&mut self, arc_tolerance: f64) {
        clipper2_sys_cxx::cxx_clipper_offset_set_arc_tolerance(self.inner.pin_mut(), arc_tolerance);
    }

    /// Preserve-collinear flag. / 保留共线顶点。
    pub fn get_preserve_collinear(&self) -> bool {
        clipper2_sys_cxx::cxx_clipper_offset_get_preserve_collinear(&self.inner)
    }

    /// Sets preserve-collinear. / 设置是否保留共线顶点。
    pub fn set_preserve_collinear(&mut self, preserve_collinear: bool) {
        clipper2_sys_cxx::cxx_clipper_offset_set_preserve_collinear(
            self.inner.pin_mut(),
            preserve_collinear,
        );
    }

    /// Reverse-solution flag. / 反转解。
    pub fn get_reverse_solution(&self) -> bool {
        clipper2_sys_cxx::cxx_clipper_offset_get_reverse_solution(&self.inner)
    }

    /// Sets reverse-solution. / 设置是否反转解方向。
    pub fn set_reverse_solution(&mut self, reverse_solution: bool) {
        clipper2_sys_cxx::cxx_clipper_offset_set_reverse_solution(
            self.inner.pin_mut(),
            reverse_solution,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClipperOffset, EndType, JoinType, Path64, Point64};

    #[test]
    fn offset_square_returns_paths() {
        let path = Path64::new(vec![
            Point64::new(0, 0),
            Point64::new(100, 0),
            Point64::new(100, 100),
            Point64::new(0, 100),
        ]);
        let mut co = ClipperOffset::new(2.0, 0.0, false, false);
        co.add_path(&path, JoinType::MiterJoin, EndType::PolygonEnd);
        let out = co.execute(10.0);
        assert!(!out.is_empty());
    }
}
