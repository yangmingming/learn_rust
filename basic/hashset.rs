use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut set = HashSet::new();

    // 插入元素
    set.insert("apple");
    set.insert("banana");
    set.insert("orange");

    // 检查元素是否存在
    if set.contains("banana") {
        println!("Set contains banana");
    }

    // 删除元素
    set.remove("banana");

    // 通过引用遍历和通过迭代器遍历,性能上没有区别，因为两者底层都是通过迭代器实现的
    // 通过引用遍历集合
    for fruit in &set {
        println!("{}", fruit);
    }

    // 通过迭代器遍历集合
    for value in set.iter() {
        println!("{}", value);
    }

    // 获取集合的长度
    println!("Set length: {}", set.len());

    // 检查是否为空
    println!("Set is_empty: {}", set.is_empty());

    // set清空
    set.clear();
    println!("Set is_empty: {}", set.is_empty());

    // 使用自定义类型
    let mut people = HashSet::new();

    people.insert(Person {
        name: String::from("Alice"),
        age: 30,
    });
    people.insert(Person {
        name: String::from("Bob"),
        age: 25,
    });

    for person in &people {
        println!("{:?}", person);
    }
}
