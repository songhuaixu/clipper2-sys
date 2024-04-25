use crate::Point64;

#[derive(Clone, Debug)]
pub struct Path64(pub(crate) Vec<Point64>);

impl Path64 {
    pub fn new(points: &Vec<Point64>) -> Self {
        Self(points.clone())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_point(&self, index: usize) -> Point64 {
        self.0[index]
    }

    pub fn add_point(&mut self, point: Point64) {
        self.0.push(point)
    }
}
