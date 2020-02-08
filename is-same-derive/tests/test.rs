use is_same::IsSame;
use is_same_derive::IsSame;

#[derive(IsSame)]
struct MyCustomType {
    foo: usize,
    bar: String,
    baz: char,
}

#[test]
fn test_cmp() {
    let left = MyCustomType {
        foo: 2,
        bar: "asdf".to_owned(),
        baz: 'a',
    };
    let mut right = MyCustomType {
        foo: 2,
        bar: "asdf".to_owned(),
        baz: 'a',
    };
    assert!(left.is_same(&right));
    right.foo += 1;
    assert!(left.is_not_same(&right));
}
