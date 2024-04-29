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

    pub fn translate(&self, dx: f64, dy: f64) -> Self {
        let new_points = self
            .0
            .iter()
            .map(|p| PointD {
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
            .map(|p| PointD {
                x: p.x * _sx,
                y: p.y * _sy,
            })
            .collect();
        Self(new_points)
    }
}
