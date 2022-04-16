use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Should be replaced with crate like `cc`.
    Command::new("clang++")
        .args(&[
            "-c",
            "-Wall",
            "-std=c++17",
            "-fno-objc-arc",
            &format!("-I{}/metal-cpp", manifest_dir),
            &format!("-I{}/metal-cpp-extensions", manifest_dir),
            &format!("{}/src/metal_cpp_rs_bindings.cpp", manifest_dir),
            &format!("-o{}/metal_cpp_rs_bindings.o", out_dir),
        ])
        .status()
        .unwrap();
    Command::new("ar")
        .args(&[
            "crus",
            "libmetal_cpp_rs_bindings.a",
            "metal_cpp_rs_bindings.o",
        ])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();

    println!("cargo:rustc-link-arg=-framework");
    println!("cargo:rustc-link-arg=Metal");
    println!("cargo:rustc-link-arg=-framework");
    println!("cargo:rustc-link-arg=Foundation");
    println!("cargo:rustc-link-arg=-framework");
    println!("cargo:rustc-link-arg=Cocoa");
    println!("cargo:rustc-link-arg=-framework");
    println!("cargo:rustc-link-arg=CoreGraphics");
    println!("cargo:rustc-link-arg=-framework");
    println!("cargo:rustc-link-arg=MetalKit");
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=metal_cpp_rs_bindings");
    println!("cargo:rerun-if-changed=src/metal_cpp_rs_bindings.cpp");
}
