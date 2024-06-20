use std::collections::LinkedList;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    // 长度
    println!("The length of the linked list is {}", list.len());

    // 链头添加数据
    list.push_front(0);
    // 链尾添加数据
    list.push_back(99);

    // 打印初始列表
    println!("Initial list: {:?}", list);

    for element in list.iter_mut() {
        *element += 10;
    }

    // 打印修改后的列表
    println!("Initial list: {:?}", list);

    // 链头数据弹出
    if let Some(value) = list.pop_front() {
        println!("Removed pop value: {}", value);
    } else {
        println!("The list is empty.");
    }
    // 链尾数据弹出
    if let Some(value) = list.pop_back() {
        println!("Removed back value: {}", value);
    } else {
        println!("The list is empty.");
    }

    // 普通遍历
    for element in list.iter() {
        println!("{}", element);
    }

    // 判空
    println!("List is empty: {}", list.is_empty());

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&11));
    assert_eq!(iter.next(), Some(&12));
    assert_eq!(iter.next(), Some(&13));
    assert_eq!(iter.next(), None);
}
