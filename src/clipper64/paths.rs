use crate::cxx_bridge::clipper2_sys_cxx::PathsBlob64;
use crate::paths_blob::{paths64_from_blob, PathsBlob64Iter};
use crate::Path64;

define_lazy_paths! {
    /// Lazy view of multiple `Path64` from a C++ `PathsBlob64` (one path per `next()`).
    ///
    /// 来自 C++ `PathsBlob64` 的多条 [`Path64`] 惰性视图（每次 `next` 构造一条路径）。
    LazyPaths64,
    blob = PathsBlob64,
    paths = Paths64,
    path = Path64,
    iter = PathsBlob64Iter,
    from_blob = paths64_from_blob,
}

/// Owned collection of closed/open paths in integer space.
///
/// 整数空间下的多条路径集合。
#[derive(Clone, Debug, Default)]
pub struct Paths64(pub(crate) Vec<Path64>);

impl_paths_collection!(int64, Paths64, Path64);

#[cfg(test)]
mod tests {
    use crate::{Path64, Paths64, Point64};

    #[test]
    fn new_add_path_len() {
        let mut ps = Paths64::default();
        assert!(ps.is_empty());
        ps.add_path(Path64::new(vec![Point64::new(0, 0), Point64::new(1, 0)]));
        assert_eq!(ps.len(), 1);
        let t = ps.translate(10, 0);
        let q = t.path(0).get_point(0);
        assert_eq!(q.x, 10);
        assert_eq!(q.y, 0);
    }
}
