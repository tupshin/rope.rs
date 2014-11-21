//! Ropes based on persistent Finger Trees.

use epsilonz_algebra::{
    Magma,
    Monoid,
    Semigroup,
};
use fingertree::persistent::{
    FingerTree,
    Measurable,
};

#[deriving(Clone)]
pub struct Offset(uint);

impl ::std::num::Zero for Offset {
    fn zero() -> Offset {
        Offset(0u)
    }

    fn is_zero(&self) -> bool {
        let &Offset(ref i) = self;
        i.is_zero()
    }
}

impl ::std::ops::Add<Offset,Offset> for Offset {
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
        ::std::num::Zero::zero()
    }
}

pub struct Chunk(Vec<u8>);

impl Measurable<Offset> for Chunk {
    fn measure(&self) -> Offset {
        let &Chunk(ref data) = self;
        Offset(data.len())
    }
}

pub struct Body(FingerTree<Offset,Chunk>);

impl Measurable<Offset> for Body {
    fn measure(&self) -> Offset {
        let &Body(ref tree) = self;
        tree.measure()
    }
}

pub struct Rope(Body);

impl Rope {
    fn len(&self) -> uint {
        let &Rope(ref body) = self;
        let Offset(off) = body.measure();
        off
    }

    fn is_empty(&self) -> bool {
        let &Rope(Body(ref tree)) = self;
        tree.is_empty()
    }
}
