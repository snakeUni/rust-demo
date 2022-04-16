#![allow(unused)]
fn main() {
    println!("Hello, world!");

    // is_some
    let is = Some(2);
    assert_eq!(is.is_some(), true);
    let is_n: Option<u32> = None;
    assert_eq!(is_n.is_some(), false);

    // is_none 判断是否是 none，与 is_some 相反。

    // as_ref Converts from &Option<T> to Option<&T>.
    let text: Option<String> = Some("Hello, world!".to_string());
    let text_length = text.as_ref().map(|s| s.len());
    println!("still can print text: {:?}", text);

    // as_mut Converts from &mut Option<T> to Option<&mut T>.
    let mut am = Some(2);
    match am.as_mut() {
        Some(v) => *v = 42,
        None => {}
    }
    assert_eq!(am, Some(42));

    // expect Returns the contained Some value, consuming the self value.
    let ex = Some("value");
    assert_eq!(ex.expect("fruits are healthy"), "value");

    // unwrap Returns the contained Some value,
    // Because this function may panic, its use is generally discouraged. Instead, prefer to use pattern matching and handle the None case explicitly, or call unwrap_or, unwrap_or_else, or unwrap_or_default.
    let unwrap = Some("air");
    assert_eq!(unwrap.unwrap(), "air");

    // unwrap_or
    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");

    // unwrap_or_else Returns the contained Some value or computes it from a closure.
    let k = 10;
    assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
    assert_eq!(None.unwrap_or_else(|| 2 * k), 20);

    // unwrap_or_default
    let good_year_from_input = "1909";
    let bad_year_from_input = "190blarg";
    let good_year = good_year_from_input.parse().ok().unwrap_or_default();
    let bad_year = bad_year_from_input.parse().ok().unwrap_or_default();

    assert_eq!(1909, good_year);
    assert_eq!(0, bad_year);

    // unwrap_unchecked Returns the contained Some value, consuming the self value, without checking that the value is not None.
    let air = Some("air");
    // assert_eq!(unsafe { air.unwrap_unchecked() }, "air");

    // map 转换，类似 js 的 map，只是不会迭代
    let maybe_some_string = Some(String::from("Hello, World!"));
    // `Option::map` takes self *by value*, consuming `maybe_some_string`
    let maybe_some_len = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(13));

    // map_or 推荐使用 map_or_else 如果需要使用函数
    assert_eq!(Some("foo").map_or(42, |v| v.len()), 3);
    let mo: Option<&str> = None;
    assert_eq!(mo.map_or(42, |v| v.len()), 42);

    // map_or_else
    let moe1 = Some("foo");
    assert_eq!(moe1.map_or_else(|| 2 * 21, |v| v.len()), 3);
    let moe2: Option<&str> = None;
    assert_eq!(moe2.map_or_else(|| 2 * 21, |v| v.len()), 42);

    // ok_or Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err).
    assert_eq!(Some("foo").ok_or(0), Ok("foo"));
    let oko: Option<&str> = None;
    assert_eq!(oko.ok_or(0), Err(0));

    // ok_or_else
    // Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err()).
    let ooe = Some("foo");
    assert_eq!(ooe.ok_or_else(|| 0), Ok("foo"));
    let ooe_n: Option<&str> = None;
    assert_eq!(ooe_n.ok_or_else(|| 0), Err(0));

    // as_deref Converts from Option<T> (or &Option<T>) to Option<&T::Target>.
    let ad: Option<String> = Some("hey".to_owned());
    assert_eq!(ad.as_deref(), Some("hey"));

    let ad_n: Option<String> = None;
    assert_eq!(ad_n.as_deref(), None);

    // iter 返回一个迭代

    // iter_mut 迭代是可以突变的

    // add Returns None if the option is None, otherwise returns optb.
    // and<U>(self, optb: Option<U>) -> Option<U>
    // 如果只要有一个 None 就返回 None,否则返回后面的那个参数。
    assert_eq!(Some(2).and(None as Option<&str>), None);
    assert_eq!((None as Option<&str>).and(Some("foo")), None);
    assert_eq!(Some(2).and(Some("foo")), Some("foo"));
    assert_eq!((None as Option<u32>).and((None as Option<&str>)), None);

    // add_then
    let arr_2d = [["A0", "A1"], ["B0", "B1"]];
    let item_0_1 = arr_2d.get(0).and_then(|row| row.get(1));
    assert_eq!(item_0_1, Some(&"A1"));

    let item_2_0 = arr_2d.get(2).and_then(|row| row.get(0));
    assert_eq!(item_2_0, None);

    // filter Returns None if the option is None, otherwise calls predicate with the wrapped value and returns:
    // Some(t) if predicate returns true (where t is the wrapped value), and
    // None if predicate returns false.
    fn is_even(n: &i32) -> bool {
        n % 2 == 0
    }
    assert_eq!(None.filter(is_even), None);
    assert_eq!(Some(3).filter(is_even), None);
    assert_eq!(Some(4).filter(is_even), Some(4));

    // or Returns the option if it contains a value, otherwise returns optb.
    assert_eq!(Some(2).or(None), Some(2));
    assert_eq!(None.or(Some(100)), Some(100));
    assert_eq!(Some(2).or(Some(100)), Some(2));
    assert_eq!((None as Option<u32>).or(None), None);

    // or_else Returns the option if it contains a value, otherwise calls f and returns the result.
    fn nobody() -> Option<&'static str> {
        None
    }
    fn vikings() -> Option<&'static str> {
        Some("vikings")
    }
    assert_eq!(Some("barbarians").or_else(vikings), Some("vikings"));
    assert_eq!(None.or_else(vikings), Some("vikings"));
    assert_eq!(None.or_else(nobody), None);

    // xor Returns Some if exactly one of self, optb is Some, otherwise returns None.
    // 类似异或操作
    assert_eq!(Some(2).xor((None as Option<u32>)), Some(2));
    assert_eq!((None as Option<u32>).xor(Some(2)), Some(2));
    assert_eq!(Some(2).xor(Some(2)), None);
    assert_eq!((None as Option<u32>).xor((None as Option<u32>)), None);

    // insert insert(&mut self, value: T) -> &mut T
    let mut opt = None;
    let val = opt.insert(1);
    assert_eq!(*val, 1);
    assert_eq!(opt.unwrap(), 1);
    let val = opt.insert(2);
    assert_eq!(*val, 2);
    *val = 3;
    assert_eq!(opt.unwrap(), 3);

    // get_or_insert_with<F>(&mut self, f: F) -> &mut T
    let mut goi = None;
    {
        let y: &mut u32 = goi.get_or_insert_with(|| 5);
        assert_eq!(y, &5);

        *y = 7;
    }
    assert_eq!(goi, Some(7));

    // take Takes the value out of the option, leaving a None in its place.
    let mut take = Some(2);
    let take_y = take.take();
    assert_eq!(take, None);
    assert_eq!(take_y, Some(2));

    let mut take_n: Option<u32> = None;
    let take_ny = take_n.take();
    assert_eq!(take_n, None);
    assert_eq!(take_ny, None);

    // replace
    let mut rep = Some(2);
    let old_rep = rep.replace(5);
    assert_eq!(rep, Some(5));
    assert_eq!(old_rep, Some(2));

    let mut rep_none = None;
    let old_n = rep_none.replace(3);
    assert_eq!(rep_none, Some(3));
    assert_eq!(old_n, None);

    // zip zip<U>(self, other: Option<U>) -> Option<(T, U)>
    assert_eq!(Some(1).zip(Some("hi")), Some((1, "hi")));
    assert_eq!(Some(1).zip(None::<u8>), None);

    // zip_with zip_with<U, F, R>(self, other: Option<U>, f: F) -> Option<R>
    // If self is Some(s) and other is Some(o), this method returns Some(f(s, o)). Otherwise, None is returned.

    // copied Maps an Option<&T> to an Option<T> by copying the contents of the option.
    let cx = 12;
    let opt_x = Some(&cx);
    assert_eq!(opt_x, Some(&12));
    let copied = opt_x.copied();
    assert_eq!(copied, Some(12));

    // cloned Maps an Option<&T> to an Option<T> by cloning the contents of the option.
    let clx = 12;
    let cl_opt_x = Some(&clx);
    assert_eq!(cl_opt_x, Some(&12));
    let cloned = cl_opt_x.cloned();
    assert_eq!(cloned, Some(12));

    // from
    // from(o: &'a Option<T>) -> Option<&'a T> Converts from &Option<T> to Option<&T>.
    // from(o: &'a mut Option<T>) -> Option<&'a mut T>  Converts from &mut Option<T> to Option<&mut T>
    // from(val: T) -> Option<T> Moves val into a new Some.
}
