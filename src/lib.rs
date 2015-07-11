#![crate_name = "mecab"]
#![crate_type = "lib"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(cstr_memory)]

extern crate libc;

pub use mecab::*;
mod mecab;
