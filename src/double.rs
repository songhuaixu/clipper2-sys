use std::os::raw::c_int;

mod path_d;
pub use path_d::*;

mod paths_d;
pub use paths_d::*;

mod clipper_d;
pub use clipper_d::*;

mod poly_tree_d;
pub use poly_tree_d::*;

use crate::{clipper2::*, malloc, EndType, JoinType};

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
            let path_ptr = clipper_pathd_simplify(
                mem,
                self.get_clipper_path(),
                epsilon,
                if is_open_path { 1 } else { 0 },
            );
            let path = Self::from(path_ptr);
            clipper_delete_pathd(path_ptr);
            path
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

impl PathsD {
    pub fn simplify(&self, epsilon: f64, is_open_path: bool) -> PathsD {
        unsafe {
            let mem = malloc(clipper_pathsd_size());
            let paths_ptr = clipper_pathsd_simplify(
                mem,
                self.get_clipper_paths(),
                epsilon,
                if is_open_path { 1 } else { 0 },
            );
            let paths = Self::from(paths_ptr);
            clipper_delete_pathsd(paths_ptr);
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
            let paths_ptr = clipper_pathsd_inflate(
                mem,
                self.get_clipper_paths(),
                delta,
                join_type.into(),
                end_type.into(),
                miter_limit,
                precision,
            );
            let paths = Self::from(paths_ptr);
            clipper_delete_pathsd(paths_ptr);
            paths
        }
    }
}
