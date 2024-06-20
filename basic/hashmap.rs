use std::collections::HashMap;

// 使用自定义类型作为键
#[derive(Hash, Eq, PartialEq, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut map = HashMap::new();

    // 插入键值对
    map.insert("Alice", 30);
    map.insert("Bob", 25);
    map.insert("Charlie", 35);

    // 获取值
    if let Some(age) = map.get("Alice") {
        println!("Alice's age is {}", age);
    } else {
        println!("Alice is not in the map");
    }

    // 删除键值对
    map.remove("Bob");

    // 遍历 HashMap
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // 获取 HashMap 的长度
    println!("Map length: {}", map.len());

    // 自定义类型作为键
    let mut person = HashMap::new();

    person.insert(
        Person {
            name: String::from("Alice"),
            age: 30,
        },
        "Engineer",
    );
    person.insert(
        Person {
            name: String::from("Bob"),
            age: 25,
        },
        "Designer",
    );

    for (person, age) in &person {
        println!("{:?}: {}", person, age);
    }
}
