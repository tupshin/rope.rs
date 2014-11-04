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

use algebra::{
    M,
    Magma,
    Monoid,
    Semigroup,
};
use fingertree::{
    Deep,
    Digit,
    Empty,
    FingerTree,
    Four,
    Measurable,
    One,
    Single,
    Three,
    Two,
};

struct Offset(uint);

impl std::num::Zero for Offset {
    fn zero() -> Offset {
        Offset(0u)
    }

    fn is_zero(&self) -> bool {
        let &Offset(ref i) = self;
        i.is_zero()
    }
}

impl std::ops::Add<Offset,Offset> for Offset {
    fn add(&self, that:&Offset) -> Offset {
        let &Offset(ref lhs) = self;
        let &Offset(ref rhs) = that;
        Offset(lhs.add(rhs))
    }
}

impl Magma for Offset {
    fn op(&self, rhs:&Offset) -> Offset {
        self.add(rhs)
    }
}

impl Semigroup for Offset {}

impl Monoid for Offset {
    fn nil() -> Offset {
        std::num::Zero::zero()
    }
}

struct Chunk(Vec<u8>);

impl Measurable<Offset> for Chunk {
    fn measure(&self) -> Offset {
        let &Chunk(ref data) = self;
        Offset(data.len())
    }
}

struct Body(FingerTree<Offset,Chunk>);

struct Rope(Body);
