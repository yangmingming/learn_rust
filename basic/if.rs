fn main() {
    // 普通使用方法
    let number = 0;

    if number > 0 {
        println!("The number is positive");
    } else if number < 0 {
        println!("The number is negative");
    } else {
        println!("The number is zero");
    }

    // if let 语句
    let some_option = Some(5);

    if let Some(x) = some_option {
        println!("The value is {}", x);
    } else {
        println!("The option is None");
    }

    // if match
    let num = 5;

    match num {
        n if n > 0 => println!("The num is positive"),
        n if n < 0 => println!("The num is negative"),
        _ => println!("The num is zero"),
    }
}
