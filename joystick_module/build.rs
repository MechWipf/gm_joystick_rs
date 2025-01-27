
fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/lib.cc")
        .include("include/")
        .std("c++14")
        .compile("joystick_module");

    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rerun-if-changed=src/lib.cc");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
