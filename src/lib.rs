#![crate_name = "mecab"]
#![crate_type = "lib"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

extern crate libc;

pub use mecab::*;
mod mecab;
