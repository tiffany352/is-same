//! This crate provides the IsSame trait, which is specifically for
//! diffing immutable data that has been transformed. The trait differs
//! from PartialEq in some important ways:
//!
//! - Floating point values are compared by their bit patterns,
//!   preventing NaN values from making a data structure permanently
//!   compare as not equal to itself. This also lets you detect a value
//!   changing from `-0.0` to `0.0`.
//! - Referential equality is used to make comparisons more efficient.
//!   The library assumes that the contents of `Rc<T>` and `Arc<T>` are
//!   immutable and can't change, so they only need to be compared by
//!   their pointers.
//!
//! There is also a `is-same-derive` crate which can automatically
//! derive an IsSame implementation for your structs:
//! ```rs
//! use is_same_derive::IsSame;
//!
//! #[derive(IsSame)]
//! struct MyStruct {
//!     count: usize,
//!     ch: char,
//!     text: String,
//! }
//! ```

#![forbid(missing_docs)]
#![deny(clippy::all)]

use std::any::TypeId;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::{BuildHasher, Hash};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

/// Compares two versions of a piece of data to see if it has changed.
pub trait IsSame<Rhs = Self>
where
    Rhs: ?Sized,
{
    /// Returns true if the two values are identical.
    fn is_same(&self, other: &Rhs) -> bool;

    /// Equivalent to `!self.is_same(other)`.
    fn is_not_same(&self, other: &Rhs) -> bool {
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

impl<T, Rhs> IsSame<Rhs> for Vec<T>
where
    T: IsSame,
    Rhs: AsRef<[T]>,
{
    fn is_same(&self, other: &Rhs) -> bool {
        let other = other.as_ref();
        if self.as_ptr() == other.as_ptr() {
            true
        } else if self.len() != other.len() {
            false
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

impl<Key, Value, State> IsSame for HashMap<Key, Value, State>
where
    Key: IsSame + Eq + Hash,
    Value: IsSame,
    State: BuildHasher,
{
    fn is_same(&self, other: &Self) -> bool {
        // Both a fast path and required to make sure we don't miss any
        // keys that exist in `other` but not `self`. Assumes that the
        // Key type has a non-broken PartialEq implementation, which
        // could cause two entries to have the same key.
        if self.len() != other.len() {
            return false;
        }
        for (left_key, left_val) in self {
            if let Some(right_val) = other.get(left_key) {
                if left_val.is_not_same(&right_val) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

impl<Key, State> IsSame for HashSet<Key, State>
where
    Key: IsSame + Eq + Hash,
    State: BuildHasher,
{
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

impl<'a, T> IsSame for &'a T
where
    T: IsSame + ?Sized + 'a,
{
    fn is_same(&self, other: &Self) -> bool {
        if (*self as *const T) == (*other as *const T) {
            true
        } else {
            (*self).is_same(other)
        }
    }
}

impl<'a, T> IsSame for [T]
where
    T: IsSame + 'a,
{
    fn is_same(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            false
        } else {
            for i in 0..self.len() {
                if self[i].is_not_same(&other[i]) {
                    return false;
                }
            }
            true
        }
    }
}

impl<Rhs> IsSame<Rhs> for PathBuf
where
    Rhs: AsRef<Path>,
{
    fn is_same(&self, other: &Rhs) -> bool {
        self == other.as_ref()
    }
}

impl<Rhs> IsSame<Rhs> for Path
where
    Rhs: AsRef<Path>,
{
    fn is_same(&self, other: &Rhs) -> bool {
        self == other.as_ref()
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
simple_impl!(str);
simple_impl!(TypeId);
simple_impl!(Path);

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
