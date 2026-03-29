#include "clipper2-sys/src/cxx_bridge.rs.h"

#include <cmath>
#include <cstring>
#include <memory>
#include <utility>

#include "clipper2/clipper.minkowski.h"

using namespace Clipper2Lib;

// `cxx::bridge` 的 P64/PD 与 Clipper2 `Point64`/`PointD` 内存布局一致（T x; T y;）。
static_assert(sizeof(P64) == sizeof(Point64), "P64 must match Point64 layout for memcpy");
static_assert(sizeof(PD) == sizeof(PointD), "PD must match PointD layout for memcpy");

static Paths64 paths_from_blob(const PathsBlob64 &b) {
  Paths64 out;
  if (b.path_starts.size() < 2) {
    return out;
  }
  const std::size_t n_paths = b.path_starts.size() - 1;
  out.reserve(n_paths);
  for (std::size_t i = 0; i < n_paths; ++i) {
    std::size_t a = b.path_starts[i];
    std::size_t e = b.path_starts[i + 1];
    Path64 p;
    const std::size_t len = e - a;
    if (len != 0) {
      p.resize(len);
      std::memcpy(p.data(), &b.points[a], len * sizeof(P64));
    }
    out.push_back(std::move(p));
  }
  return out;
}

static PathsD paths_d_from_blob(const PathsBlobD &b) {
  PathsD out;
  if (b.path_starts.size() < 2) {
    return out;
  }
  const std::size_t n_paths = b.path_starts.size() - 1;
  out.reserve(n_paths);
  for (std::size_t i = 0; i < n_paths; ++i) {
    std::size_t a = b.path_starts[i];
    std::size_t e = b.path_starts[i + 1];
    PathD p;
    const std::size_t len = e - a;
    if (len != 0) {
      p.resize(len);
      std::memcpy(p.data(), &b.points[a], len * sizeof(PD));
    }
    out.push_back(std::move(p));
  }
  return out;
}

static PathsBlob64 blob_from_paths(const Paths64 &ps) {
  PathsBlob64 b;
  std::size_t total_pts = 0;
  for (const auto &path : ps) {
    total_pts += path.size();
  }
  b.points.reserve(total_pts);
  b.path_starts.reserve(ps.size() + 1);
  b.path_starts.push_back(0);
  for (const auto &path : ps) {
    const std::size_t len = path.size();
    if (len != 0) {
      const Point64 *src = path.data();
      for (std::size_t j = 0; j < len; ++j) {
        P64 q;
        std::memcpy(&q, src + j, sizeof(P64));
        b.points.push_back(q);
      }
    }
    b.path_starts.push_back(b.points.size());
  }
  return b;
}

static PathsBlobD blob_from_paths_d(const PathsD &ps) {
  PathsBlobD b;
  std::size_t total_pts = 0;
  for (const auto &path : ps) {
    total_pts += path.size();
  }
  b.points.reserve(total_pts);
  b.path_starts.reserve(ps.size() + 1);
  b.path_starts.push_back(0);
  for (const auto &path : ps) {
    const std::size_t len = path.size();
    if (len != 0) {
      const PointD *src = path.data();
      for (std::size_t j = 0; j < len; ++j) {
        PD q;
        std::memcpy(&q, src + j, sizeof(PD));
        b.points.push_back(q);
      }
    }
    b.path_starts.push_back(b.points.size());
  }
  return b;
}

static PathsBlob64 blob_single_path64(const Path64 &path) {
  return blob_from_paths(Paths64{path});
}

static PathsBlobD blob_single_path_d(const PathD &path) {
  return blob_from_paths_d(PathsD{path});
}

static JoinType join_from_u32(std::uint32_t j) {
  switch (j) {
    case 0:
      return JoinType::Square;
    case 1:
      return JoinType::Round;
    case 2:
      return JoinType::Miter;
    default:
      return JoinType::Miter;
  }
}

static EndType end_from_u32(std::uint32_t e) {
  if (e <= 4) {
    return static_cast<EndType>(e);
  }
  return EndType::Polygon;
}

