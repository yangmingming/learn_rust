// 普通enum
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// 枚举变量携带数据
// 这种用法的常见场景是，当错误或状态需要附带一些上下文信息时。例如，错误类型不仅需要标识错误类型，还需要提供详细的错误信息。
enum TicketError {
    TitleError(String),
    DescriptionError(String),
}

// 使用 TicketError 枚举
fn check_ticket(title: &str, description: &str) -> Result<(), TicketError> {
    if title.is_empty() {
        return Err(TicketError::TitleError("Title cannot be empty".to_string()));
    }
    if description.is_empty() {
        return Err(TicketError::DescriptionError("Description cannot be empty".to_string()));
    }
    Ok(())
}

enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

/*
在 Rust 中，枚举可以携带变量，以此来表示不同的状态或者情况。通过使用不同的方式来定义枚举变体，可以让变体携带额外的数据。枚举变体后面可以跟 {} 或 ()，分别表示命名字段和元组结构体。

定义时使用 {} 和 () 的区别
命名字段（使用 {}）：
    * 这种方式类似于结构体，每个字段都有一个名字。
    * 优点是字段名称可以增加代码的可读性，并且在使用时可以明确字段的含义。
元组结构体（使用 ()）：
    * 这种方式类似于元组，每个字段没有名字，只有类型。
    * 优点是定义简洁，当字段的数量较少且含义明确时比较方便。
*/

// 枚举变体携带变量的例子
// 定义一个枚举，用于表示网络请求的结果
#[derive(Debug)]
enum HttpRequest {
    // 成功状态，携带响应码和响应体
    Success { code: u16, body: String },
    // 失败状态，携带错误信息
    Error(String),
    // 重定向状态，携带新的 URL
    Redirect(String),
}

fn main() {
    // 普通enum
    let coin = Coin::Penny;
    let coin_num = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
    println!("coin {:?} num is {}", coin, coin_num);

    // 枚举变量携带()变量
    match check_ticket("", "This is a description") {
        Ok(_) => println!("Ticket is valid"),
        Err(TicketError::TitleError(msg)) => println!("Title error: {}", msg),
        Err(TicketError::DescriptionError(msg)) => println!("Description error: {}", msg),
    }

    // 枚举变量携带{}命名字段
    let status = Status::InProgress { assigned_to: "Alice".to_string() };
    match status {
        Status::ToDo => println!("Task is to be done"),
        Status::InProgress { assigned_to } => println!("Task is in progress, assigned to {}", assigned_to),
        Status::Done => println!("Task is done"),
    }

    // 混合举例
    let tasks = vec![
        HttpRequest::Success { code: 200, body: "wellcom to rust".to_string()},
        HttpRequest::Error ("4xx".to_string()),
        HttpRequest::Redirect ("http:://404.com".to_string()),
    ];

    for task in tasks {
        print!("task {:?} ", task);
        match task {
            HttpRequest::Success {code, body } => {
                println!("success code {}, body {}", code, body);
            },
            HttpRequest::Error(err) => println!("error {}", err),
            HttpRequest::Redirect(url) => println!("redirect to {}", url),
        }
    }
}

