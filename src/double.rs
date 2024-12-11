use std::os::raw::c_int;

mod path_d;
pub use path_d::*;

mod paths_d;
pub use paths_d::*;

mod clipper_d;
pub use clipper_d::*;

mod poly_tree_d;
pub use poly_tree_d::*;

use crate::{clipper2::*, malloc, EndType, JoinType, Path64, Paths64, PointInPolygonResult};

pub type PointD = ClipperPointD;

impl PointD {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x: x, y: y }
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

impl PathD {
    pub fn simplify(&self, epsilon: f64, is_open_path: bool) -> PathD {
        unsafe {
            let mem = malloc(clipper_pathd_size());
            let pathd_prt = self.get_clipper_path();
            let path_ptr =
                clipper_pathd_simplify(mem, pathd_prt, epsilon, if is_open_path { 1 } else { 0 });
            let path = Self::from(path_ptr);
            clipper_delete_pathd(path_ptr);
            clipper_delete_pathd(pathd_prt);
            path
        }
    }

    pub fn point_in_polygon(&self, point: PointD) -> PointInPolygonResult {
        unsafe {
            let pathd_prt = self.get_clipper_path();
            let result = clipper_point_in_pathd(pathd_prt, point).into();
            clipper_delete_pathd(pathd_prt);
            result
        }
    }

    pub fn to_path64(&self) -> Path64 {
        unsafe {
            let mem = malloc(clipper_path64_size());
            let pathd_prt = self.get_clipper_path();
            let path64_ptr = clipper_pathd_to_path64(mem, pathd_prt);
            let path64 = Path64::from(path64_ptr);
            clipper_delete_path64(path64_ptr);
            clipper_delete_pathd(pathd_prt);
            path64
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
            let result = clipper_pathsd_of_paths(mem, paths.as_mut_ptr(), self.len());
            for ptr in paths {
                clipper_delete_pathd(ptr);
            }
            result
        }
    }
}

impl PathsD {
    pub fn simplify(&self, epsilon: f64, is_open_path: bool) -> PathsD {
        unsafe {
            let mem = malloc(clipper_pathsd_size());
            let pathsd_prt = self.get_clipper_paths();
            let paths_ptr =
                clipper_pathsd_simplify(mem, pathsd_prt, epsilon, if is_open_path { 1 } else { 0 });
            let paths = Self::from(paths_ptr);
            clipper_delete_pathsd(paths_ptr);
            clipper_delete_pathsd(pathsd_prt);
            paths
        }
    }

    pub fn inflate(
        &self,
        delta: f64,
        join_type: JoinType,
        end_type: EndType,
        miter_limit: f64,
        precision: c_int,
    ) -> PathsD {
        unsafe {
            let mem = malloc(clipper_pathsd_size());
            let pathsd_prt = self.get_clipper_paths();
            let paths_ptr = clipper_pathsd_inflate(
                mem,
                pathsd_prt,
                delta,
                join_type.into(),
                end_type.into(),
                miter_limit,
                precision,
            );
            let paths = Self::from(paths_ptr);
            clipper_delete_pathsd(paths_ptr);
            clipper_delete_pathsd(pathsd_prt);
            paths
        }
    }

    pub fn to_paths64(&self) -> Paths64 {
        unsafe {
            let mem = malloc(clipper_paths64_size());
            let pathsd_prt = self.get_clipper_paths();
            let paths64_ptr = clipper_pathsd_to_paths64(mem, pathsd_prt);
            let paths64 = Paths64::from(paths64_ptr);
            clipper_delete_paths64(paths64_ptr);
            clipper_delete_pathsd(pathsd_prt);
            paths64
        }
    }
}

// Minkowski

impl PathD {
    pub fn minkowski_sum(&self, pattern: &PathD, is_closed: bool, precision: i32) -> PathsD {
        unsafe {
            let pattern_ptr = pattern.get_clipper_path();
            let path_ptr = self.get_clipper_path();

            let mem = malloc(clipper_pathsd_size());
            let result_prt = clipper_pathd_minkowski_sum(
                mem,
                pattern_ptr,
                path_ptr,
                if is_closed { 1 } else { 0 },
                precision,
            );
            let result = PathsD::from(result_prt);
            clipper_delete_pathsd(result_prt);
            clipper_delete_pathd(pattern_ptr);
            clipper_delete_pathd(path_ptr);
            result
        }
    }

    pub fn minkowski_diff(&self, pattern: &PathD, is_closed: bool, precision: i32) -> PathsD {
        unsafe {
            let pattern_ptr = pattern.get_clipper_path();
            let path_ptr = self.get_clipper_path();

            let mem = malloc(clipper_pathsd_size());
            let result_prt = clipper_pathd_minkowski_diff(
                mem,
                pattern_ptr,
                path_ptr,
                if is_closed { 1 } else { 0 },
                precision,
            );
            let result = PathsD::from(result_prt);
            clipper_delete_pathsd(result_prt);
            clipper_delete_pathd(pattern_ptr);
            clipper_delete_pathd(path_ptr);
            result
        }
    }
}

impl PathsD {
    pub fn minkowski_sum(
        &self,
        pattern: &PathD,
        is_closed: bool,
        precision: i32,
        fillrule: ClipperFillRule,
    ) -> PathsD {
        unsafe {
            let pattern_ptr = pattern.get_clipper_path();
            let paths_ptr = self.get_clipper_paths();

            let mem = malloc(clipper_pathsd_size());
            let result_prt = clipper_pathsd_minkowski_sum(
                mem,
                pattern_ptr,
                paths_ptr,
                if is_closed { 1 } else { 0 },
                precision,
                fillrule.into(),
            );
            let result = PathsD::from(result_prt);
            clipper_delete_pathsd(result_prt);
            clipper_delete_pathd(pattern_ptr);
            clipper_delete_pathsd(paths_ptr);
            result
        }
    }

    pub fn minkowski_diff(
        &self,
        pattern: &PathD,
        is_closed: bool,
        precision: i32,
        fillrule: ClipperFillRule,
    ) -> PathsD {
        unsafe {
            let pattern_ptr = pattern.get_clipper_path();
            let paths_ptr = self.get_clipper_paths();

            let mem = malloc(clipper_pathsd_size());
            let result_prt = clipper_pathsd_minkowski_diff(
                mem,
                pattern_ptr,
                paths_ptr,
                if is_closed { 1 } else { 0 },
                precision,
                fillrule.into(),
            );
            let result = PathsD::from(result_prt);
            clipper_delete_pathsd(result_prt);
            clipper_delete_pathd(pattern_ptr);
            clipper_delete_pathsd(paths_ptr);
            result
        }
    }
}

// Area

impl PathD {

    pub fn area(&self) -> f64 {
        unsafe {
            let pathd_prt = self.get_clipper_path();
            let reuslt = clipper_pathd_area(pathd_prt);
            clipper_delete_pathd(pathd_prt);
            reuslt
        }
    }

}

impl PathsD {

    pub fn area(&self) -> f64 {
        unsafe {
            let pathsd_prt = self.get_clipper_paths();
            let reuslt = clipper_pathsd_area(pathsd_prt);
            clipper_delete_pathsd(pathsd_prt);
            reuslt
        }
    }

}