static PolyPath64 *P64ptr(std::size_t x) { return reinterpret_cast<PolyPath64 *>(x); }
static PolyPathD *PDptr(std::size_t x) { return reinterpret_cast<PolyPathD *>(x); }

std::unique_ptr<Clipper64Box> cxx_clipper64_new() {
  return std::make_unique<Clipper64Box>();
}

void cxx_clipper64_set_preserve_collinear(Clipper64Box &c, bool v) { c.inner.PreserveCollinear(v); }

bool cxx_clipper64_get_preserve_collinear(const Clipper64Box &c) {
  return c.inner.PreserveCollinear();
}

void cxx_clipper64_set_reverse_solution(Clipper64Box &c, bool v) { c.inner.ReverseSolution(v); }

bool cxx_clipper64_get_reverse_solution(const Clipper64Box &c) {
  return c.inner.ReverseSolution();
}

void cxx_clipper64_clear(Clipper64Box &c) { c.inner.Clear(); }

void cxx_clipper64_add_subject(Clipper64Box &c, const PathsBlob64 &paths) {
  c.inner.AddSubject(paths_from_blob(paths));
}

void cxx_clipper64_add_open_subject(Clipper64Box &c, const PathsBlob64 &paths) {
  c.inner.AddOpenSubject(paths_from_blob(paths));
}

void cxx_clipper64_add_clip(Clipper64Box &c, const PathsBlob64 &paths) {
  c.inner.AddClip(paths_from_blob(paths));
}

Exec64 cxx_clipper64_execute(Clipper64Box &c, ClipperClipType ct, ClipperFillRule fr) {
  Paths64 closed, open;
  c.inner.Execute(static_cast<ClipType>(static_cast<std::uint32_t>(ct)),
                  static_cast<FillRule>(static_cast<std::uint32_t>(fr)), closed, open);
  Exec64 r;
  r.closed = blob_from_paths(closed);
  r.open = blob_from_paths(open);
  return r;
}

TreeExec64 cxx_clipper64_execute_tree(Clipper64Box &c, ClipperClipType ct, ClipperFillRule fr) {
  auto root = std::make_unique<PolyPath64>();
  Paths64 open;
  c.inner.Execute(static_cast<ClipType>(static_cast<std::uint32_t>(ct)),
                  static_cast<FillRule>(static_cast<std::uint32_t>(fr)), *root, open);
  TreeExec64 r;
  r.root = reinterpret_cast<std::size_t>(root.release());
  r.open = blob_from_paths(open);
  return r;
}

std::unique_ptr<ClipperDBox> cxx_clipperd_new(std::int32_t precision) {
  return std::make_unique<ClipperDBox>(precision);
}

void cxx_clipperd_set_preserve_collinear(ClipperDBox &c, bool v) { c.inner.PreserveCollinear(v); }

bool cxx_clipperd_get_preserve_collinear(const ClipperDBox &c) {
  return c.inner.PreserveCollinear();
}

void cxx_clipperd_set_reverse_solution(ClipperDBox &c, bool v) { c.inner.ReverseSolution(v); }

bool cxx_clipperd_get_reverse_solution(const ClipperDBox &c) {
  return c.inner.ReverseSolution();
}

void cxx_clipperd_clear(ClipperDBox &c) { c.inner.Clear(); }

void cxx_clipperd_add_subject(ClipperDBox &c, const PathsBlobD &paths) {
  c.inner.AddSubject(paths_d_from_blob(paths));
}

void cxx_clipperd_add_open_subject(ClipperDBox &c, const PathsBlobD &paths) {
  c.inner.AddOpenSubject(paths_d_from_blob(paths));
}

void cxx_clipperd_add_clip(ClipperDBox &c, const PathsBlobD &paths) {
  c.inner.AddClip(paths_d_from_blob(paths));
}

