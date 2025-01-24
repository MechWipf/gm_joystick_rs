fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/lib.cc")
        .include("include/")
        .std("c++14")
        .shared_flag(true)
        .compile("gmcl_joystick_linux64.so");

    println!("cargo:rerun-if-changed=src/lib.cc");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
