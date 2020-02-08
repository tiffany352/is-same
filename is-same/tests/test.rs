use is_same::IsSame;

#[test]
fn check_floats() {
    assert!((1.0f32).is_same(&1.0f32));
    assert!((0.0f32).is_same(&0.0f32));
    assert!((0.0f32).is_not_same(&1.0f32));
    assert!(std::f32::NAN.is_same(&std::f32::NAN));
    assert!(std::f32::INFINITY.is_same(&std::f32::INFINITY));
    assert!(std::f32::INFINITY.is_not_same(&std::f32::NEG_INFINITY));

    assert!((1.0f64).is_same(&1.0f64));
    assert!((0.0f64).is_same(&0.0f64));
    assert!((0.0f64).is_not_same(&1.0f64));
    assert!(std::f64::NAN.is_same(&std::f64::NAN));
    assert!(std::f64::INFINITY.is_same(&std::f64::INFINITY));
    assert!(std::f64::INFINITY.is_not_same(&std::f64::NEG_INFINITY));
}

#[test]
fn check_btree_map() {
    use std::collections::BTreeMap;

    let mut map1 = BTreeMap::new();
    map1.insert("foo", "bar");
    map1.insert("bar", "foo");
    let mut map2 = BTreeMap::new();
    map2.insert("bar", "foo");
    map2.insert("foo", "bar");
    assert!(map1.is_same(&map2));
    map2.insert("baz", "f");
    assert!(map1.is_not_same(&map2));
    map2.remove("baz");
    assert!(map1.is_same(&map2));
    map2.insert("bar", "asdf");
    assert!(map1.is_not_same(&map2));
}

#[test]
fn check_btree_set() {
    use std::collections::BTreeSet;

    let mut map1 = BTreeSet::new();
    map1.insert("foo");
    map1.insert("bar");
    let mut map2 = BTreeSet::new();
    map2.insert("bar");
    map2.insert("foo");
    assert!(map1.is_same(&map2));
    map2.insert("baz");
    assert!(map1.is_not_same(&map2));
    map2.remove("baz");
    assert!(map1.is_same(&map2));
}

#[test]
fn check_vec() {
    let vec1 = vec![1, 2, 3];
    let mut vec2 = vec![1, 2];
    assert!(vec1.is_not_same(&vec2));
    assert!(vec2.is_not_same(&vec1));
    vec2.push(3);
    assert!(vec1.is_same(&vec2));
    vec2.swap(1, 2);
    assert!(vec1.is_not_same(&vec2));
}

#[test]
fn simple_checks() {
    use std::rc::Rc;
    use std::sync::Arc;

    let rc1 = Rc::new(4);
    let rc2 = rc1.clone();
    assert!(rc1.is_same(&rc2));
    let rc2 = Rc::new(3);
    assert!(rc1.is_not_same(&rc2));

    let rc1 = Arc::new(4);
    let rc2 = rc1.clone();
    assert!(rc1.is_same(&rc2));
    let rc2 = Arc::new(3);
    assert!(rc1.is_not_same(&rc2));
}

#[test]
fn check_arrays() {
    let arr1 = [1, 2, 3];
    let arr2 = [1, 2, 3];
    assert!(arr1.is_same(&arr2));
    let arr2 = [1, 2, 2];
    assert!(arr1.is_not_same(&arr2));
}

#[test]
fn check_tuples() {
    let t1 = (1, 2, "baz");
    let t2 = (1, 2, "baz");
    assert!(t1.is_same(&t2));
    let t2 = (1, 3, "baz");
    assert!(t1.is_not_same(&t2));
}
