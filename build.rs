use std::env;

fn main() {
    println!("cargo:rerun-if-changed=Clipper2");
    if cfg!(feature = "update-bindings") {
        println!("cargo:rerun-if-changed=generated");
    }

    cc::Build::new()
        .cpp(true)
        .opt_level(3)
        .include("clipper2c/vendor/Clipper2/CPP/Clipper2Lib/include/")
        .include("clipper2c/vendor/Clipper2/CPP/Utils/")
        .include("clipper2c/include")
        .include("clipper2c/src")
        .files([
            "clipper2c/vendor/Clipper2/CPP/Clipper2Lib/src/clipper.engine.cpp",
            "clipper2c/vendor/Clipper2/CPP/Clipper2Lib/src/clipper.offset.cpp",
            "clipper2c/vendor/Clipper2/CPP/Clipper2Lib/src/clipper.rectclip.cpp",
            "clipper2c/vendor/Clipper2/CPP/Utils/clipper.svg.cpp",
            "clipper2c/src/clipper2c.cpp",
            "clipper2c/src/clipper64.cpp",
            "clipper2c/src/clipperd.cpp",
            "clipper2c/src/clipperoffset.cpp",
            "clipper2c/src/conv.cpp",
            "clipper2c/src/polytree.cpp",
            "clipper2c/src/rect.cpp",
            "clipper2c/src/svg.cpp",
            ])
        .flag_if_supported("-std=c++17")
        .compile("Clipper2");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    match (target_os.as_str(), target_env.as_str()) {
        ("linux", _) | ("windows", "gnu") |  ("android", _)  => println!("cargo:rustc-link-lib=dylib=stdc++"),
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

            // ClipperD Methods
            .allowlist_function("clipper_clipperd_size")
            .allowlist_function("clipper_clipperd")
            .allowlist_function("clipper_clipperd_add_subject")
            .allowlist_function("clipper_clipperd_add_open_subject")
            .allowlist_function("clipper_clipperd_add_clip")
            .allowlist_function("clipper_clipperd_execute")
            .allowlist_function("clipper_clipperd_execute_tree_with_open")
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