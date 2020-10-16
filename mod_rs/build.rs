extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the
    // shared library.
    println!("cargo:include=/home/rustacian/downloads/httpd-2.4.46/include");
    println!("cargo:include=/home/rustacian/downloads/apr-1.7.0/include");
    println!("cargo:include=/usr/include/apache2");
    println!("cargo:include=/usr/include/apr-1.0");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Add the includes for C header files.
        // Derived from https://httpd.apache.org/docs/2.4/developer/modguide.html
        // $ apxs -a -c mod_example.c
        .clang_arg("-I/usr/include/apache2 -I/usr/include/apr-1.0 -I/home/rustacian/downloads/httpd-2.4.46/include -I/home/rustacian/downloads/apr-1.7.0/include")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