ExecD cxx_clipperd_execute(ClipperDBox &c, ClipperClipType ct, ClipperFillRule fr) {
  PathsD closed, open;
  c.inner.Execute(static_cast<ClipType>(static_cast<std::uint32_t>(ct)),
                  static_cast<FillRule>(static_cast<std::uint32_t>(fr)), closed, open);
  ExecD r;
  r.closed = blob_from_paths_d(closed);
  r.open = blob_from_paths_d(open);
  return r;
}

TreeExecD cxx_clipperd_execute_tree(ClipperDBox &c, ClipperClipType ct, ClipperFillRule fr) {
  auto root = std::make_unique<PolyPathD>();
  PathsD open;
  c.inner.Execute(static_cast<ClipType>(static_cast<std::uint32_t>(ct)),
                  static_cast<FillRule>(static_cast<std::uint32_t>(fr)), *root, open);
  TreeExecD r;
  r.root = reinterpret_cast<std::size_t>(root.release());
  r.open = blob_from_paths_d(open);
  return r;
}

std::unique_ptr<ClipperOffsetBox> cxx_clipper_offset_new(double miter_limit, double arc_tolerance,
                                                         bool preserve_collinear,
                                                         bool reverse_solution) {
  return std::make_unique<ClipperOffsetBox>(miter_limit, arc_tolerance, preserve_collinear,
                                             reverse_solution);
}

void cxx_clipper_offset_set_miter_limit(ClipperOffsetBox &c, double v) { c.inner.MiterLimit(v); }

double cxx_clipper_offset_get_miter_limit(const ClipperOffsetBox &c) {
  return c.inner.MiterLimit();
}

void cxx_clipper_offset_set_arc_tolerance(ClipperOffsetBox &c, double v) {
  c.inner.ArcTolerance(v);
}

double cxx_clipper_offset_get_arc_tolerance(const ClipperOffsetBox &c) {
  return c.inner.ArcTolerance();
}

void cxx_clipper_offset_set_preserve_collinear(ClipperOffsetBox &c, bool v) {
  c.inner.PreserveCollinear(v);
}

bool cxx_clipper_offset_get_preserve_collinear(const ClipperOffsetBox &c) {
  return c.inner.PreserveCollinear();
}

void cxx_clipper_offset_set_reverse_solution(ClipperOffsetBox &c, bool v) {
  c.inner.ReverseSolution(v);
}

bool cxx_clipper_offset_get_reverse_solution(const ClipperOffsetBox &c) {
  return c.inner.ReverseSolution();
}

std::int32_t cxx_clipper_offset_error_code(const ClipperOffsetBox &c) {
  return c.inner.ErrorCode();
}

void cxx_clipper_offset_clear(ClipperOffsetBox &c) { c.inner.Clear(); }

void cxx_clipper_offset_add_path(ClipperOffsetBox &c, const PathsBlob64 &path, ClipperJoinType jt,
                                 ClipperEndType et) {
  Paths64 ps = paths_from_blob(path);
  if (!ps.empty()) {
    c.inner.AddPath(ps[0], join_from_u32(static_cast<std::uint32_t>(jt)),
                    end_from_u32(static_cast<std::uint32_t>(et)));
  }
}

void cxx_clipper_offset_add_paths(ClipperOffsetBox &c, const PathsBlob64 &paths, ClipperJoinType jt,
                                  ClipperEndType et) {
  c.inner.AddPaths(paths_from_blob(paths), join_from_u32(static_cast<std::uint32_t>(jt)),
                   end_from_u32(static_cast<std::uint32_t>(et)));
}

PathsBlob64 cxx_clipper_offset_execute(ClipperOffsetBox &c, double delta) {
  Paths64 sol;
  c.inner.Execute(delta, sol);
  return blob_from_paths(sol);
}

PathsBlob64 cxx_path64_simplify(const PathsBlob64 &blob, double epsilon, bool is_open_path) {
  Paths64 ps = paths_from_blob(blob);
  if (ps.empty()) {
    return PathsBlob64{};
  }
  Path64 out = SimplifyPath(ps[0], epsilon, !is_open_path);
  return blob_single_path64(out);
}

