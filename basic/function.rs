
// 普通函数定义
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 字符串返回
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// 返回元组
fn sum_and_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

// 泛型函数
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// 传递可变引用
fn increment(value: &mut i32) {
    *value += 1;
}

// 传递和返回引用
// 'a 是一种生命周期参数（Lifetime Parameter）的标识符。用于指定引用的有效期，帮助 Rust 编译器确定引用在内存中的有效范围，从而确保引用不会指向悬空的数据。
fn find_largest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a > b {
        a
    } else {
        b
    }
}

// 入参枚举类型，搭配模式匹配
#[derive(Debug, Clone)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// 错误处理函数
use std::fs::File;
use std::io::{self, Read};
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 返回自定义enum错误
enum TicketError {
    TitleError(String),
    DescriptionError(String),
}

fn create_ticket(title: String, description: String) -> Result<(String, String), TicketError> {
    if title.is_empty() {
        return Err(TicketError::TitleError("Title cannot be empty".to_string()));
    }
    if title.len() > 50 {
        return Err(TicketError::TitleError("Title cannot be longer than 50 bytes".to_string()));
    }

    if description.is_empty() {
        return Err(TicketError::DescriptionError("Description cannot be empty".to_string()));
    }
    if description.len() > 500 {
        return Err(TicketError::DescriptionError("Description cannot be longer than 500 bytes".to_string()));
    }

    Ok((title, description))
}

// 闭包相关
// FnOnce 是一个闭包特性（Trait），它表明闭包可以被调用一次，并且不再使用闭包中捕获的任何变量。
/*
   FnOnce 闭包会消耗其捕获的变量，只能被调用一次；
   FnMut 闭包可以修改其捕获的变量，并且可以被多次调用；
   Fn 闭包只能读取其捕获的变量，不能修改它们，并且可以被多次调用。
*/
fn apply<F>(f: F) where F: FnOnce() {
    f();
}

fn main() {
    let x = 5;
    let y = 10;
    let result = add(x, y);
    println!("The sum of {} and {} is {}", x, y, result);

    // 字符串返回
    let name = "Alice";
    let greeting = greet(name);
    println!("{}", greeting);

    // 返回元组并结构返回数据
    let a = 3;
    let b = 4;
    let (sum, product) = sum_and_product(a, b);
    println!("The sum of {} and {} is {}", a, b, sum);
    println!("The product of {} and {} is {}", a, b, product);

    // 泛型函数的使用，一次定义支持多类型
    let int_x = 5;
    let int_y = 10;
    println!("The max of {} and {} is {}", int_x, int_y, max(int_x, int_y));
    let float_x = 3.5;
    let float_y = 2.5;
    println!("The max of {} and {} is {}", float_x, float_y, max(float_x, float_y));

    // 可变引用
    let mut z = 5;
    increment(&mut z);
    println!("z is now {}", z);

    // 传递和返回引用
    let str1 = "apple";
    let str2 = "orange";
    let result = find_largest(str1, str2);
    println!("The largest string is {}", result);

    // 入参是枚举类型
    let coin = Coin::Dime;
    println!("The value of the coin {:?} is {} cents", coin.clone(), value_in_cents(coin));

    // 错误处理
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading file: {}", e),
    }

    // 自定义枚举异常
    match create_ticket("".to_string(), "today is 5".to_string()) {
        Ok((title, description)) => {
            println!("title {}, description {}", title, description);
        },
        Err(TicketError::TitleError(e)) => println!("Title error: {}", e),
        Err(TicketError::DescriptionError(e)) => println!("Description error: {}", e),
    }

    // 闭包函数
    let greeting = String::from("Hello");
    let say_hello = || println!("{}", greeting); // 闭包函数,可以使用外部变量
    apply(say_hello);
}

