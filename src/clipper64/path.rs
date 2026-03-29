use crate::Point64;

/// Integer grid polyline / polygon ring (`Path64`), backed by `Vec<Point64>`.
///
/// 整数坐标折线或闭合轮廓（`Path64`），内部为 `Vec<Point64>`。
///
/// # Geometry helpers / 几何辅组
///
/// - [`Path64::new`], [`Path64::from_vec`] — construct / 构造  
/// - [`Path64::translate`], [`Path64::scale`] — transform (integer deltas / float scale) / 平移与缩放  
/// - [`Path64::iter`], [`Path64::len`], [`Path64::get_point`] — access / 访问  
///
/// Clipper-related methods (`simplify`, `area`, …) are on the same `Path64` type in `clipper64/mod.rs`
/// (re-exported at the crate root).
///
/// 与 Clipper 相关的方法（`simplify`、`area` 等）在同一 `Path64` 上于 `clipper64/mod.rs` 中实现（并在 crate 根再导出）。
#[derive(Clone, Debug, Default)]
pub struct Path64(pub(crate) Vec<Point64>);

impl_path_geom!(int64, Path64, Point64);

#[cfg(test)]
mod tests {
    use crate::{Path64, Point64};

    #[test]
    fn new_translate_scale_len() {
        let p = Path64::new(vec![
            Point64::new(0, 0),
            Point64::new(10, 0),
            Point64::new(10, 10),
        ]);
        assert_eq!(p.len(), 3);
        let t = p.translate(5, -1);
        let p0 = t.get_point(0);
        assert_eq!(p0.x, 5);
        assert_eq!(p0.y, -1);
        let s = p.scale(2.0, 2.0);
        let p1 = s.get_point(1);
        assert_eq!(p1.x, 20);
        assert_eq!(p1.y, 0);
    }

    #[test]
    fn iter_and_for_in_by_ref() {
        let p = Path64::new(vec![Point64::new(1, 2), Point64::new(3, 4)]);
        let s: i64 = p.iter().map(|q| q.x + q.y).sum();
        assert_eq!(s, 10);
        let t: i64 = (&p).into_iter().map(|q| q.x).sum();
        assert_eq!(t, 4);
    }
}
