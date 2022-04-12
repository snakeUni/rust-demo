use std::collections::HashMap;

fn main() {
    // keys
    // let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    // for key in map.keys() {
    //     println!("{}", key);
    // }

    // into_keys
    // values 返回所有的 value
    // values_mut 突变值
    // into_values
    // iter 迭代
    // iter_mut 迭代过程中可以突变
    // len
    let mut a = HashMap::new();
    assert_eq!(a.len(), 0);
    a.insert(1, "a");
    assert_eq!(a.len(), 1);

    // is_empty
    let mut em = HashMap::new();
    assert!(em.is_empty());
    em.insert(1, "a");
    assert!(!em.is_empty());

    // drain clears the map
    let mut drain = HashMap::new();
    drain.insert(1, "a");
    drain.insert(2, "b");

    for (k, v) in drain.drain().take(1) {
        assert!(k == 1 || k == 2);
        assert!(v == "a" || v == "b");
    }

    assert!(drain.is_empty());

    // drain_filter 过滤之前的，也可以突变之前的数据
    // retain 保留符合条件的
    let mut re: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
    re.retain(|&k, _| k % 2 == 0);
    assert_eq!(re.len(), 4);

    // clear 清空
    let mut cl = HashMap::new();
    cl.insert(1, "a");
    cl.clear();
    assert!(cl.is_empty());

    // get 获取相应的值
    let mut ge = HashMap::new();
    ge.insert(1, "a");
    assert_eq!(ge.get(&1), Some(&"a"));
    assert_eq!(ge.get(&2), None);

    // get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &V)>
    let mut gkv = HashMap::new();
    gkv.insert(1, "a");
    assert_eq!(gkv.get_key_value(&1), Some((&1, &"a")));
    assert_eq!(gkv.get_key_value(&2), None);

    // contains_key<Q: ?Sized>(&self, k: &Q) -> bool
    let mut ck = HashMap::new();
    ck.insert(1, "a");
    assert_eq!(ck.contains_key(&1), true);
    assert_eq!(ck.contains_key(&2), true);

    // get_mut 用于突变
    // insert 插入指定的 key, value

    // remove 移除指定的 key,但是返回相应的值 remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    let mut rm = HashMap::new();
    rm.insert(1, "a");
    assert_eq!(rm.remove(&1), Some("a"));
    assert_eq!(rm.remove(&1), None);

    // remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)> 返回指定的 key, value
    let mut rme = HashMap::new();
    rme.insert(1, "a");
    assert_eq!(rme.remove_entry(&1), Some((1, "a")));
    assert_eq!(rme.remove_entry(&1), None);

    // clone(&self) -> Self 返回 clone 的值

    // clone_from(&mut self, other: &Self) clone other
}
