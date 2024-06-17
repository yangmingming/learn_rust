fn main() {
    let name = "Alice";
    let age = 30;
    let height = 1.75;

    // 基本实例
    println!("Name: {}", name);
    println!("Age: {}", age);

    // 占位符多变量打印
    println!("Name: {}, Age: {}, Height: {} meters", name, age, height);

    // 通过变量名称打印
    println!("Name: {name}, Age: {age}, Height: {height} meters");

    // 常用变量打印
    let boolean = true;
    let character = 'R';
    let integer = 42;
    let float = 3.14;

    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
    println!("Integer: {}", integer);
    println!("Float: {}", float);

    // 指定宽度 / 精度 / 对齐方式 等占位符
    let number = 42.6789;

    // 指定宽度和精度
    println!("Number: {:8.2}", number); // 宽度为8，精度为2
    // 左对齐
    println!("Number: {:<8.2}", number);
    // 右对齐
    println!("Number: {:>8.2}", number);
    // 以零填充
    println!("Number: {:08.2}", number);

    // 使用格式化占位符, 打印复杂数据结构
    let arr = [1, 2, 3, 4, 5];
    let tuple = (10, "hello", 4.5);

    println!("Array: {:?}", arr);
    println!("Tuple: {:?}", tuple);

    // 结构体打印，需要实现Debug trait
    let person = Person {
        name: String::from("Charlie"),
        age: 28,
    };

    println!("Person: {:?}", person);
    println!("Person (pretty print): {:#?}", person);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
