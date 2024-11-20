mod path_64;
pub use path_64::*;

mod paths_64;
pub use paths_64::*;

mod clipper_64;
pub use clipper_64::*;

mod poly_tree_64;
pub use poly_tree_64::*;

use crate::{clipper2::*, malloc, EndType, JoinType, PathD, PathsD, PointInPolygonResult};

pub type Point64 = ClipperPoint64;

impl Point64 {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x: x, y: y }
    }
}

impl Path64 {
    pub(crate) fn from(ptr: *mut ClipperPath64) -> Self {
        let points = unsafe {
            let len: i32 = clipper_path64_length(ptr).try_into().unwrap();
            (0..len).map(|i| clipper_path64_get_point(ptr, i)).collect()
        };
        Self::new(&points)
    }

    pub(crate) fn get_clipper_path(&self) -> *mut ClipperPath64 {
        unsafe {
            let mem = malloc(clipper_path64_size());
            clipper_path64_of_points(mem, self.0.clone().as_mut_ptr(), self.len())
        }
    }
}

impl Path64 {
    pub fn simplify(&self, epsilon: f64, is_open_path: bool) -> Path64 {
        unsafe {
            let mem: *mut std::ffi::c_void = malloc(clipper_path64_size());
            let path_ptr = self.get_clipper_path();
            let path_ptr = clipper_path64_simplify(
                mem,
                path_ptr,
                epsilon,
                if is_open_path { 1 } else { 0 },
            );
            let path = Self::from(path_ptr);
            clipper_delete_path64(path_ptr);
            clipper_delete_path64(path_ptr);
            path
        }
    }

    pub fn point_in_polygon(&self, point: Point64) -> PointInPolygonResult {
        unsafe {
            let path_ptr = self.get_clipper_path();
            let result = clipper_point_in_path64(path_ptr, point).into();
            clipper_delete_path64(path_ptr);
            result
        }
    }

    pub fn to_pathd(&self) -> PathD {
        unsafe {
            let mem = malloc(clipper_pathd_size());
            let path_ptr = self.get_clipper_path();
            let pathd_ptr = clipper_path64_to_pathd(mem, path_ptr);
            let pathd = PathD::from(pathd_ptr);
            clipper_delete_pathd(pathd_ptr);
            clipper_delete_path64(path_ptr);
            pathd
        }
    }
}

impl Paths64 {
    pub(crate) fn from(ptr: *mut ClipperPaths64) -> Self {
        let paths = unsafe {
            let len: i32 = clipper_paths64_length(ptr).try_into().unwrap();
            (0..len)
                .map(|i| {
                    let point_len: i32 = clipper_paths64_path_length(ptr, i).try_into().unwrap();
                    let points = (0..point_len)
                        .map(|j| clipper_paths64_get_point(ptr, i, j))
                        .collect();
                    Path64::new(&points)
                })
                .collect()
        };
        Self::new(&paths)
    }

    pub(crate) fn get_clipper_paths(&self) -> *mut ClipperPaths64 {
        unsafe {
            let mem = malloc(clipper_paths64_size());
            let mut paths = self
                .0
                .iter()
                .map(|p| p.get_clipper_path())
                .collect::<Vec<*mut ClipperPath64>>();
            let result = clipper_paths64_of_paths(mem, paths.as_mut_ptr(), self.len());
            for prt in paths {
                clipper_delete_path64(prt);
            }
            result
        }
    }
}

impl Paths64 {
    pub fn simplify(&self, epsilon: f64, is_open_path: bool) -> Paths64 {
        unsafe {
            let mem = malloc(clipper_paths64_size());
            let path_prt = self.get_clipper_paths();
            let paths_ptr =
                clipper_paths64_simplify(mem, path_prt, epsilon, if is_open_path { 1 } else { 0 });
            let paths = Self::from(paths_ptr);
            clipper_delete_paths64(paths_ptr);
            clipper_delete_paths64(path_prt);
            paths
        }
    }

