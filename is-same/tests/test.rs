use is_same::IsSame;

#[test]
fn check_floats() {
    assert!((1.0f32).is_same(&1.0f32));
    assert!((0.0f32).is_same(&0.0f32));
    assert!((0.0f32).is_not_same(&1.0f32));
    assert!(std::f32::NAN.is_same(&std::f32::NAN));
    assert!(std::f32::INFINITY.is_same(&std::f32::INFINITY));
    assert!(std::f32::INFINITY.is_not_same(&std::f32::NEG_INFINITY));
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
