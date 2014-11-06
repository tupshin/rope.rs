#![crate_name="rope"]
#![crate_type="lib"]

#![license = "MIT"]
#![doc(html_root_url = "http://www.rust-ci.org/epsilonz/rope.rs/doc/rope/")]

#![feature(struct_variant)]

//! This crate implements the Rope data type.

extern crate algebra;
extern crate fingertree;

pub mod persistent;
