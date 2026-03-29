#pragma once

#include <cstdint>
#include <cstdlib>
#include <memory>

#include "clipper2/clipper.h"

// cxx 共享枚举（完整定义见生成的 cxx_bridge.rs.h）；此处仅前向声明供手写声明使用。
enum class ClipperClipType : std::uint32_t;
enum class ClipperFillRule : std::uint32_t;
enum class ClipperJoinType : std::uint32_t;
enum class ClipperEndType : std::uint32_t;
enum class ClipperPointInPolygonResult : std::uint32_t;

// --- cxx 在生成代码开头 include 本头文件：此处仅提供 Box 定义与 `::cxx_*` 声明。
//    共享结构体 P64、PathsBlob64 等由 cxx 生成；实现在 clipper2_sys_bridge.cpp 中（同翻译单元链接）。

struct Clipper64Box {
  Clipper2Lib::Clipper64 inner;
};

struct ClipperDBox {
  Clipper2Lib::ClipperD inner;
  explicit ClipperDBox(int precision) : inner(precision) {}
};

struct ClipperOffsetBox {
  Clipper2Lib::ClipperOffset inner;
  ClipperOffsetBox(double miter_limit, double arc_tolerance, bool preserve_collinear,
                   bool reverse_solution)
      : inner(miter_limit, arc_tolerance, preserve_collinear, reverse_solution) {}
};

inline std::uint32_t clipper2_sys_cxx_link_marker() noexcept { return 0x63707831u; }
inline std::size_t clipper2_sys_malloc(std::size_t n) noexcept {
  return reinterpret_cast<std::size_t>(std::malloc(n));
}
inline void clipper2_sys_free(std::size_t p) noexcept {
  std::free(reinterpret_cast<void *>(p));
}

struct PathsBlob64;
struct PathsBlobD;
struct Exec64;
struct ExecD;
struct TreeExec64;
struct TreeExecD;

std::unique_ptr<Clipper64Box> cxx_clipper64_new();
void cxx_clipper64_set_preserve_collinear(Clipper64Box &c, bool v);
bool cxx_clipper64_get_preserve_collinear(const Clipper64Box &c);
void cxx_clipper64_set_reverse_solution(Clipper64Box &c, bool v);
bool cxx_clipper64_get_reverse_solution(const Clipper64Box &c);
void cxx_clipper64_clear(Clipper64Box &c);
void cxx_clipper64_add_subject(Clipper64Box &c, const PathsBlob64 &paths);
void cxx_clipper64_add_open_subject(Clipper64Box &c, const PathsBlob64 &paths);
void cxx_clipper64_add_clip(Clipper64Box &c, const PathsBlob64 &paths);
Exec64 cxx_clipper64_execute(Clipper64Box &c, ClipperClipType ct, ClipperFillRule fr);
TreeExec64 cxx_clipper64_execute_tree(Clipper64Box &c, ClipperClipType ct, ClipperFillRule fr);

std::unique_ptr<ClipperDBox> cxx_clipperd_new(std::int32_t precision);
void cxx_clipperd_set_preserve_collinear(ClipperDBox &c, bool v);
bool cxx_clipperd_get_preserve_collinear(const ClipperDBox &c);
void cxx_clipperd_set_reverse_solution(ClipperDBox &c, bool v);
bool cxx_clipperd_get_reverse_solution(const ClipperDBox &c);
void cxx_clipperd_clear(ClipperDBox &c);
void cxx_clipperd_add_subject(ClipperDBox &c, const PathsBlobD &paths);
void cxx_clipperd_add_open_subject(ClipperDBox &c, const PathsBlobD &paths);
void cxx_clipperd_add_clip(ClipperDBox &c, const PathsBlobD &paths);
ExecD cxx_clipperd_execute(ClipperDBox &c, ClipperClipType ct, ClipperFillRule fr);
TreeExecD cxx_clipperd_execute_tree(ClipperDBox &c, ClipperClipType ct, ClipperFillRule fr);

std::unique_ptr<ClipperOffsetBox> cxx_clipper_offset_new(double miter_limit, double arc_tolerance,
                                                         bool preserve_collinear,
                                                         bool reverse_solution);
