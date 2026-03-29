# clipper2-sys

Rust bindings for **[Clipper2](https://github.com/AngusJohnson/Clipper2)** (C++), via [`cxx`](https://crates.io/crates/cxx): integer `Path64` / floating `PathD`, boolean clipping, offsetting, simplify, Minkowski, and lazy consumption of C++ path blobs.

---

## 简介（中文）

本 crate 在 Rust 中封装 Clipper2 C++ 库：提供整数坐标 **Clipper64** 与双精度 **ClipperD**、路径偏移 **ClipperOffset**、以及与 C++ 侧一致的扁平路径缓冲 **`PathsBlob*`** 互转。多边形的树形结果通过 **`PolyCxxPreorderIter*`** 前序遍历消费，避免在 Rust 侧拷贝整棵树。

---

## Requirements / 构建要求

- **Rust** 2021 edition  
- **C++17** toolchain (`clang++` / `g++` / MSVC) for `cxx` and bundled Clipper2 sources  
- On Linux you may need `libstdc++`; on macOS, `libc++`

## Build / 编译

```bash
cargo build
cargo test
```

Clipper2 C++ sources are compiled by `build.rs` together with `cpp/clipper2_sys_bridge.cpp`.

---

## Examples cookbook / 示例合集

### `Clipper64`: union + lazy closed / 布尔并 + 惰性闭合解

```rust
use clipper2_sys::{
    ClipType, Clipper64, FillRule, Paths64, Point64,
};
# use clipper2_sys::Path64;
# fn square_i(x0: i64, y0: i64, s: i64) -> Path64 {
#     Path64::new(vec![Point64::new(x0, y0), Point64::new(x0 + s, y0),
#         Point64::new(x0 + s, y0 + s), Point64::new(x0, y0 + s)])
# }

let mut clip = Clipper64::new();
clip.add_subject(&Paths64::new(vec![square_i(0, 0, 100)]));
clip.add_clip(&Paths64::new(vec![square_i(50, 50, 100)]));
let sol = clip.execute(ClipType::Union, FillRule::NonZero);
let (closed, _open) = sol.into_lazy();
assert!(!closed.is_empty());
```

### `Clipper64`: iterate closed paths / 逐条遍历闭合路径

```rust
use clipper2_sys::{
    ClipType, Clipper64, FillRule, Path64, Paths64, Point64,
};
# fn square_i(x0: i64, y0: i64, s: i64) -> Path64 {
#     Path64::new(vec![Point64::new(x0, y0), Point64::new(x0 + s, y0),
#         Point64::new(x0 + s, y0 + s), Point64::new(x0, y0 + s)])
# }

let mut clip = Clipper64::new();
clip.add_subject(&Paths64::new(vec![square_i(0, 0, 100)]));
clip.add_clip(&Paths64::new(vec![square_i(50, 50, 100)]));
let sol = clip.execute(ClipType::Union, FillRule::NonZero);
let all: Paths64 = sol.iter_closed().chain(sol.iter_open()).collect();
assert!(!all.is_empty());
```

### `Clipper64`: `execute_tree` + preorder on `PolyPath` / 树形解与前序遍历

```rust
use clipper2_sys::{
    ClipType, Clipper64, FillRule, Path64, Paths64, Point64,
};
# fn square_i(x0: i64, y0: i64, s: i64) -> Path64 {
#     Path64::new(vec![Point64::new(x0, y0), Point64::new(x0 + s, y0),
#         Point64::new(x0 + s, y0 + s), Point64::new(x0, y0 + s)])
# }

let mut clip = Clipper64::new();
clip.add_subject(&Paths64::new(vec![square_i(0, 0, 100)]));
clip.add_clip(&Paths64::new(vec![square_i(50, 50, 100)]));
let sol = clip.execute_tree(ClipType::Union, FillRule::NonZero);
let (_open_lazy, preorder) = sol.into_open_and_poly_preorder();
let n = preorder.count();
assert!(n > 0);
```

### `ClipperD`: union / 双精度布尔并

```rust
use clipper2_sys::{
    ClipType, ClipperD, FillRule, PathD, PathsD, PointD,
};
# fn square_f(x0: f64, y0: f64, s: f64) -> PathD {
#     PathD::new(vec![PointD::new(x0, y0), PointD::new(x0 + s, y0),
#         PointD::new(x0 + s, y0 + s), PointD::new(x0, y0 + s)])
# }

let mut clip = ClipperD::new(4);
clip.add_subject(&PathsD::new(vec![square_f(0.0, 0.0, 100.0)]));
clip.add_clip(&PathsD::new(vec![square_f(50.0, 50.0, 100.0)]));
let sol = clip.execute(ClipType::Union, FillRule::NonZero);
let (closed, _open) = sol.into_lazy();
assert!(!closed.is_empty());
```

### `ClipperOffset`: inflate a square / 方形外扩

```rust
use clipper2_sys::{ClipperOffset, EndType, JoinType, Path64, Point64};

let path = Path64::new(vec![
    Point64::new(0, 0),
    Point64::new(100, 0),
    Point64::new(100, 100),
    Point64::new(0, 100),
]);
let mut co = ClipperOffset::new(2.0, 0.0, false, false);
co.add_path(&path, JoinType::MiterJoin, EndType::PolygonEnd);
let out = co.execute(10.0);
assert!(!out.is_empty());
```

### `Path64`: area, point-in-polygon, translate, simplify / 面积、点包含、平移、简化

```rust
use clipper2_sys::{Path64, Point64, PointInPolygonResult};

let p = Path64::new(vec![
    Point64::new(0, 0),
    Point64::new(10, 0),
    Point64::new(10, 10),
    Point64::new(0, 10),
]);
assert!((p.area().abs() - 100.0).abs() < 1e-3);
assert!(matches!(
    p.point_in_polygon(Point64::new(5, 5)),
    PointInPolygonResult::Inside
));
let t = p.translate(3, -2);
assert_eq!(t.get_point(0).x, 3);
let collinear = Path64::new(vec![
    Point64::new(0, 0),
    Point64::new(5, 0),
    Point64::new(10, 0),
    Point64::new(10, 10),
]);
let simp = collinear.simplify(1.0, false);
assert!(simp.into_first_path().len() <= collinear.len());
```

### `Paths64`: inflate (offset helper) / 多路径偏移

```rust
use clipper2_sys::{EndType, JoinType, Path64, Paths64, Point64};

let paths = Paths64::new(vec![Path64::new(vec![
    Point64::new(0, 0),
    Point64::new(100, 0),
    Point64::new(100, 100),
    Point64::new(0, 100),
])]);
let grown = paths.inflate(10.0, JoinType::MiterJoin, EndType::PolygonEnd, 2.0);
assert!(!grown.is_empty());
```

### `Path64` / `Paths64`: Minkowski sum / 闵可夫斯基和

```rust
use clipper2_sys::{FillRule, Path64, Paths64, Point64};
# fn square_i(x0: i64, y0: i64, s: i64) -> Path64 {
#     Path64::new(vec![Point64::new(x0, y0), Point64::new(x0 + s, y0),
#         Point64::new(x0 + s, y0 + s), Point64::new(x0, y0 + s)])
# }

let a = square_i(0, 0, 50);
let b = square_i(0, 0, 30);
let ms = a.minkowski_sum(&b, true);
assert!(!ms.is_empty());

let many = Paths64::new(vec![square_i(0, 0, 40)]);
let ms2 = many.minkowski_sum(&b, true, FillRule::NonZero.into());
assert!(!ms2.is_empty());
```

### `PathD` ↔ `Path64`: simplify and convert / 双精度简化与转整型

```rust
use clipper2_sys::{LazyPaths64, PathD, PointD};

let p = PathD::new(vec![
    PointD::new(0.0, 0.0),
    PointD::new(5.0, 0.0),
    PointD::new(10.0, 0.0),
    PointD::new(10.0, 10.0),
]);
let simplified = p.simplify(1.0, false);
let _: LazyPaths64 = p.to_path64();
assert!(simplified.into_first_path().len() <= p.len());
```

### `ClipSolution`: `to_closed` / 物化全部闭合多边形

```rust
use clipper2_sys::{
    ClipType, Clipper64, FillRule, Path64, Paths64, Point64,
};
# fn square_i(x0: i64, y0: i64, s: i64) -> Path64 {
#     Path64::new(vec![Point64::new(x0, y0), Point64::new(x0 + s, y0),
#         Point64::new(x0 + s, y0 + s), Point64::new(x0, y0 + s)])
# }

let mut clip = Clipper64::new();
clip.add_subject(&Paths64::new(vec![square_i(0, 0, 100)]));
clip.add_clip(&Paths64::new(vec![square_i(50, 50, 100)]));
let sol = clip.execute(ClipType::Union, FillRule::NonZero);
let closed: Paths64 = sol.to_closed();
assert!(!closed.is_empty());
```

### `ClipTreeSolution`: open-only shortcut / 只要开放解

```rust
use clipper2_sys::{
    ClipType, Clipper64, FillRule, Path64, Paths64, Point64,
};
# fn square_i(x0: i64, y0: i64, s: i64) -> Path64 {
#     Path64::new(vec![Point64::new(x0, y0), Point64::new(x0 + s, y0),
#         Point64::new(x0 + s, y0 + s), Point64::new(x0, y0 + s)])
# }

let mut clip = Clipper64::new();
clip.add_subject(&Paths64::new(vec![square_i(0, 0, 100)]));
clip.add_clip(&Paths64::new(vec![square_i(50, 50, 100)]));
let sol = clip.execute_tree(ClipType::Union, FillRule::NonZero);
let _open = sol.into_open_lazy();
```

### Module map / 模块结构

- **`clipper64`** (`src/clipper64/`, types re-exported at crate root) — integer paths and **`Clipper64`**. / 整数路径与 **`Clipper64`**。
- **`clipperd`** — double paths and **`ClipperD`**. / 双精度路径与 **`ClipperD`**。
- **`ClipperOffset`** — path offset (inflate/deflate). / 路径偏移。

---

## Crate layout / 源代码布局

| Path | Role |
|------|------|
| `src/lib.rs` | Shared enums (`FillRule`, `ClipType`, …), re-exports / 公共枚举与再导出 |
| `src/cxx_bridge.rs` | `cxx::bridge` shared types and extern C++ API / cxx 共享类型与 FFI |
| `src/paths_blob.rs` | `PathsBlob64`/`PathsBlobD` ↔ `Path*` / 扁平路径缓冲转换 |
| `src/clipper64/` | `Point64`, `Path64`, `Paths64`, `Clipper64`, … / 整数流水线 |
| `src/clipperd/` | `PointD`, `PathD`, `PathsD`, `ClipperD`, … / 双精度流水线 |
| `src/offset.rs` | `ClipperOffset` (inflate/deflate on `Path64`) / 路径偏移 |
| `src/poly_path.rs` | Preorder iterators over C++ `PolyPath*` / C++ 多边形树前序迭代 |
| `src/macros.rs` | Internal `macro_rules!` for shared path logic / 内部宏 |
| `cpp/` | C++ bridge implementation / C++ 桥接实现 |

---

## Related / 相关项目

- Upstream: [Clipper2](https://github.com/AngusJohnson/Clipper2) (C++)  
- Prior art: [clipper-sys](https://crates.io/crates/clipper-sys) (Clipper1), [clipper2c](https://github.com/geoffder/clipper2c) (C layer)

## License

MIT — see upstream Clipper2 for its license. Clipper2 上游许可证请参阅其仓库。
