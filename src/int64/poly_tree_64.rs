use crate::{
    clipper_delete_path64, clipper_path64_size, clipper_polytree64_count,
    clipper_polytree64_get_child, clipper_polytree64_is_hole, clipper_polytree64_polygon, malloc,
    ClipperPolyTree64, Path64, Paths64,
};

#[derive(Debug)]
pub struct PolyTree64 {
    pub(crate) childs: Vec<Self>,
    pub(crate) is_hole: bool,
    pub(crate) polygon: Path64,
}

impl PolyTree64 {
    pub(crate) fn from(ptr: *mut ClipperPolyTree64) -> Self {
        let is_hole: bool;
        let polygon: Path64;
        let childs: Vec<Self>;
        unsafe {
            is_hole = clipper_polytree64_is_hole(ptr) == 1;
            let mem = malloc(clipper_path64_size());
            let polygon_prt = clipper_polytree64_polygon(mem, ptr);
            polygon = Path64::from(polygon_prt);
            clipper_delete_path64(polygon_prt);
            let count = clipper_polytree64_count(ptr);
            childs = (0..count)
                .map(|i| {
                    let tree_ptr = clipper_polytree64_get_child(ptr, i);
                    Self::from(tree_ptr as *mut ClipperPolyTree64)
                })
                .collect();
        }
        Self {
            childs: childs,
            is_hole: is_hole,
            polygon: polygon,
        }
    }
}

impl PolyTree64 {
    pub fn get_childs(&mut self) -> &mut Vec<Self> {
        &mut self.childs
    }

    pub fn is_hole(&self) -> bool {
        self.is_hole
    }

    pub fn get_hole_paths(&self) -> Paths64 {
        let mut paths = Paths64::new(&vec![]);
        for child in &self.childs {
            if child.is_hole {
                paths.add_path(child.get_polygon());
            }
        }
        paths
    }

    pub fn get_polygon(&self) -> Path64 {
        self.polygon.clone()
    }
}
