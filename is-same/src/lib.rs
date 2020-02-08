use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;
use std::sync::Arc;

pub trait IsSame {
    fn is_same(&self, other: &Self) -> bool;

    fn is_not_same(&self, other: &Self) -> bool {
        !self.is_same(other)
    }
}

impl<T> IsSame for Rc<T> {
    fn is_same(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }
}

impl<T> IsSame for Arc<T> {
    fn is_same(&self, other: &Self) -> bool {
        Arc::ptr_eq(self, other)
    }
}

impl<T> IsSame for Vec<T>
where
    T: IsSame,
{
    fn is_same(&self, other: &Self) -> bool {
        if self.as_ptr() == other.as_ptr() {
            true
        } else {
            self.iter()
                .zip(other.iter())
                .all(|(left, right)| left.is_same(right))
        }
    }
}

impl<Key, Value> IsSame for BTreeMap<Key, Value>
where
    Key: IsSame + Ord,
    Value: IsSame,
{
    fn is_same(&self, other: &Self) -> bool {
        let mut left = self.iter();
        let mut right = other.iter();

        loop {
            let a = left.next();
            let b = right.next();
            match (a, b) {
                (None, None) => return true,
                (Some((left_key, left_val)), Some((right_key, right_val)))
                    if left_key == right_key =>
                {
                    if left_val.is_not_same(right_val) {
                        return false;
                    }
                }
                (_, _) => return false,
            }
        }
    }
}

impl<Key> IsSame for BTreeSet<Key>
where
    Key: IsSame + Ord,
{
    fn is_same(&self, other: &Self) -> bool {
        let mut left = self.iter();
        let mut right = other.iter();

        loop {
            let a = left.next();
            let b = right.next();
            match (a, b) {
                (None, None) => return true,
                (Some(left_key), Some(right_key)) if left_key == right_key => (),
                (_, _) => return false,
            }
        }
    }
}

impl<'a> IsSame for &'a str {
    fn is_same(&self, other: &Self) -> bool {
        self == other
    }
}

impl IsSame for f32 {
    fn is_same(&self, other: &Self) -> bool {
        self.to_bits() == other.to_bits()
    }
}

impl IsSame for f64 {
    fn is_same(&self, other: &Self) -> bool {
        self.to_bits() == other.to_bits()
    }
}

macro_rules! simple_impl {
    ($name:ty) => {
        impl IsSame for $name {
            fn is_same(&self, other: &Self) -> bool {
                self == other
            }
        }
    };
}

simple_impl!(u8);
simple_impl!(u16);
simple_impl!(u32);
simple_impl!(u64);
simple_impl!(u128);
simple_impl!(usize);
simple_impl!(i8);
simple_impl!(i16);
simple_impl!(i32);
simple_impl!(i64);
simple_impl!(i128);
simple_impl!(isize);
simple_impl!(bool);
simple_impl!(char);
simple_impl!(());
simple_impl!(String);

macro_rules! tuple_impl {
    ($($tyname:ident, $left:ident, $right:ident;)+) => {
        impl<$($tyname),+> IsSame for ($($tyname,)+)
        where
            $($tyname : IsSame),+
        {
            fn is_same(&self, other: &Self) -> bool {
                let ($(ref $left,)+) = self;
                let ($(ref $right,)+) = other;
                $( $left.is_same($right) )&&+
            }
        }
    };
}

tuple_impl! {
    T1, left, right;
}

tuple_impl! {
    T1, left1, right1;
    T2, left2, right2;
}

tuple_impl! {
    T1, left1, right1;
    T2, left2, right2;
    T3, left3, right3;
}

tuple_impl! {
    T1, left1, right1;
    T2, left2, right2;
    T3, left3, right3;
    T4, left4, right4;
}

tuple_impl! {
    T1, left1, right1;
    T2, left2, right2;
    T3, left3, right3;
    T4, left4, right4;
    T5, left5, right5;
}

tuple_impl! {
    T1, left1, right1;
    T2, left2, right2;
    T3, left3, right3;
    T4, left4, right4;
    T5, left5, right5;
    T6, left6, right6;
}

tuple_impl! {
    T1, left1, right1;
    T2, left2, right2;
    T3, left3, right3;
    T4, left4, right4;
    T5, left5, right5;
    T6, left6, right6;
    T7, left7, right7;
}

tuple_impl! {
    T1, left1, right1;
    T2, left2, right2;
    T3, left3, right3;
    T4, left4, right4;
    T5, left5, right5;
    T6, left6, right6;
    T7, left7, right7;
    T8, left8, right8;
}

macro_rules! array_impl {
    ($( $count:tt )+) => {$(
        impl<T> IsSame for [T; $count]
        where
            T: IsSame,
        {
            fn is_same(&self, other: &Self) -> bool {
                for i in 0..$count {
                    if self[i].is_not_same(&other[i]) {
                        return false;
                    }
                }
                true
            }
        }
    )+};
}

array_impl!(
    0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29 30 31 32
);
