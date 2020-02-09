use is_same::IsSame;
use is_same_derive::IsSame;

#[derive(IsSame)]
struct MyCustomType {
    foo: usize,
    bar: String,
    baz: char,
}

#[derive(IsSame)]
struct MyTupleStruct(usize, &'static str);

#[derive(IsSame)]
struct MyUnitStruct;

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

    let left = MyTupleStruct(2, "foo");
    let mut right = MyTupleStruct(2, "foo");
    assert!(left.is_same(&right));
    right.0 += 1;
    assert!(left.is_not_same(&right));

    assert!(MyUnitStruct.is_same(&MyUnitStruct));
}
