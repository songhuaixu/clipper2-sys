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

    pub fn translate(&self, dx: i64, dy: i64) -> Self {
        let new_points = self
            .0
            .iter()
            .map(|p| Point64 {
                x: p.x + dx,
                y: p.y + dy,
            })
            .collect();
        Self(new_points)
    }

    pub fn scale(&self, sx: f64, sy: f64) -> Self {
        let mut _sx = sx;
        if _sx == 0. {
            _sx = 1.;
        }
        let mut _sy = sy;
        if _sy == 0. {
            _sy = 1.;
        }
        let new_points = self
            .0
            .iter()
            .map(|p| Point64 {
                x: ((p.x as f64) * sx) as i64,
                y: ((p.y as f64) * sy) as i64,
            })
            .collect();
        Self(new_points)
    }
}
