// (c) Copyright 2019-2023 OLX
fn main() {
    #[cfg(windows)] 
    {
    println!("cargo:rustc-link-search=native={}", "lib");
    println!("cargo:rustc-link-lib=libvips.lib");
    println!("cargo:rustc-link-lib=libglib-2.0.lib");
    println!("cargo:rustc-link-lib=libgobject-2.0.lib");
    }

    #[cfg(not(windows))]
    {
    println!("cargo:rustc-link-lib=vips");
    println!("cargo:rustc-link-lib=glib-2.0");
    println!("cargo:rustc-link-lib=gobject-2.0");
    }
}
