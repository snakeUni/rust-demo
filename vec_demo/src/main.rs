fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    vec[0] = 7;
    assert_eq!(vec[0], 7);

    // index
    let v = vec![0, 2, 4, 6];
    println!("{}", v[1]);

    // slicing
    // with_capacity
    let mut vec1 = Vec::with_capacity(10);
    assert_eq!(vec1.len(), 0);
    assert_eq!(vec1.capacity(), 10);

    for i in 0..10 {
        vec1.push(i)
    }
    assert_eq!(vec1.len(), 10);
    assert_eq!(vec1.capacity(), 10);

    vec1.push(11);
    assert_eq!(vec1.len(), 11);
    assert!(vec1.capacity() >= 11);

    // capacity 容量
    let vec2: Vec<i32> = Vec::with_capacity(10);
    assert_eq!(vec2.capacity(), 10);

    // reserve 备用
    let mut vec3 = vec![1];
    vec3.reserve(10);
    assert!(vec3.capacity() >= 10);

    // reserve_exact
    let mut vec4 = vec![1];
    vec4.reserve_exact(10);
    assert!(vec4.capacity() >= 10);

    // shrink_to_fit
    let mut vec5 = Vec::with_capacity(10);
    vec5.extend([1, 2, 3]);
    assert_eq!(vec5.capacity(), 10);
    vec5.shrink_to_fit();
    assert!(vec5.capacity() >= 3);

    // shrink_to
    let mut vec6 = Vec::with_capacity(10);
    vec6.extend([1, 2, 3]);
    assert_eq!(vec6.capacity(), 10);
    // vec6.shrink_to(4);
    // assert!(vec6.capacity() >= 4);
    // vec6.shrink_to(0);
    // assert!(vec6.capacity() >= 3);

    // into_boxed_slice
    let b = vec![1, 2, 3];
    let slice = b.into_boxed_slice();

    // truncate
    let mut t = vec![1, 2, 3, 4, 5];
    t.truncate(2);
    assert_eq!(t, [1, 2]);

    // 如果参数是 0 ，则相当于 clear
    let mut t1 = vec![1, 2, 3];
    t1.truncate(0);
    assert_eq!(t1, []);

    // swap_remove 移除一个元素，移除的元素被最后一个元素替代
    let mut s = vec!["foo", "bar", "baz", "qux"];
    assert_eq!(s.swap_remove(1), "bar");
    assert_eq!(s, ["foo", "qux", "baz"]);
    assert_eq!(s.swap_remove(0), "foo");
    assert_eq!(s, ["baz", "qux"]);

    // insert 在指定位置插入元素，其他的元素移到右边，类似于 splice if index > len, 则 panic
    let mut i = vec![1, 2, 3];
    i.insert(1, 4);
    assert_eq!(i, [1, 4, 2, 3]);
    i.insert(4, 5);
    assert_eq!(i, [1, 4, 2, 3, 5]);

    // remove 移除并且返回指定位置的元素, 改变原数据
    let mut r = vec![1, 2, 3];
    assert_eq!(r.remove(1), 2);
    assert_eq!(r, [1, 3]);

    // retain 保留符合条件的元素 F: FnMut(&T) -> bool,
    let mut re = vec![1, 2, 3, 4];
    re.retain(|&x| x % 2 == 0);
    assert_eq!(re, [2, 4]);
    let mut re1 = vec![1, 2, 3, 4, 5];
    let keep = [false, true, true, false, true];
    let mut iter = keep.iter();
    re1.retain(|_| *iter.next().unwrap());
    assert_eq!(re1, [2, 3, 5]);

    // dedup_by_key FnMut(&mut T) -> K, 删除向量中解析为相同键的所有连续元素，但第一个元素除外。
    // 如果向量已排序，则会删除所有重复项。
    let mut de = vec![10, 20, 21, 30, 20];
    de.dedup_by_key(|i| *i / 10);
    assert_eq!(de, [10, 20, 30, 20]);

    // dedup_by FnMut(&mut T, &mut T) -> bool, same_bucket function 比较使用相反的顺序，
    // if same_bucket(a, b) returns true, a is removed.
    let mut ded = vec!["foo", "bar", "Bar", "baz", "bar"];
    ded.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    assert_eq!(ded, ["foo", "bar", "baz", "bar"]);

    // push
    let mut p = vec![1, 2];
    p.push(3);
    assert_eq!(p, [1, 2, 3]);

    // pop
    let mut po = vec![1, 2, 3];
    assert_eq!(po.pop(), Some(3));
    assert_eq!(po, [1, 2]);

    // append Moves all the elements of other into Self, leaving other empty.
    let mut ap1 = vec![1, 2, 3];
    let mut ap2 = vec![4, 5, 6];
    ap1.append(&mut ap2);
    assert_eq!(ap1, [1, 2, 3, 4, 5, 6]);
    assert_eq!(ap2, []);

    // drain
    let mut da = vec![1, 2, 3];
    let u: Vec<_> = da.drain(1..).collect();
    assert_eq!(da, &[1]);
    assert_eq!(u, &[2, 3]);

    da.drain(..);
    assert_eq!(da, &[]);

    // clear 清除
    // len 长度
    // is_empty 是否为空

    // split_off 分为两部分, 返回 [at, len)，原来的变为 [0, at)
    let mut sp = vec![1, 2, 3];
    let sp1 = sp.split_off(1);
    assert_eq!(sp, [1]);
    assert_eq!(sp1, [2, 3]);

    // resize_with 可通过第二个参数来调整填充的数字

    // leak Consumes and leaks the Vec, returning a mutable reference to the contents, &'a mut [T].
    let le = vec![1, 2, 3];
    let static_ref: &'static mut [usize] = le.leak();
    static_ref[0] += 1;
    assert_eq!(static_ref, &[2, 2, 3]);

    // resize
    let mut resize = vec!["hello"];
    resize.resize(3, "world");
    assert_eq!(resize, ["hello", "world", "world"]);

    let mut resize1 = vec![1, 2, 3, 4];
    resize1.resize(2, 0);
    assert_eq!(resize1, [1, 2]);

    // splice
    let mut splice1 = vec![1, 2, 3, 4];
    let new = [7, 8, 9];
    let u1: Vec<_> = splice1.splice(1..3, new).collect();
    assert_eq!(splice1, &[1, 7, 8, 9, 4]);
    assert_eq!(u1, &[2, 3]);

    // drain_filter 会突变源数据。返回的是过滤后的元素，源数据变为过滤掉的数据
    let iter = &[1, 2, 4];
    let mut iter_x = iter.iter();
    assert_eq!(iter_x.next(), Some(&1));
    assert_eq!(iter_x.next(), Some(&2));
    assert_eq!(iter_x.next(), Some(&4));
    assert_eq!(iter_x.next(), None);
}
