use crate::{
    clipper_delete_pathd, clipper_pathd_size, clipper_polytreed_count, clipper_polytreed_get_child,
    clipper_polytreed_inv_scale, clipper_polytreed_is_hole, clipper_polytreed_polygon, malloc,
    ClipperPolyTreeD, PathD, PathsD,
};

#[derive(Debug)]
pub struct PolyTreeD {
    pub(crate) childs: Vec<Self>,
    pub(crate) is_hole: bool,
    pub(crate) polygon: PathD,
    pub(crate) scale: f64,
}

impl PolyTreeD {
    pub(crate) fn from(ptr: *mut ClipperPolyTreeD) -> Self {
        let is_hole: bool;
        let scale: f64;
        let polygon: PathD;
        let childs: Vec<Self>;
        unsafe {
            is_hole = clipper_polytreed_is_hole(ptr) == 1;
            scale = clipper_polytreed_inv_scale(ptr);
            let mem = malloc(clipper_pathd_size());
            let polygon_prt = clipper_polytreed_polygon(mem, ptr);
            polygon = PathD::from(polygon_prt);
            clipper_delete_pathd(polygon_prt);
            let count = clipper_polytreed_count(ptr);
            childs = (0..count)
                .map(|i| {
                    let tree_ptr = clipper_polytreed_get_child(ptr, i);
                    Self::from(tree_ptr as *mut ClipperPolyTreeD)
                })
                .collect();
        }
        Self {
            childs: childs,
            is_hole: is_hole,
            polygon: polygon,
            scale: scale,
        }
    }
}

impl PolyTreeD {

    pub fn get_childs(&mut self) -> &mut Vec<Self> {
        &mut self.childs
    }

    pub fn is_hole(&self) -> bool {
        self.is_hole
    }

    pub fn get_hole_paths(&self) -> PathsD {
        let mut paths = PathsD::new(&vec![]);
        for child in &self.childs {
            if child.is_hole {
                paths.add_path(child.get_polygon());
            }
        }
        paths
    }

    pub fn get_polygon(&self) -> PathD {
        self.polygon.clone()
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }
}