PathsBlob64 cxx_paths64_simplify(const PathsBlob64 &blob, double epsilon, bool is_open_paths) {
  return blob_from_paths(SimplifyPaths(paths_from_blob(blob), epsilon, !is_open_paths));
}

PathsBlobD cxx_pathd_simplify(const PathsBlobD &blob, double epsilon, bool is_open_path) {
  PathsD ps = paths_d_from_blob(blob);
  if (ps.empty()) {
    return PathsBlobD{};
  }
  PathD out = SimplifyPath(ps[0], epsilon, !is_open_path);
  return blob_single_path_d(out);
}

PathsBlobD cxx_pathsd_simplify(const PathsBlobD &blob, double epsilon, bool is_open_paths) {
  return blob_from_paths_d(SimplifyPaths(paths_d_from_blob(blob), epsilon, !is_open_paths));
}

PathsBlobD cxx_path64_to_pathd(const PathsBlob64 &blob) {
  PathsD out;
  for (const auto &p : paths_from_blob(blob)) {
    PathD q;
    q.reserve(p.size());
    for (const auto &pt : p) {
      q.emplace_back(static_cast<double>(pt.x), static_cast<double>(pt.y));
    }
    out.push_back(std::move(q));
  }
  return blob_from_paths_d(out);
}

PathsBlob64 cxx_pathd_to_path64(const PathsBlobD &blob, std::int32_t precision) {
  int ec = 0;
  double scale = std::pow(10.0, static_cast<double>(precision));
  Paths64 out;
  for (const auto &p : paths_d_from_blob(blob)) {
    out.push_back(ScalePath<int64_t, double>(p, scale, ec));
  }
  return blob_from_paths(out);
}

PathsBlobD cxx_paths64_to_pathsd(const PathsBlob64 &blob) {
  PathsD out;
  int ec = 0;
  for (const auto &p : paths_from_blob(blob)) {
    PathD q;
    q.reserve(p.size());
    for (const auto &pt : p) {
      q.emplace_back(static_cast<double>(pt.x), static_cast<double>(pt.y));
    }
    out.push_back(std::move(q));
  }
  (void)ec;
  return blob_from_paths_d(out);
}

PathsBlob64 cxx_pathsd_to_paths64(const PathsBlobD &blob, std::int32_t precision) {
  int ec = 0;
  double scale = std::pow(10.0, static_cast<double>(precision));
  Paths64 out;
  for (const auto &p : paths_d_from_blob(blob)) {
    out.push_back(ScalePath<int64_t, double>(p, scale, ec));
  }
  return blob_from_paths(out);
}

ClipperPointInPolygonResult cxx_point_in_path64(const PathsBlob64 &blob, std::int64_t x,
                                                std::int64_t y) {
  Paths64 ps = paths_from_blob(blob);
  if (ps.empty()) {
    return static_cast<ClipperPointInPolygonResult>(2);
  }
  return static_cast<ClipperPointInPolygonResult>(
      static_cast<std::uint32_t>(PointInPolygon(Point64(x, y), ps[0])));
}

ClipperPointInPolygonResult cxx_point_in_pathd(const PathsBlobD &blob, double x, double y) {
  PathsD ps = paths_d_from_blob(blob);
  if (ps.empty()) {
    return static_cast<ClipperPointInPolygonResult>(2);
  }
  return static_cast<ClipperPointInPolygonResult>(
      static_cast<std::uint32_t>(PointInPolygon(PointD(x, y), ps[0])));
}

double cxx_path64_area(const PathsBlob64 &blob) {
  Paths64 ps = paths_from_blob(blob);
  if (ps.empty()) {
    return 0.0;
  }
  return Area<int64_t>(ps[0]);
}

double cxx_paths64_area(const PathsBlob64 &blob) { return Area<int64_t>(paths_from_blob(blob)); }

double cxx_pathd_area(const PathsBlobD &blob) {
  PathsD ps = paths_d_from_blob(blob);
  if (ps.empty()) {
    return 0.0;
  }
  return Area<double>(ps[0]);
}

