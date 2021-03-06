#![crate_name="rope"]
#![crate_type="lib"]
#![feature(zero_one)]
// #![license = "MIT"]
#![doc(html_root_url = "http://www.rust-ci.org/epsilonz/rope.rs/doc/rope/")]

//! This crate implements the Rope data type.

extern crate algebra;
extern crate missing_algebra;
extern crate fingertree;

pub mod persistent;
