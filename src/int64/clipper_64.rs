use crate::{
    clipper_clipper64, clipper_clipper64_add_clip, clipper_clipper64_add_open_subject,
    clipper_clipper64_add_subject, clipper_clipper64_clear, clipper_clipper64_execute,
    clipper_clipper64_execute_tree_with_open, clipper_clipper64_get_preserve_collinear,
    clipper_clipper64_get_reverse_solution, clipper_clipper64_set_preserve_collinear,
    clipper_clipper64_set_reverse_solution, clipper_clipper64_size, clipper_delete_clipper64,
    clipper_delete_paths64, clipper_delete_polytree64, clipper_polytree64, clipper_polytree64_size,
    malloc, ClipType, ClipperClipper64, FillRule, Paths64, PolyTree64,
};

pub struct Clipper64 {
    ptr: *mut ClipperClipper64,
}

impl Clipper64 {
    pub fn new() -> Self {
        let ptr = unsafe {
            let mem = malloc(clipper_clipper64_size());
            clipper_clipper64(mem)
        };
        Self { ptr: ptr }
    }

    pub fn set_preserve_collinear(&self, value: bool) {
        unsafe { clipper_clipper64_set_preserve_collinear(self.ptr, if value { 1 } else { 0 }) }
    }

    pub fn get_preserve_collinear(&self) -> bool {
        unsafe { clipper_clipper64_get_preserve_collinear(self.ptr) == 1 }
    }

    pub fn set_reverse_solution(&self, value: bool) {
        unsafe { clipper_clipper64_set_reverse_solution(self.ptr, if value { 1 } else { 0 }) }
    }

    pub fn get_reverse_solution(&self) -> bool {
        unsafe { clipper_clipper64_get_reverse_solution(self.ptr) == 1 }
    }

    pub fn clear(&self) {
        unsafe { clipper_clipper64_clear(self.ptr) }
    }

    pub fn add_open_subject(&self, open_subject: Paths64) {
        unsafe {
            let path_prt = open_subject.get_clipper_paths();
            clipper_clipper64_add_open_subject(self.ptr, path_prt);
            clipper_delete_paths64(path_prt);
        }
    }

    pub fn add_subject(&self, subject: Paths64) {
        unsafe {
            let path_prt = subject.get_clipper_paths();
            clipper_clipper64_add_subject(self.ptr, path_prt);
            clipper_delete_paths64(path_prt);
        }
    }

    pub fn add_clip(&self, clip: Paths64) {
        unsafe {
            let path_prt = clip.get_clipper_paths();
            clipper_clipper64_add_clip(self.ptr, path_prt);
            clipper_delete_paths64(path_prt);
        }
    }

    pub fn boolean_operation(&self, clip_type: ClipType, fill_rule: FillRule) -> Paths64 {
        let closed_path = Paths64::new(&vec![]).get_clipper_paths();
        let open_path = Paths64::new(&vec![]).get_clipper_paths();
        unsafe {
            let is_success = clipper_clipper64_execute(
                self.ptr,
                clip_type.into(),
                fill_rule.into(),
                closed_path,
                open_path,
            );

            let path = Paths64::from(closed_path);
            clipper_delete_paths64(closed_path);
            clipper_delete_paths64(open_path);
            path
        }
    }

    pub fn boolean_operation_tree(&self, clip_type: ClipType, fill_rule: FillRule) -> PolyTree64 {
        let tree_ptr = unsafe {
            let mem = malloc(clipper_polytree64_size());
            clipper_polytree64(mem, std::ptr::null_mut())
        };
        let open_path = Paths64::new(&vec![]).get_clipper_paths();
        unsafe {
            let is_success = clipper_clipper64_execute_tree_with_open(
                self.ptr,
                clip_type.into(),
                fill_rule.into(),
                tree_ptr,
                open_path,
            );
            clipper_delete_paths64(open_path);
            let tree = PolyTree64::from(tree_ptr);
            clipper_delete_polytree64(tree_ptr);
            tree
        }
    }
}

impl Drop for Clipper64 {
    fn drop(&mut self) {
        unsafe { clipper_delete_clipper64(self.ptr) }
    }
}
