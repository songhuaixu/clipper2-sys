use crate::Path64;

#[derive(Clone, Debug)]
pub struct Paths64(pub(crate) Vec<Path64>);

impl Paths64 {
    pub fn new(paths: &Vec<Path64>) -> Self {
        Self(paths.clone())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_path(&self, index: usize) -> Path64 {
        self.0[index].clone()
    }

    pub fn add_path(&mut self, path: Path64) {
        self.0.push(path)
    }

    pub fn add_paths(&mut self, mut paths: Vec<Path64>) {
        self.0.append(&mut paths)
    }

    pub fn get_paths(&self) -> Vec<Path64> {
        self.0.clone()
    }

    pub fn translate(&self, dx: i64, dy: i64) -> Self {
        let new_paths = self.0.iter().map(|p| p.translate(dx, dy)).collect();
        Self(new_paths)
    }

    pub fn scale(&self, sx: f64, sy: f64) -> Self {
        let new_paths = self.0.iter().map(|p| p.scale(sx, sy)).collect();
        Self(new_paths)
    }
}
