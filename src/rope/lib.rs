#![crate_name="rope"]
#![crate_type="lib"]

#![license = "MIT"]
#![doc(html_root_url = "http://darinmorrison.github.io/rope.rs/doc/rope/")]

#![feature(struct_variant)]

//! This crate implements the Rope data type.

#![allow(dead_code)]
#![allow(unused_imports)]

extern crate algebra;
extern crate fingertree;

use fingertree::{
    Deep,
    Digit,
    Empty,
    FingerTree,
    Four,
    One,
    Single,
    Three,
    Two,
};

struct Offset(i32);
struct Chunk(Vec<u8>);
struct Body(FingerTree<Offset,Chunk>);
struct Rope(Body);
