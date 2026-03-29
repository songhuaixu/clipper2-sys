# clipper2-sys

Rust bindings for [Clipper2](https://github.com/AngusJohnson/Clipper2) (C++) using [`cxx`](https://crates.io/crates/cxx).

Provides integer `Path64` / `Paths64` with **Clipper64**, floating-point **PathD** / **PathsD** with **ClipperD**, **ClipperOffset**, boolean clipping, simplification, Minkowski operations, and lazy iteration over Clipper2 path blobs. Polygon hierarchy from `execute_tree` is consumed through **PolyCxxPreorderIter** preorder iterators without materializing a full Rust tree.

## 中文简介

本 crate 将 Clipper2 以 C++ 库形式接入 Rust：**Clipper64** 使用整数坐标，**ClipperD** 使用双精度坐标，并提供 **ClipperOffset** 做路径偏移。与 C++ 侧交换几何时使用扁平的 **PathsBlob** 缓冲；树形裁剪结果可通过 **PolyCxxPreorderIter** 前序遍历，避免在 Rust 中整棵拷贝多边形树。

## Features

- Boolean clip on `Clipper64` / `ClipperD` (`Union`, `Intersection`, `Difference`, `Xor`, fill rules).
- Offset / inflate helpers (`ClipperOffset`, `Paths64::inflate`).
- `Path64` / `PathD` helpers: area, point-in-polygon, simplify, translate, `PathD` → `Path64` conversion.
- Optional `execute_tree` + preorder iterator over the native `PolyPath` tree.

## Requirements

- **Rust** (edition 2021, as specified in `Cargo.toml`).
- **C++17** toolchain (`clang++`, `g++`, or MSVC) for `cxx` and the bundled Clipper2 sources.
- Linux: typically `libstdc++`; macOS: `libc++`.

## Building

```bash
cargo build
cargo test
```

Clipper2 sources are built from `build.rs` together with `cpp/clipper2_sys_bridge.cpp`.

## Examples

### Clipper64 — union and lazy closed solution

```rust
use clipper2_sys::{
    ClipType, Clipper64, FillRule, Path64, Paths64, Point64,
};

let rect = |x0: i64, y0: i64, w: i64| {
    Path64::new(vec![
        Point64::new(x0, y0),
        Point64::new(x0 + w, y0),
        Point64::new(x0 + w, y0 + w),
        Point64::new(x0, y0 + w),
    ])
};

let mut clip = Clipper64::new();
clip.add_subject(&Paths64::new(vec![rect(0, 0, 100)]));
clip.add_clip(&Paths64::new(vec![rect(50, 50, 100)]));
let sol = clip.execute(ClipType::Union, FillRule::NonZero);
let (closed, _open) = sol.into_lazy();
assert!(!closed.is_empty());
```

### Clipper64 — collect closed and open paths

```rust
use clipper2_sys::{
    ClipType, Clipper64, FillRule, Path64, Paths64, Point64,
};

let rect = |x0: i64, y0: i64, w: i64| {
    Path64::new(vec![
        Point64::new(x0, y0),
        Point64::new(x0 + w, y0),
        Point64::new(x0 + w, y0 + w),
        Point64::new(x0, y0 + w),
    ])
};

let mut clip = Clipper64::new();
clip.add_subject(&Paths64::new(vec![rect(0, 0, 100)]));
clip.add_clip(&Paths64::new(vec![rect(50, 50, 100)]));
let sol = clip.execute(ClipType::Union, FillRule::NonZero);
let all: Paths64 = sol.iter_closed().chain(sol.iter_open()).collect();
assert!(!all.is_empty());
```

### Clipper64 — `execute_tree` and `PolyPath` preorder

```rust
use clipper2_sys::{
    ClipType, Clipper64, FillRule, Path64, Paths64, Point64,
};

let rect = |x0: i64, y0: i64, w: i64| {
    Path64::new(vec![
        Point64::new(x0, y0),
        Point64::new(x0 + w, y0),
        Point64::new(x0 + w, y0 + w),
        Point64::new(x0, y0 + w),
    ])
};

let mut clip = Clipper64::new();
clip.add_subject(&Paths64::new(vec![rect(0, 0, 100)]));
clip.add_clip(&Paths64::new(vec![rect(50, 50, 100)]));
let sol = clip.execute_tree(ClipType::Union, FillRule::NonZero);
let (_open_lazy, preorder) = sol.into_open_and_poly_preorder();
let n = preorder.count();
assert!(n > 0);
```

### ClipperD — union

```rust
use clipper2_sys::{
    ClipType, ClipperD, FillRule, PathD, PathsD, PointD,
};

let rect = |x0: f64, y0: f64, w: f64| {
    PathD::new(vec![
        PointD::new(x0, y0),
        PointD::new(x0 + w, y0),
        PointD::new(x0 + w, y0 + w),
        PointD::new(x0, y0 + w),
    ])
};

let mut clip = ClipperD::new(4);
clip.add_subject(&PathsD::new(vec![rect(0.0, 0.0, 100.0)]));
clip.add_clip(&PathsD::new(vec![rect(50.0, 50.0, 100.0)]));
let sol = clip.execute(ClipType::Union, FillRule::NonZero);
let (closed, _open) = sol.into_lazy();
assert!(!closed.is_empty());
```

### ClipperOffset — inflate a square

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

### Path64 — area, point-in-polygon, translate, simplify

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

### Paths64 — inflate helper

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

### Path64 and Paths64 — Minkowski sum

```rust
use clipper2_sys::{FillRule, Path64, Paths64, Point64};

let rect = |x0: i64, y0: i64, w: i64| {
    Path64::new(vec![
        Point64::new(x0, y0),
        Point64::new(x0 + w, y0),
        Point64::new(x0 + w, y0 + w),
        Point64::new(x0, y0 + w),
    ])
};

let a = rect(0, 0, 50);
let b = rect(0, 0, 30);
let ms = a.minkowski_sum(&b, true);
assert!(!ms.is_empty());

let many = Paths64::new(vec![rect(0, 0, 40)]);
let ms2 = many.minkowski_sum(&b, true, FillRule::NonZero.into());
assert!(!ms2.is_empty());
```

### PathD — simplify and convert to Path64

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

### ClipSolution64 — materialize closed paths

```rust
use clipper2_sys::{
    ClipType, Clipper64, FillRule, Path64, Paths64, Point64,
};

let rect = |x0: i64, y0: i64, w: i64| {
    Path64::new(vec![
        Point64::new(x0, y0),
        Point64::new(x0 + w, y0),
        Point64::new(x0 + w, y0 + w),
        Point64::new(x0, y0 + w),
    ])
};

let mut clip = Clipper64::new();
clip.add_subject(&Paths64::new(vec![rect(0, 0, 100)]));
clip.add_clip(&Paths64::new(vec![rect(50, 50, 100)]));
let sol = clip.execute(ClipType::Union, FillRule::NonZero);
let closed: Paths64 = sol.to_closed();
assert!(!closed.is_empty());
```

### ClipTreeSolution64 — open paths only (drop polygon tree)

```rust
use clipper2_sys::{
    ClipType, Clipper64, FillRule, Path64, Paths64, Point64,
};

let rect = |x0: i64, y0: i64, w: i64| {
    Path64::new(vec![
        Point64::new(x0, y0),
        Point64::new(x0 + w, y0),
        Point64::new(x0 + w, y0 + w),
        Point64::new(x0, y0 + w),
    ])
};

let mut clip = Clipper64::new();
clip.add_subject(&Paths64::new(vec![rect(0, 0, 100)]));
clip.add_clip(&Paths64::new(vec![rect(50, 50, 100)]));
let sol = clip.execute_tree(ClipType::Union, FillRule::NonZero);
let _open = sol.into_open_lazy();
```

## Module overview

| Area | Contents |
|------|----------|
| **clipper64** (`src/clipper64/`) | `Point64`, `Path64`, `Paths64`, `Clipper64` (re-exported at crate root). |
| **clipperd** (`src/clipperd/`) | `PointD`, `PathD`, `PathsD`, `ClipperD`. |
| **offset** | `ClipperOffset` for path offset on `Path64`. |
| **poly_path** | `PolyCxxPreorderIter64` / `PolyCxxPreorderIterD` for native `PolyPath` preorder. |
| **paths_blob** | Conversions between `PathsBlob*` and Rust path types. |
| **cxx_bridge** | `cxx::bridge` definitions and FFI to `cpp/`. |

## Repository layout

| Path | Role |
|------|------|
| `src/lib.rs` | Crate root: shared enums, re-exports, documentation. |
| `src/cxx_bridge.rs` | `cxx::bridge` types and `extern "C++"` API. |
| `src/paths_blob.rs` | `PathsBlob64` / `PathsBlobD` ↔ `Path64` / `PathD`. |
| `src/clipper64/` | Integer coordinate pipeline. |
| `src/clipperd/` | Double-precision pipeline. |
| `src/offset.rs` | `ClipperOffset`. |
| `src/poly_path.rs` | Preorder iterators for C++ `PolyPath`. |
| `src/macros.rs` | Shared `macro_rules!` for paths and blobs. |
| `cpp/` | C++ bridge (`clipper2_sys_bridge`). |

## Related projects

- [Clipper2](https://github.com/AngusJohnson/Clipper2) (upstream C++).
- [clipper-sys](https://crates.io/crates/clipper-sys) (Clipper1 bindings).
- [clipper2c](https://github.com/geoffder/clipper2c) (C layer for Clipper2).

## License

This crate is licensed under the **MIT License** (see `LICENSE` in this repository).

The bundled **Clipper2** C++ library has its own license; refer to the [Clipper2 repository](https://github.com/AngusJohnson/Clipper2) for upstream terms.

本仓库中的 Rust 绑定以 MIT 授权；**Clipper2** 上游 C++ 库的许可请以官方仓库为准。
