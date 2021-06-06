fn main() {
    println!("------ 使用 loop 循环 -------");
    let mut counter = 0;

    // loop 循环是通过 Break 的形式来打破循环
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    println!("------ 使用 while 循环 -------");

    let mut number = 3;

    // while 循环是根据条件来判断是否继续循环
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    // for 循环
    println!("------ 使用 for 循环 -------");
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
