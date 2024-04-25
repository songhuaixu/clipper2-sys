use crate::PointD;

#[derive(Clone, Debug)]
pub struct PathD(pub(crate) Vec<PointD>);

impl PathD {
    pub fn new(points: &Vec<PointD>) -> Self {
        Self(points.clone())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_point(&self, index: usize) -> PointD {
        self.0[index]
    }

    pub fn add_point(&mut self, point: PointD) {
        self.0.push(point)
    }
}