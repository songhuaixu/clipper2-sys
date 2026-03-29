use crate::cxx_bridge::clipper2_sys_cxx::PathsBlobD;
use crate::paths_blob::{pathsd_from_blob, PathsBlobDIter};
use crate::PathD;

define_lazy_paths! {
    /// Lazy iterators over `PathD` entries from a `PathsBlobD`.
    ///
    /// 由 `PathsBlobD` 惰性产出多条 [`PathD`]。
    LazyPathsD,
    blob = PathsBlobD,
    paths = PathsD,
    path = PathD,
    iter = PathsBlobDIter,
    from_blob = pathsd_from_blob,
}

/// Collection of double-precision paths.
///
/// 双精度路径集合。
#[derive(Clone, Debug, Default)]
pub struct PathsD(pub(crate) Vec<PathD>);

impl_paths_collection!(float, PathsD, PathD);

#[cfg(test)]
mod tests {
    use crate::{PathD, PathsD, PointD};

    #[test]
    fn paths_d_append_and_translate() {
        let mut ps = PathsD::default();
        ps.add_path(PathD::new(vec![PointD::new(0.0, 0.0), PointD::new(1.0, 0.0)]));
        let u = ps.translate(1.0, 2.0);
        assert!((u.path(0).get_point(0).x - 1.0).abs() < 1e-9);
    }
}
