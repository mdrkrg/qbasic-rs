fn main() {
    cxx_build::bridge("src/ffi/mod.rs")
        .std("c++23")
        .compile("qbasic_rs");

    println!("cargo:rerun-if-changed=src/ffi.rs");
}