void cxx_clipper_offset_set_miter_limit(ClipperOffsetBox &c, double v);
double cxx_clipper_offset_get_miter_limit(const ClipperOffsetBox &c);
void cxx_clipper_offset_set_arc_tolerance(ClipperOffsetBox &c, double v);
double cxx_clipper_offset_get_arc_tolerance(const ClipperOffsetBox &c);
void cxx_clipper_offset_set_preserve_collinear(ClipperOffsetBox &c, bool v);
bool cxx_clipper_offset_get_preserve_collinear(const ClipperOffsetBox &c);
void cxx_clipper_offset_set_reverse_solution(ClipperOffsetBox &c, bool v);
bool cxx_clipper_offset_get_reverse_solution(const ClipperOffsetBox &c);
std::int32_t cxx_clipper_offset_error_code(const ClipperOffsetBox &c);
void cxx_clipper_offset_clear(ClipperOffsetBox &c);
void cxx_clipper_offset_add_path(ClipperOffsetBox &c, const PathsBlob64 &path, ClipperJoinType jt,
                                 ClipperEndType et);
void cxx_clipper_offset_add_paths(ClipperOffsetBox &c, const PathsBlob64 &paths, ClipperJoinType jt,
                                  ClipperEndType et);
PathsBlob64 cxx_clipper_offset_execute(ClipperOffsetBox &c, double delta);

PathsBlob64 cxx_path64_simplify(const PathsBlob64 &blob, double epsilon, bool is_open_path);
PathsBlob64 cxx_paths64_simplify(const PathsBlob64 &blob, double epsilon, bool is_open_paths);
PathsBlobD cxx_pathd_simplify(const PathsBlobD &blob, double epsilon, bool is_open_path);
PathsBlobD cxx_pathsd_simplify(const PathsBlobD &blob, double epsilon, bool is_open_paths);

PathsBlobD cxx_path64_to_pathd(const PathsBlob64 &blob);
PathsBlob64 cxx_pathd_to_path64(const PathsBlobD &blob, std::int32_t precision);
PathsBlobD cxx_paths64_to_pathsd(const PathsBlob64 &blob);
PathsBlob64 cxx_pathsd_to_paths64(const PathsBlobD &blob, std::int32_t precision);

ClipperPointInPolygonResult cxx_point_in_path64(const PathsBlob64 &blob, std::int64_t x,
                                                  std::int64_t y);
ClipperPointInPolygonResult cxx_point_in_pathd(const PathsBlobD &blob, double x, double y);

double cxx_path64_area(const PathsBlob64 &blob);
double cxx_paths64_area(const PathsBlob64 &blob);
double cxx_pathd_area(const PathsBlobD &blob);
double cxx_pathsd_area(const PathsBlobD &blob);

PathsBlob64 cxx_path64_minkowski_sum(const PathsBlob64 &a, const PathsBlob64 &b, bool closed);
PathsBlob64 cxx_path64_minkowski_diff(const PathsBlob64 &a, const PathsBlob64 &b, bool closed);
PathsBlob64 cxx_paths64_minkowski_sum(const PathsBlob64 &pattern, const PathsBlob64 &paths,
                                      bool closed, ClipperFillRule fillrule);
PathsBlob64 cxx_paths64_minkowski_diff(const PathsBlob64 &pattern, const PathsBlob64 &paths,
                                       bool closed, ClipperFillRule fillrule);

PathsBlobD cxx_pathd_minkowski_sum(const PathsBlobD &a, const PathsBlobD &b, bool closed,
                                   std::int32_t precision);
PathsBlobD cxx_pathd_minkowski_diff(const PathsBlobD &a, const PathsBlobD &b, bool closed,
                                    std::int32_t precision);
PathsBlobD cxx_pathsd_minkowski_sum(const PathsBlobD &pattern, const PathsBlobD &paths, bool closed,
                                    std::int32_t precision, ClipperFillRule fillrule);
PathsBlobD cxx_pathsd_minkowski_diff(const PathsBlobD &pattern, const PathsBlobD &paths, bool closed,
                                     std::int32_t precision, ClipperFillRule fillrule);

PathsBlob64 cxx_paths64_inflate(const PathsBlob64 &paths, double delta, ClipperJoinType jt,
                                ClipperEndType et, double miter_limit);
PathsBlobD cxx_pathsd_inflate(const PathsBlobD &paths, double delta, ClipperJoinType jt,
                              ClipperEndType et, double miter_limit, std::int32_t precision);

bool cxx_poly64_is_hole(std::size_t p);
PathsBlob64 cxx_poly64_polygon(std::size_t p);
std::size_t cxx_poly64_child_count(std::size_t p);
std::size_t cxx_poly64_child_at(std::size_t p, std::size_t i);
void cxx_poly64_delete(std::size_t p);

double cxx_polyd_scale(std::size_t p);
bool cxx_polyd_is_hole(std::size_t p);
PathsBlobD cxx_polyd_polygon(std::size_t p);
std::size_t cxx_polyd_child_count(std::size_t p);
std::size_t cxx_polyd_child_at(std::size_t p, std::size_t i);
void cxx_polyd_delete(std::size_t p);
