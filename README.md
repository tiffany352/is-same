# is-same

[![Crates.io](https://img.shields.io/crates/v/is-same)](https://crates.io/crates/is-same)
![License: MIT/Apache-2.0](https://img.shields.io/crates/l/is-same)
![build](https://github.com/tiffany352/is-same/workflows/build/badge.svg)
[![Coverage Status](https://img.shields.io/codecov/c/github/tiffany352/is-same)](https://codecov.io/gh/tiffany352/is-same)

This crate provides the IsSame trait, which is specifically for diffing
immutable data that has been transformed. The trait differs from
PartialEq in some important ways:

- Floating point values are compared by their bit patterns, preventing
  NaN values from making a data structure permanently compare as not
  equal to itself. This also lets you detect a value changing from
  `-0.0` to `0.0`.
- Referential equality is used to make comparisons more efficient. The
  library assumes that the contents of `Rc<T>` and `Arc<T>` are
  immutable and can't change, so they only need to be compared by their
  pointers.

This trait is implemented out of the box for a lot of standard library
types, but if any are missing feel free to file an issue or contribute a
PR. Issues relating to error messages or other usability issues
(including with the proc macro) are also welcome!

The trait is explicitly not implemented for interior mutability types
(Cell, RefCell, AtomicUSize, Mutex, etc.), as this would make the
assumptions based on referential equality unsound. This could be changed
in the future if it presents a problem.

## Install

Add to your `Cargo.toml`:

```toml
is-same = "0.1"
is-same-derive = "0.1"
```

## Usage

```rust
use is_same::IsSame;
use is_same_derive::IsSame;

#[derive(IsSame)]
struct MyStruct {
    text: String,
    foo: usize,
    bar: char,
}

fn diff(left: &MyStruct, right: &MyStruct) {
    println!("is_same? {}", left.is_same(right));
}
```
