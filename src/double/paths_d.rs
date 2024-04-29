use crate::PathD;

#[derive(Clone, Debug)]
pub struct PathsD(pub(crate) Vec<PathD>);

impl PathsD {
    pub fn new(paths: &Vec<PathD>) -> Self {
        Self(paths.clone())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_path(&self, index: usize) -> PathD {
        self.0[index].clone()
    }

    pub fn add_path(&mut self, path: PathD) {
        self.0.push(path)
    }

    pub fn add_paths(&mut self, mut paths: Vec<PathD>) {
        self.0.append(&mut paths)
    }

    pub fn get_paths(&self) -> Vec<PathD> {
        self.0.clone()
    }

    pub fn translate(&self, dx: f64, dy: f64) -> Self {
        let new_paths = self.0.iter().map(|p| p.translate(dx, dy)).collect();
        Self(new_paths)
    }

    pub fn scale(&self, sx: f64, sy: f64) -> Self {
        let new_paths = self.0.iter().map(|p| p.scale(sx, sy)).collect();
        Self(new_paths)
    }
}