double cxx_pathsd_area(const PathsBlobD &blob) { return Area<double>(paths_d_from_blob(blob)); }

PathsBlob64 cxx_path64_minkowski_sum(const PathsBlob64 &a, const PathsBlob64 &b, bool closed) {
  Paths64 pa = paths_from_blob(a);
  Paths64 pb = paths_from_blob(b);
  if (pa.empty() || pb.empty()) {
    return PathsBlob64{};
  }
  return blob_from_paths(MinkowskiSum(pa[0], pb[0], closed));
}

PathsBlob64 cxx_path64_minkowski_diff(const PathsBlob64 &a, const PathsBlob64 &b, bool closed) {
  Paths64 pa = paths_from_blob(a);
  Paths64 pb = paths_from_blob(b);
  if (pa.empty() || pb.empty()) {
    return PathsBlob64{};
  }
  return blob_from_paths(MinkowskiDiff(pa[0], pb[0], closed));
}

PathsBlob64 cxx_paths64_minkowski_sum(const PathsBlob64 &pattern, const PathsBlob64 &paths,
                                      bool closed, ClipperFillRule fillrule) {
  Paths64 pat = paths_from_blob(pattern);
  Paths64 pths = paths_from_blob(paths);
  if (pat.empty() || pths.empty()) {
    return PathsBlob64{};
  }
  const Path64 &fpat = pat[0];
  Paths64 acc;
  for (const auto &pth : pths) {
    Paths64 part = detail::Minkowski(fpat, pth, true, closed);
    acc.insert(acc.end(), part.begin(), part.end());
  }
  return blob_from_paths(
      detail::Union(acc, static_cast<FillRule>(static_cast<std::uint32_t>(fillrule))));
}

PathsBlob64 cxx_paths64_minkowski_diff(const PathsBlob64 &pattern, const PathsBlob64 &paths,
                                       bool closed, ClipperFillRule fillrule) {
  Paths64 pat = paths_from_blob(pattern);
  Paths64 pths = paths_from_blob(paths);
  if (pat.empty() || pths.empty()) {
    return PathsBlob64{};
  }
  const Path64 &fpat = pat[0];
  Paths64 acc;
  for (const auto &pth : pths) {
    Paths64 part = detail::Minkowski(fpat, pth, false, closed);
    acc.insert(acc.end(), part.begin(), part.end());
  }
  return blob_from_paths(
      detail::Union(acc, static_cast<FillRule>(static_cast<std::uint32_t>(fillrule))));
}

PathsBlobD cxx_pathd_minkowski_sum(const PathsBlobD &a, const PathsBlobD &b, bool closed,
                                   std::int32_t precision) {
  PathsD pa = paths_d_from_blob(a);
  PathsD pb = paths_d_from_blob(b);
  if (pa.empty() || pb.empty()) {
    return PathsBlobD{};
  }
  return blob_from_paths_d(MinkowskiSum(pa[0], pb[0], closed, precision));
}

PathsBlobD cxx_pathd_minkowski_diff(const PathsBlobD &a, const PathsBlobD &b, bool closed,
                                    std::int32_t precision) {
  PathsD pa = paths_d_from_blob(a);
  PathsD pb = paths_d_from_blob(b);
  if (pa.empty() || pb.empty()) {
    return PathsBlobD{};
  }
  return blob_from_paths_d(MinkowskiDiff(pa[0], pb[0], closed, precision));
}

