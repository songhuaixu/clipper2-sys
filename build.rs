use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process::{Command, Stdio};

macro_rules! logMe {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    };
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let clipper2 = manifest_dir.join("Clipper2/CPP/Clipper2Lib");

    println!("cargo:rerun-if-changed=Clipper2/CPP/Clipper2Lib");
    println!("cargo:rerun-if-changed=cpp/clipper2_sys_bridge.hpp");
    println!("cargo:rerun-if-changed=cpp/clipper2_sys_bridge.cpp");
    println!("cargo:rerun-if-changed=src/cxx_bridge.rs");

    let extra_dir = if is_alpine_linux() {
        Some(alpine_includes())
    } else {
        None
    };

    let mut build = cxx_build::bridge("src/cxx_bridge.rs");
    build
        .opt_level(3)
        .include(&manifest_dir)
        .include(&clipper2.join("include"))
        .file(clipper2.join("src/clipper.engine.cpp"))
        .file(clipper2.join("src/clipper.offset.cpp"))
        .file(clipper2.join("src/clipper.rectclip.cpp"))
        .file(manifest_dir.join("cpp/clipper2_sys_bridge.cpp"))
        .flag_if_supported("-std:c++17")
        .flag_if_supported("-std=c++17");

    if let Some(dirs) = extra_dir {
        build.includes(dirs);
    }

    build.compile("clipper2");

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
    use std::path::Path;
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
