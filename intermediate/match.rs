#[derive(Debug)]
enum Shape {
    Circle,
    Square,
    Rectangle,
    Triangle,
    Pentagon,
}

struct Person {
    name: String,
    age: u8,
}

/*
    let x = Some(3);

    match x {
        Some(value) => println!("Value found: {}", value),
        None => println!("No value")
    }


    let result = divide(4, 2);

    match result {
        Ok(num) => println!("Division is successful: {}", num),
        Err(e) => println!("Error occurred: {}", e)
    }
*/


fn main() {
    // 匹配简单数值
    let number = 2;
    match number {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        _ => println!("Something else!"), // `_` 是一个通配符，匹配所有未列出的情况
    }

    // match匹配枚举类型
    let shape = Shape::Rectangle;
    let sides = match shape {
        Shape::Circle => 0,
        Shape::Square | Shape::Rectangle => 4,
        Shape::Triangle => 3,
        Shape::Pentagon => 5
    };
    println!("{:?} has {} sides", shape, sides);

    // 匹配元组
    let point = (0, 3);
    match point {
        (0, 0) => println!("Point is at the origin"),
        (0, y) => println!("Point is on the y-axis at {}", y),
        (x, 0) => println!("Point is on the x-axis at {}", x),
        (x, y) => println!("Point is at ({}, {})", x, y),
    }

    // match解构结构体
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    match person {
        Person { name, age: 30 } => println!("hit: {} is 30 years old", name),
        Person { name, age } => println!("not hit: {} is {} years old", name, age),
    }

    /* 
        在Rust中，Option类型用于表示一个可能包含值或者不包含值的类型。
        Option有两个变体：
            * Some(T)表示包含一个值的变体
            * None表示没有值的变体。
    */
    // match 匹配 option
    let some_number = Some(5);
    let no_number: Option<i32> = None;
    match some_number {
        Some(x) => println!("The number is {}", x),
        None => println!("No number found"),
    }
    match no_number {
        Some(x) => println!("The number is {}", x),
        None => println!("No number found"),
    }

    // match 匹配组合option
    let a: Option<i32> = Some(10);
    let b: Option<i32> = None;
    let c: Option<i32> = Some(20);
    match (a, b, c) {
        (Some(x), Some(y), Some(z)) => println!("All values are present: {}, {}, {}", x, y, z),
        (Some(x), Some(y), None) => println!("Two values are present: {}, {}", x, y),
        (Some(x), None, Some(z)) => println!("Two values are present: {}, {}", x, z),
        (None, Some(y), Some(z)) => println!("Two values are present: {}, {}", y, z),
        (Some(x), None, None) => println!("Only one value is present: {}", x),
        (None, Some(y), None) => println!("Only one value is present: {}", y),
        (None, None, Some(z)) => println!("Only one value is present: {}", z),
        (None, None, None) => println!("No values are present"),
    }

    // 如果只关心Option类型的某一个变体，可以使用if let语句来简化代码。
    let some_number = Some(5);
    if let Some(x) = some_number {
        println!("The number is {}", x);
    } else {
        println!("No number found");
    }

    /*
        在Rust中，Result类型用于表示操作的成功或失败。Result有两个变体：
        * Ok(T)表示成功并包含一个值
        * Err(E)表示失败并包含一个错误值。
    */
    // match 匹配 result
    let ok_result: Result<i32, &str> = Ok(42);
    let err_result: Result<i32, &str> = Err("An error occurred");
    match ok_result {
        Ok(value) => println!("Success: {}", value),
        Err(err) => println!("Error: {}", err),
    }
    match err_result {
        Ok(value) => println!("Success: {}", value),
        Err(err) => println!("Error: {}", err),
    }

    // 如果只关心Result类型的某一个变体，可以使用if let语句来简化代码。
    let ok_result: Result<i32, &str> = Ok(42);
    if let Ok(value) = ok_result {
        println!("Success: {}", value);
    } else {
        println!("An error occurred");
    }
}
