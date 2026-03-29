use crate::PointD;

/// Double-precision polyline / polygon (`PathD`).
///
/// 双精度折线或闭合轮廓（`PathD`）。
///
/// See [`crate::Path64`] for the parallel integer API. / 与 [`crate::Path64`] 为对偶关系。
#[derive(Clone, Debug, Default)]
pub struct PathD(pub(crate) Vec<PointD>);

impl_path_geom!(float, PathD, PointD);

#[cfg(test)]
mod tests {
    use crate::{PathD, PointD};

    #[test]
    fn new_translate_scale() {
        let p = PathD::new(vec![
            PointD::new(0.0, 0.0),
            PointD::new(1.0, 0.0),
            PointD::new(1.0, 1.0),
        ]);
        assert_eq!(p.len(), 3);
        let t = p.translate(2.5, -0.5);
        assert!((t.get_point(0).x - 2.5).abs() < 1e-9);
    }

    #[test]
    fn iter_sum_x() {
        let p = PathD::new(vec![PointD::new(1.0, 0.0), PointD::new(0.5, 1.0)]);
        let s: f64 = p.iter().map(|q| q.x).sum();
        assert!((s - 1.5).abs() < 1e-9);
        let n = (&p).into_iter().count();
        assert_eq!(n, 2);
    }
}
