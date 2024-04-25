use crate::{
    clipper_clipper64_size, clipper_clipperoffset, clipper_clipperoffset_add_path64,
    clipper_clipperoffset_add_paths64, clipper_clipperoffset_clear, clipper_clipperoffset_execute,
    clipper_clipperoffset_get_arc_tolerance, clipper_clipperoffset_get_miter_limit,
    clipper_clipperoffset_get_preserve_collinear, clipper_clipperoffset_get_reverse_solution,
    clipper_clipperoffset_set_arc_tolerance, clipper_clipperoffset_set_miter_limit,
    clipper_clipperoffset_set_preserve_collinear, clipper_clipperoffset_set_reverse_solution,
    clipper_clipperoffset_size, clipper_delete_clipperoffset, clipper_delete_paths64, malloc,
    ClipperClipperOffset, EndType, JoinType, Path64, Paths64,
};

pub struct ClipperOffset {
    ptr: *mut ClipperClipperOffset,
}

impl ClipperOffset {
    pub fn new(
        miter_limit: f64,
        arc_tolerance: f64,
        preserve_collinear: bool,
        reverse_solution: bool,
    ) -> Self {
        let ptr = unsafe {
            let mem = malloc(clipper_clipperoffset_size());
            clipper_clipperoffset(
                mem,
                miter_limit,
                arc_tolerance,
                if preserve_collinear { 1 } else { 0 },
                if reverse_solution { 1 } else { 0 },
            )
        };
        Self { ptr: ptr }
    }

    pub fn add_path(&self, path: Path64, join_type: JoinType, end_type: EndType) {
        unsafe {
            clipper_clipperoffset_add_path64(
                self.ptr,
                path.get_clipper_path(),
                join_type.into(),
                end_type.into(),
            )
        }
    }

    pub fn add_paths(&self, paths: Paths64, join_type: JoinType, end_type: EndType) {
        unsafe {
            clipper_clipperoffset_add_paths64(
                self.ptr,
                paths.get_clipper_paths(),
                join_type.into(),
                end_type.into(),
            )
        }
    }

    pub fn execute(&self, delta: f64) -> Paths64 {
        unsafe {
            let mem = malloc(clipper_clipper64_size());
            let paths_ptr = clipper_clipperoffset_execute(mem, self.ptr, delta);
            let paths = Paths64::from(paths_ptr);
            clipper_delete_paths64(paths_ptr);
            paths
        }
    }

    pub fn clear(&self) {
        unsafe { clipper_clipperoffset_clear(self.ptr) }
    }

    pub fn get_miter_limit(&self) -> f64 {
        unsafe { clipper_clipperoffset_get_miter_limit(self.ptr) }
    }

    pub fn set_miter_limit(&self, miter_limit: f64) {
        unsafe { clipper_clipperoffset_set_miter_limit(self.ptr, miter_limit) }
    }

    pub fn get_arc_tolerance(&self) -> f64 {
        unsafe { clipper_clipperoffset_get_arc_tolerance(self.ptr) }
    }

    pub fn set_arc_tolerance(&self, arc_tolerance: f64) {
        unsafe { clipper_clipperoffset_set_arc_tolerance(self.ptr, arc_tolerance) }
    }

    pub fn get_preserve_collinear(&self) -> bool {
        unsafe { clipper_clipperoffset_get_preserve_collinear(self.ptr) == 1 }
    }

    pub fn set_preserve_collinear(&self, preserve_collinear: bool) {
        unsafe {
            clipper_clipperoffset_set_preserve_collinear(
                self.ptr,
                if preserve_collinear { 1 } else { 0 },
            )
        }
    }

    pub fn get_reverse_solution(&self) -> bool {
        unsafe { clipper_clipperoffset_get_reverse_solution(self.ptr) == 1 }
    }

    pub fn set_reverse_solution(&self, reverse_solution: bool) {
        unsafe {
            clipper_clipperoffset_set_reverse_solution(
                self.ptr,
                if reverse_solution { 1 } else { 0 },
            )
        }
    }
}

impl Drop for ClipperOffset {
    fn drop(&mut self) {
        unsafe { clipper_delete_clipperoffset(self.ptr) }
    }
}
