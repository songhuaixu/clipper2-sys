//! Preorder iterators over C++ `PolyPath64` / `PolyPathD` without building a Rust tree.
//!
//! 对 C++ 侧 `PolyPath64` / `PolyPathD` 做前序遍历；不在 Rust 侧物化树结构。
//! Owning iterators release the native root on `Drop`. / 拥有根指针，在 `Drop` 时释放 C++ 子树。

define_poly_cxx_preorder_64!();
define_poly_cxx_preorder_d!();

#[cfg(test)]
mod tests {
    use crate::{
        ClipType, Clipper64, ClipperD, FillRule, Path64, PathD, Paths64, PathsD, Point64, PointD,
    };

    fn square_i(x0: i64, y0: i64, s: i64) -> Path64 {
        Path64::new(vec![
            Point64::new(x0, y0),
            Point64::new(x0 + s, y0),
            Point64::new(x0 + s, y0 + s),
            Point64::new(x0, y0 + s),
        ])
    }

    fn square_f(x0: f64, y0: f64, s: f64) -> PathD {
        PathD::new(vec![
            PointD::new(x0, y0),
            PointD::new(x0 + s, y0),
            PointD::new(x0 + s, y0 + s),
            PointD::new(x0, y0 + s),
        ])
    }

    #[test]
    fn poly_preorder_64_root_depth_zero_when_tree_exists() {
        let mut c = Clipper64::new();
        c.add_subject(&Paths64::new(vec![square_i(0, 0, 100)]));
        c.add_clip(&Paths64::new(vec![square_i(50, 50, 100)]));
        let sol = c.execute_tree(ClipType::Union, FillRule::NonZero);
        if !sol.has_poly_tree() {
            return;
        }
        let (_, mut iter) = sol.into_open_and_poly_preorder();
        let root = iter.next().expect("non-empty poly tree");
        assert_eq!(root.depth, 0);
    }

    #[test]
    fn poly_preorder_d_root_depth_zero_when_tree_exists() {
        let mut c = ClipperD::new(4);
        c.add_subject(&PathsD::new(vec![square_f(0.0, 0.0, 100.0)]));
        c.add_clip(&PathsD::new(vec![square_f(50.0, 50.0, 100.0)]));
        let sol = c.execute_tree(ClipType::Union, FillRule::NonZero);
        if !sol.has_poly_tree() {
            return;
        }
        let (_, mut iter) = sol.into_open_and_poly_preorder();
        let root = iter.next().expect("non-empty poly tree");
        assert_eq!(root.depth, 0);
    }
}
