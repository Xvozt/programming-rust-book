use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=LIBGIT2_LIB_DIR");
    let lib_dir =
        env::var("LIBGIT2_LIB_DIR").expect("Set LIBGIT2_LIB_DIR to directory containing libgit2");

    println!("cargo:rustc-link-search=native={lib_dir}");
    println!("cargo:rustc-link-lib=git2");

    // Make the executable find libgit2.dylib at runtime on macOS.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{lib_dir}");
    }
}
