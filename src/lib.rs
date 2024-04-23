mod clipper2;
mod clipper_d;
mod path_d;
mod paths_d;
mod poly_tree_d;
pub(crate) use clipper2::*;
pub use clipper_d::*;
pub use path_d::*;
pub use paths_d::*;
pub use poly_tree_d::*;

pub type PointD = ClipperPointD;

#[cfg(test)]
mod tests;

pub(crate) unsafe fn malloc(size: usize) -> *mut std::os::raw::c_void {
    libc::malloc(size)
}

pub(crate) unsafe fn free(p: *mut std::os::raw::c_void) {
    libc::free(p)
}

#[derive(Clone, Copy)]
pub enum FillRule {
    EvenOdd,
    NonZero,
    Positive,
    Negative,
}

#[derive(Clone, Copy)]
pub enum ClipType {
    None,
    Intersection,
    Union,
    Difference,
    Xor,
}

impl From<ClipType> for ClipperClipType {
    fn from(pft: ClipType) -> Self {
        match pft {
            ClipType::None => ClipperClipType_NONE,
            ClipType::Intersection => ClipperClipType_INTERSECTION,
            ClipType::Union => ClipperClipType_UNION,
            ClipType::Difference => ClipperClipType_DIFFERENCE,
            ClipType::Xor => ClipperClipType_XOR,
        }
    }
}

impl From<FillRule> for ClipperFillRule {
    fn from(pft: FillRule) -> Self {
        match pft {
            FillRule::EvenOdd => ClipperFillRule_EVEN_ODD,
            FillRule::NonZero => ClipperFillRule_NON_ZERO,
            FillRule::Positive => ClipperFillRule_POSITIVE,
            FillRule::Negative => ClipperFillRule_NEGATIVE,
        }
    }
}

impl PathD {
    pub(crate) fn from(ptr: *mut ClipperPathD) -> Self {
        let points = unsafe {
            let len: i32 = clipper_pathd_length(ptr).try_into().unwrap();
            (0..len).map(|i| clipper_pathd_get_point(ptr, i)).collect()
        };
        Self::new(&points)
    }

    pub(crate) fn get_clipper_path(&self) -> *mut ClipperPathD {
        unsafe {
            let mem = malloc(clipper_pathd_size());
            clipper_pathd_of_points(mem, self.0.clone().as_mut_ptr(), self.len())
        }
    }
}

impl PathsD {
    pub(crate) fn from(ptr: *mut ClipperPathsD) -> Self {
        let paths = unsafe {
            let len: i32 = clipper_pathsd_length(ptr).try_into().unwrap();
            (0..len)
                .map(|i| {
                    let point_len: i32 = clipper_pathsd_path_length(ptr, i).try_into().unwrap();
                    let points = (0..point_len)
                        .map(|j| clipper_pathsd_get_point(ptr, i, j))
                        .collect();
                    PathD::new(&points)
                })
                .collect()
        };
        Self::new(&paths)
    }

    pub(crate) fn get_clipper_paths(&self) -> *mut ClipperPathsD {
        unsafe {
            let mem = malloc(clipper_pathsd_size());
            let mut paths = self
                .0
                .iter()
                .map(|p| p.get_clipper_path())
                .collect::<Vec<*mut ClipperPathD>>();
            clipper_pathsd_of_paths(mem, paths.as_mut_ptr(), self.len())
        }
    }
}