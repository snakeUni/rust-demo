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
}
