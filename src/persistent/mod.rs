//! Ropes based on persistent Finger Trees.
use std::ops::Add;

use fingertree::persistent::{
    FingerTree,
    Measurable,
    Magma,
    Monoid,
    Semigroup,
};

#[derive(Clone)]
pub struct Offset(usize);

impl ::std::num::Zero for Offset {
    fn zero() -> Offset {
        Offset(0usize)
    }

//    fn is_zero(&self) -> bool {
//        let &Offset(ref i) = self;
//        i.is_zero()
//    }
}

impl ::std::ops::Add<Offset> for Offset {
	type Output = Offset;
    fn add(self, that:Offset) -> Offset {
        let Offset(ref lhs) = self;
        let Offset(ref rhs) = that;
        Offset(lhs.add(rhs))
    }
}

impl Magma for Offset {
    fn op(&self, rhs:&Offset) -> Offset {
        self.clone().add(rhs.clone()) //FIXME are these clones necessary?
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
    fn len(&self) -> usize {
        let &Rope(ref body) = self;
        let Offset(off) = body.measure();
        off
    }

    fn is_empty(&self) -> bool {
        let &Rope(Body(ref tree)) = self;
        tree.is_empty()
    }
}