PathsBlobD cxx_pathsd_minkowski_sum(const PathsBlobD &pattern, const PathsBlobD &paths, bool closed,
                                    std::int32_t precision, ClipperFillRule fillrule) {
  PathsD pat = paths_d_from_blob(pattern);
  PathsD pths = paths_d_from_blob(paths);
  if (pat.empty() || pths.empty()) {
    return PathsBlobD{};
  }
  const PathD &fpat = pat[0];
  int ec = 0;
  double scale = std::pow(10.0, static_cast<double>(precision));
  Path64 pat64 = ScalePath<int64_t, double>(fpat, scale, ec);
  Paths64 acc;
  for (const auto &pth : pths) {
    Path64 p64 = ScalePath<int64_t, double>(pth, scale, ec);
    Paths64 part = detail::Minkowski(pat64, p64, true, closed);
    acc.insert(acc.end(), part.begin(), part.end());
  }
  Paths64 united =
      detail::Union(acc, static_cast<FillRule>(static_cast<std::uint32_t>(fillrule)));
  return blob_from_paths_d(ScalePaths<double, int64_t>(united, 1 / scale, ec));
}

PathsBlobD cxx_pathsd_minkowski_diff(const PathsBlobD &pattern, const PathsBlobD &paths, bool closed,
                                     std::int32_t precision, ClipperFillRule fillrule) {
  PathsD pat = paths_d_from_blob(pattern);
  PathsD pths = paths_d_from_blob(paths);
  if (pat.empty() || pths.empty()) {
    return PathsBlobD{};
  }
  const PathD &fpat = pat[0];
  int ec = 0;
  double scale = std::pow(10.0, static_cast<double>(precision));
  Path64 pat64 = ScalePath<int64_t, double>(fpat, scale, ec);
  Paths64 acc;
  for (const auto &pth : pths) {
    Path64 p64 = ScalePath<int64_t, double>(pth, scale, ec);
    Paths64 part = detail::Minkowski(pat64, p64, false, closed);
    acc.insert(acc.end(), part.begin(), part.end());
  }
  Paths64 united =
      detail::Union(acc, static_cast<FillRule>(static_cast<std::uint32_t>(fillrule)));
  return blob_from_paths_d(ScalePaths<double, int64_t>(united, 1 / scale, ec));
}

PathsBlob64 cxx_paths64_inflate(const PathsBlob64 &paths, double delta, ClipperJoinType jt,
                                ClipperEndType et, double miter_limit) {
  return blob_from_paths(InflatePaths(paths_from_blob(paths), delta,
                                      join_from_u32(static_cast<std::uint32_t>(jt)),
                                      end_from_u32(static_cast<std::uint32_t>(et)), miter_limit,
                                      0.0));
}

PathsBlobD cxx_pathsd_inflate(const PathsBlobD &paths, double delta, ClipperJoinType jt,
                              ClipperEndType et, double miter_limit, std::int32_t precision) {
  return blob_from_paths_d(InflatePaths(
      paths_d_from_blob(paths), delta, join_from_u32(static_cast<std::uint32_t>(jt)),
      end_from_u32(static_cast<std::uint32_t>(et)), miter_limit, precision, 0.0));
}

bool cxx_poly64_is_hole(std::size_t p) { return P64ptr(p)->IsHole(); }

PathsBlob64 cxx_poly64_polygon(std::size_t p) {
  const Path64 &path = P64ptr(p)->Polygon();
  return blob_single_path64(path);
}

std::size_t cxx_poly64_child_count(std::size_t p) { return P64ptr(p)->Count(); }

std::size_t cxx_poly64_child_at(std::size_t p, std::size_t i) {
  return reinterpret_cast<std::size_t>(P64ptr(p)->Child(i));
}

void cxx_poly64_delete(std::size_t p) { delete P64ptr(p); }

double cxx_polyd_scale(std::size_t p) { return PDptr(p)->Scale(); }

bool cxx_polyd_is_hole(std::size_t p) { return PDptr(p)->IsHole(); }

PathsBlobD cxx_polyd_polygon(std::size_t p) {
  const PathD &path = PDptr(p)->Polygon();
  return blob_single_path_d(path);
}

std::size_t cxx_polyd_child_count(std::size_t p) { return PDptr(p)->Count(); }

std::size_t cxx_polyd_child_at(std::size_t p, std::size_t i) {
  return reinterpret_cast<std::size_t>(PDptr(p)->Child(i));
}

void cxx_polyd_delete(std::size_t p) { delete PDptr(p); }
