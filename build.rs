// (c) Copyright 2019-2023 OLX
fn main() {
    if cfg!(Windows) {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", std::path::Path::new(&dir).join("lib").to_str().unwrap());
    println!("cargo:rustc-link-lib=libvips");
    println!("cargo:rustc-link-lib=libglib-2.0");
    println!("cargo:rustc-link-lib=libgobject-2.0");
    } else {
    println!("cargo:rustc-link-lib=vips");
    println!("cargo:rustc-link-lib=glib-2.0");
    println!("cargo:rustc-link-lib=gobject-2.0");
    }
}
