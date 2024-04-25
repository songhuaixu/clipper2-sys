
mod path_64;
pub use path_64::*;

mod paths_64;
pub use paths_64::*;

mod clipper_64;
pub use clipper_64::*;


mod poly_tree_64;
pub use poly_tree_64::*;


use crate::{clipper2::*, malloc, EndType, JoinType};

pub type Point64 = ClipperPoint64;


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
            let mem = malloc(clipper_path64_size());
            let path_ptr = clipper_path64_simplify(
                mem,
                self.get_clipper_path(),
                epsilon,
                if is_open_path { 1 } else { 0 },
            );
            let path = Self::from(path_ptr);
            clipper_delete_path64(path_ptr);
            path
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
            clipper_paths64_of_paths(mem, paths.as_mut_ptr(), self.len())
        }
    }
}

impl Paths64 {
    pub fn simplify(&self, epsilon: f64, is_open_path: bool) -> Paths64 {
        unsafe {
            let mem = malloc(clipper_paths64_size());
            let paths_ptr = clipper_paths64_simplify(
                mem,
                self.get_clipper_paths(),
                epsilon,
                if is_open_path { 1 } else { 0 },
            );
            let paths = Self::from(paths_ptr);
            clipper_delete_paths64(paths_ptr);
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
            let paths_ptr = clipper_paths64_inflate(
                mem,
                self.get_clipper_paths(),
                delta,
                join_type.into(),
                end_type.into(),
                miter_limit,
            );
            let paths = Self::from(paths_ptr);
            clipper_delete_paths64(paths_ptr);
            paths
        }
    }
}
