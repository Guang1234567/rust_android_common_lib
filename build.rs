// build.rs

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    //cargo:rustc-cdylib-link-arg=-Wl,-soname,libfoo.so.1.2.3
    println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libgreetings.so");
}