    pub fn inflate(
        &self,
        delta: f64,
        join_type: JoinType,
        end_type: EndType,
        miter_limit: f64,
    ) -> Paths64 {
        unsafe {
            let mem = malloc(clipper_paths64_size());
            let path_prt = self.get_clipper_paths();
            let paths_ptr = clipper_paths64_inflate(
                mem,
                path_prt,
                delta,
                join_type.into(),
                end_type.into(),
                miter_limit,
            );
            let paths = Self::from(paths_ptr);
            clipper_delete_paths64(paths_ptr);
            clipper_delete_paths64(path_prt);
            paths
        }
    }

    pub fn to_pathsd(&self) -> PathsD {
        unsafe {
            let mem = malloc(clipper_pathd_size());
            let paths_ptr = self.get_clipper_paths();
            let pathsd_ptr = clipper_paths64_to_pathsd(mem, paths_ptr);
            let pathsd = PathsD::from(pathsd_ptr);
            clipper_delete_pathsd(pathsd_ptr);
            clipper_delete_paths64(paths_ptr);
            pathsd
        }
    }
}

// Minkowski

impl Path64 {
    pub fn minkowski_sum(&self, pattern: &Path64, is_closed: bool) -> Paths64 {
        unsafe {
            let pattern_ptr = pattern.get_clipper_path();
            let path_ptr = self.get_clipper_path();

            let mem = malloc(clipper_paths64_size());
            let result_prt = clipper_path64_minkowski_sum(
                mem,
                pattern_ptr,
                path_ptr,
                if is_closed { 1 } else { 0 },
            );
            let result = Paths64::from(result_prt);
            clipper_delete_paths64(result_prt);
            clipper_delete_path64(pattern_ptr);
            clipper_delete_path64(path_ptr);
            result
        }
    }

    pub fn minkowski_diff(&self, pattern: &Path64, is_closed: bool) -> Paths64 {
        unsafe {
            let pattern_ptr = pattern.get_clipper_path();
            let path_ptr = self.get_clipper_path();

            let mem = malloc(clipper_paths64_size());
            let result_prt = clipper_path64_minkowski_diff(
                mem,
                pattern_ptr,
                path_ptr,
                if is_closed { 1 } else { 0 },
            );
            let result = Paths64::from(result_prt);
            clipper_delete_paths64(result_prt);
            clipper_delete_path64(pattern_ptr);
            clipper_delete_path64(path_ptr);
            result
        }
    }
}

impl Paths64 {
    pub fn minkowski_sum(
        &self,
        pattern: &Path64,
        is_closed: bool,

        fillrule: ClipperFillRule,
    ) -> Paths64 {
        unsafe {
            let pattern_ptr = pattern.get_clipper_path();
            let paths_ptr = self.get_clipper_paths();

            let mem = malloc(clipper_paths64_size());
            let result_prt = clipper_paths64_minkowski_sum(
                mem,
                pattern_ptr,
                paths_ptr,
                if is_closed { 1 } else { 0 },
                fillrule.into(),
            );
            let result = Paths64::from(result_prt);
            clipper_delete_paths64(result_prt);
            clipper_delete_path64(pattern_ptr);
            clipper_delete_paths64(paths_ptr);
            result
        }
    }

    pub fn minkowski_diff(
        &self,
        pattern: &Path64,
        is_closed: bool,
        fillrule: ClipperFillRule,
    ) -> Paths64 {
        unsafe {
            let pattern_ptr = pattern.get_clipper_path();
            let paths_ptr = self.get_clipper_paths();

            let mem = malloc(clipper_paths64_size());
            let result_prt = clipper_paths64_minkowski_diff(
                mem,
                pattern_ptr,
                paths_ptr,
                if is_closed { 1 } else { 0 },
                fillrule.into(),
            );
            let result = Paths64::from(result_prt);
            clipper_delete_paths64(result_prt);
            clipper_delete_path64(pattern_ptr);
            clipper_delete_paths64(paths_ptr);
            result
        }
    }
}
