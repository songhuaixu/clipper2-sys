use std::env;
use std::process::{Command, Stdio};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path,PathBuf};

macro_rules! logMe {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    println!("cargo:rerun-if-changed=Clipper2");
    if cfg!(feature = "update-bindings") {
        println!("cargo:rerun-if-changed=generated");
    }

    let extra_dir = if is_alpine_linux() {
        Some(alpine_includes())
    } else {
        None
    };
    
    let mut builder = cc::Build::new();

    let mut build = builder.cpp(true)
        .opt_level(3)
        .include("clipper2c/vendor/Clipper2/CPP/Clipper2Lib/include/")
        // .include("clipper2c/vendor/Clipper2/CPP/Utils/")
        ;

        if let Some(dirs) = extra_dir {
            build = build.includes(dirs);
        }

        build
        .include("clipper2c/include")
        .include("clipper2c/src")
        .files([
            "clipper2c/vendor/Clipper2/CPP/Clipper2Lib/src/clipper.engine.cpp",
            "clipper2c/vendor/Clipper2/CPP/Clipper2Lib/src/clipper.offset.cpp",
            "clipper2c/vendor/Clipper2/CPP/Clipper2Lib/src/clipper.rectclip.cpp",
            "clipper2c/src/clipper2c.cpp",
            "clipper2c/src/clipper64.cpp",
            "clipper2c/src/clipperd.cpp",
            "clipper2c/src/clipperoffset.cpp",
            "clipper2c/src/conv.cpp",
            "clipper2c/src/polytree.cpp",
            "clipper2c/src/rect.cpp",
            // "clipper2c/vendor/Clipper2/CPP/Utils/clipper.svg.cpp",
            // "clipper2c/src/svg.cpp",
        ])
        .flag_if_supported("-std:c++17") // MSVC
        .flag_if_supported("-std=c++17")
        .compile("Clipper2");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    match (target_os.as_str(), target_env.as_str()) {
        ("linux", _) | ("windows", "gnu") | ("android", _) => {
            println!("cargo:rustc-link-lib=dylib=stdc++")
        }
        ("macos", _) | ("ios", _) => println!("cargo:rustc-link-lib=dylib=c++"),
        ("windows", "msvc") => {}
        _ => unimplemented!(
            "target_os: {}, target_env: {}",
            target_os.as_str(),
            target_env.as_str()
        ),
    }

    #[cfg(feature = "generate-bindings")]
    {
        let bindings = bindgen::Builder::default()
            .header("clipper2c/include/clipper2c.h")
            .header("clipper2c/include/types.h")
            .allowlist_type("ClipperClipperD")
            .allowlist_type("ClipperPathD")
            .allowlist_type("ClipperPathsD")
            .allowlist_type("ClipperPolyTreeD")
            .allowlist_type("ClipperPointD")
            .allowlist_type("ClipperFillRule")
            .allowlist_type("ClipperClipType")
            // PathD Methods
            .allowlist_function("clipper_pathd_size")
            .allowlist_function("clipper_pathd")
            .allowlist_function("clipper_pathd_of_points")
            .allowlist_function("clipper_pathd_add_point")
            .allowlist_function("clipper_pathd_length")
            .allowlist_function("clipper_pathd_get_point")
            .allowlist_function("clipper_delete_pathd")
            .allowlist_function("clipper_pathd_simplify")
            .allowlist_function("clipper_pathd_to_path64")
            .allowlist_function("clipper_point_in_pathd")
            // PathsD Methods
            .allowlist_function("clipper_pathsd_size")
            .allowlist_function("clipper_pathsd")
            .allowlist_function("clipper_pathsd_of_paths")
            .allowlist_function("clipper_pathsd_add_path")
            .allowlist_function("clipper_pathsd_add_paths")
            .allowlist_function("clipper_pathsd_get_path")
            .allowlist_function("clipper_pathsd_get_point")
            .allowlist_function("clipper_pathsd_length")
            .allowlist_function("clipper_pathsd_path_length")
            .allowlist_function("clipper_delete_pathsd")
            .allowlist_function("clipper_pathsd_simplify")
            .allowlist_function("clipper_pathsd_inflate")
            .allowlist_function("clipper_pathsd_to_paths64")
            // ClipperD Methods
            .allowlist_function("clipper_clipperd_size")
            .allowlist_function("clipper_clipperd")
            .allowlist_function("clipper_clipperd_add_subject")
            .allowlist_function("clipper_clipperd_add_open_subject")
            .allowlist_function("clipper_clipperd_add_clip")
            .allowlist_function("clipper_clipperd_execute")
            .allowlist_function("clipper_clipperd_execute_tree_with_open")
            .allowlist_function("clipper_clipperd_set_preserve_collinear")
            .allowlist_function("clipper_clipperd_get_preserve_collinear")
            .allowlist_function("clipper_clipperd_set_reverse_solution")
            .allowlist_function("clipper_clipperd_get_reverse_solution")
            .allowlist_function("clipper_clipperd_clear")
            .allowlist_function("clipper_delete_clipperd")
            // PolyTreeD Methods
            .allowlist_function("clipper_polytreed_size")
            .allowlist_function("clipper_polytreed")
            .allowlist_function("clipper_polytreed_parent")
            .allowlist_function("clipper_polytreed_count")
            .allowlist_function("clipper_polytreed_get_child")
            .allowlist_function("clipper_polytreed_set_inv_scale")
            .allowlist_function("clipper_polytreed_inv_scale")
            .allowlist_function("clipper_polytreed_add_child")
            .allowlist_function("clipper_polytreed_is_hole")
            .allowlist_function("clipper_polytreed_polygon")
            .allowlist_function("clipper_polytreed_area")
            .allowlist_function("clipper_polytreed_to_paths")
            .allowlist_function("clipper_delete_polytreed")
            // Path64 Methods
            .allowlist_function("clipper_path64_size")
            .allowlist_function("clipper_path64")
            .allowlist_function("clipper_path64_of_points")
            .allowlist_function("clipper_path64_add_point")
            .allowlist_function("clipper_path64_length")
            .allowlist_function("clipper_path64_get_point")
            .allowlist_function("clipper_delete_path64")
            .allowlist_function("clipper_path64_simplify")
            .allowlist_function("clipper_path64_to_pathd")
            .allowlist_function("clipper_point_in_path64")
            // Paths64 Methods
            .allowlist_function("clipper_paths64_size")
            .allowlist_function("clipper_paths64")
            .allowlist_function("clipper_paths64_of_paths")
            .allowlist_function("clipper_paths64_add_path")
            .allowlist_function("clipper_paths64_add_paths")
            .allowlist_function("clipper_paths64_get_path")
            .allowlist_function("clipper_paths64_get_point")
            .allowlist_function("clipper_paths64_length")
            .allowlist_function("clipper_paths64_path_length")
            .allowlist_function("clipper_delete_paths64")
            .allowlist_function("clipper_paths64_simplify")
            .allowlist_function("clipper_paths64_inflate")
            .allowlist_function("clipper_paths64_to_pathsd")
            // ClipperD Methods
            .allowlist_function("clipper_clipper64_size")
            .allowlist_function("clipper_clipper64")
            .allowlist_function("clipper_clipper64_add_subject")
            .allowlist_function("clipper_clipper64_add_open_subject")
            .allowlist_function("clipper_clipper64_add_clip")
            .allowlist_function("clipper_clipper64_execute")
            .allowlist_function("clipper_clipper64_execute_tree_with_open")
            .allowlist_function("clipper_clipper64_set_preserve_collinear")
            .allowlist_function("clipper_clipper64_get_preserve_collinear")
            .allowlist_function("clipper_clipper64_set_reverse_solution")
            .allowlist_function("clipper_clipper64_get_reverse_solution")
            .allowlist_function("clipper_clipper64_clear")
            .allowlist_function("clipper_delete_clipper64")
            // PolyTreeD Methods
            .allowlist_function("clipper_polytree64_size")
            .allowlist_function("clipper_polytree64")
            .allowlist_function("clipper_polytree64_parent")
            .allowlist_function("clipper_polytree64_count")
            .allowlist_function("clipper_polytree64_get_child")
            .allowlist_function("clipper_polytree64_set_inv_scale")
            .allowlist_function("clipper_polytree64_inv_scale")
            .allowlist_function("clipper_polytree64_add_child")
            .allowlist_function("clipper_polytree64_is_hole")
            .allowlist_function("clipper_polytree64_polygon")
            .allowlist_function("clipper_polytree64_area")
            .allowlist_function("clipper_polytree64_to_paths")
            .allowlist_function("clipper_delete_polytree64")
            // ClipperOffset Methods
            .allowlist_function("clipper_clipperoffset_size")
            .allowlist_function("clipper_clipperoffset")
            .allowlist_function("clipper_clipperoffset_set_miter_limit")
            .allowlist_function("clipper_clipperoffset_set_arc_tolerance")
            .allowlist_function("clipper_clipperoffset_set_preserve_collinear")
            .allowlist_function("clipper_clipperoffset_set_reverse_solution")
            .allowlist_function("clipper_clipperoffset_get_miter_limit")
            .allowlist_function("clipper_clipperoffset_get_arc_tolerance")
            .allowlist_function("clipper_clipperoffset_get_preserve_collinear")
            .allowlist_function("clipper_clipperoffset_get_reverse_solution")
            .allowlist_function("clipper_clipperoffset_error_code")
            .allowlist_function("clipper_clipperoffset_clear")
            .allowlist_function("clipper_clipperoffset_add_path64")
            .allowlist_function("clipper_clipperoffset_add_paths64")
            .allowlist_function("clipper_clipperoffset_execute")
            .allowlist_function("clipper_delete_clipperoffset")

            // Path64 Minkowski
            .allowlist_function("clipper_path64_minkowski_sum")
            .allowlist_function("clipper_path64_minkowski_diff")

            // Paths64 Minkowski
            .allowlist_function("clipper_paths64_minkowski_sum")
            .allowlist_function("clipper_paths64_minkowski_diff")

            // Pathd Minkowski
            .allowlist_function("clipper_pathd_minkowski_sum")
            .allowlist_function("clipper_pathd_minkowski_diff")

            // Pathsd Minkowski
            .allowlist_function("clipper_pathsd_minkowski_sum")
            .allowlist_function("clipper_pathsd_minkowski_diff")

            // Area
            .allowlist_function("clipper_pathd_area")
            .allowlist_function("clipper_path64_area")
            .allowlist_function("clipper_pathsd_area")
            .allowlist_function("clipper_paths64_area")

            .size_t_is_usize(true)
            .generate()
            .expect("unable to generate bindings");

        let out_path = if cfg!(feature = "update-bindings") {
            std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("generated")
        } else {
            std::path::PathBuf::from(env::var("OUT_DIR").unwrap())
        };

        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("couldn't write bindings!");
    }
}

fn get_stdlib_version() -> Option<String> {
    let mut cmd = Command::new("g++");
    cmd.arg("-dumpversion");
    let output = cmd.stderr(Stdio::inherit()).output().ok()?;
    if output.status.code() != Some(0) {
        return None;
    }
    match String::from_utf8(output.stdout).unwrap().trim() {
        "" => None,
        v => Some(String::from(v)),
    }
}

fn is_alpine_linux() -> bool {
    let path = Path::new("/etc/os-release");
    if let Ok(file) = File::open(path) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(content) = line {
                if content.trim() == "ID=alpine" {
                    return true;
                }
            }
        }
    }
    false
}

fn alpine_includes() -> Vec<PathBuf> {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match get_stdlib_version() {
        None => {
            logMe!("alpine: unable to determine g++ stdlib version");
            vec![]
        }
        Some(cpp) => vec![
            PathBuf::from(format!("/usr/include/c++/{cpp}")),
            PathBuf::from(format!("/usr/include/c++/{cpp}/{arch}-alpine-linux-musl")),
        ],
    }
}
