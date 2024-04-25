use std::os::raw::c_int;

use crate::{
    clipper_clipperd, clipper_clipperd_add_clip, clipper_clipperd_add_open_subject,
    clipper_clipperd_add_subject, clipper_clipperd_clear, clipper_clipperd_execute,
    clipper_clipperd_execute_tree_with_open, clipper_clipperd_get_preserve_collinear,
    clipper_clipperd_get_reverse_solution, clipper_clipperd_set_preserve_collinear,
    clipper_clipperd_set_reverse_solution, clipper_clipperd_size, clipper_delete_clipperd,
    clipper_delete_pathsd, clipper_delete_polytreed, clipper_polytreed, clipper_polytreed_size,
    malloc, ClipType, ClipperClipperD, FillRule, PathsD, PolyTreeD,
};

pub struct ClipperD {
    ptr: *mut ClipperClipperD,
}

impl ClipperD {
    pub fn new(precision: c_int) -> Self {
        let ptr = unsafe {
            let mem = malloc(clipper_clipperd_size());
            clipper_clipperd(mem, precision)
        };
        Self { ptr: ptr }
    }

    pub fn set_preserve_collinear(&self, value: bool) {
        unsafe { clipper_clipperd_set_preserve_collinear(self.ptr, if value { 1 } else { 0 }) }
    }

    pub fn get_preserve_collinear(&self) -> bool {
        unsafe { clipper_clipperd_get_preserve_collinear(self.ptr) == 1 }
    }

    pub fn set_reverse_solution(&self, value: bool) {
        unsafe { clipper_clipperd_set_reverse_solution(self.ptr, if value { 1 } else { 0 }) }
    }

    pub fn get_reverse_solution(&self) -> bool {
        unsafe { clipper_clipperd_get_reverse_solution(self.ptr) == 1 }
    }

    pub fn clear(&self) {
        unsafe { clipper_clipperd_clear(self.ptr) }
    }

    pub fn add_open_subject(&self, open_subject: PathsD) {
        unsafe { clipper_clipperd_add_open_subject(self.ptr, open_subject.get_clipper_paths()) }
    }

    pub fn add_subject(&self, subject: PathsD) {
        unsafe { clipper_clipperd_add_subject(self.ptr, subject.get_clipper_paths()) }
    }

    pub fn add_clip(&self, clip: PathsD) {
        unsafe { clipper_clipperd_add_clip(self.ptr, clip.get_clipper_paths()) }
    }

    pub fn boolean_operation(&self, clip_type: ClipType, fill_rule: FillRule) -> PathsD {
        let closed_path = PathsD::new(&vec![]).get_clipper_paths();
        let open_path = PathsD::new(&vec![]).get_clipper_paths();
        unsafe {
            let is_success = clipper_clipperd_execute(
                self.ptr,
                clip_type.into(),
                fill_rule.into(),
                closed_path,
                open_path,
            );

            let path = PathsD::from(closed_path);
            clipper_delete_pathsd(closed_path);
            clipper_delete_pathsd(open_path);
            path
        }
    }

    pub fn boolean_operation_tree(&self, clip_type: ClipType, fill_rule: FillRule) -> PolyTreeD {
        let tree_ptr = unsafe {
            let mem = malloc(clipper_polytreed_size());
            clipper_polytreed(mem, std::ptr::null_mut())
        };
        let open_path = PathsD::new(&vec![]).get_clipper_paths();
        unsafe {
            let is_success = clipper_clipperd_execute_tree_with_open(
                self.ptr,
                clip_type.into(),
                fill_rule.into(),
                tree_ptr,
                open_path,
            );
            clipper_delete_pathsd(open_path);
            let tree = PolyTreeD::from(tree_ptr);
            clipper_delete_polytreed(tree_ptr);
            tree
        }
    }
}

impl Drop for ClipperD {
    fn drop(&mut self) {
        unsafe { clipper_delete_clipperd(self.ptr) }
    }
}
