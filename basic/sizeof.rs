// 内存布局是一件比较复杂的元素，需要考虑"Data layout"
// https://doc.rust-lang.org/nomicon/data.html for more information.
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

pub struct Stu1 {
    id: u32,
    sex: u8,
}

pub struct Stu2 {
    id: u32,
    sex: u8,
    score: u32,
}

pub struct Stu3 {
    id: u8,
    sex: u8,
}

/*
String 的内存布局如下图所示：
+------------------+------------------+------------------+
|     pointer      |      length      |     capacity     |
+------------------+------------------+------------------+
|  8 bytes (ptr)   |  8 bytes (len)   |  8 bytes (cap)   |
+------------------+------------------+------------------+

引用举例, 注意:并不是所有的指针都指向堆。
           --------------------------------------
           |                                    |
      +----v----+--------+----------+      +----|----+
Stack | pointer | length | capacity |      | pointer |
      |  |      |   3    |    5     |      |         |
      +--|  ----+--------+----------+      +---------+
         |          s                           r
         |
         v
       +---+---+---+---+---+
Heap   | H | e | y | ? | ? |
       +---+---+---+---+---+

*/

use std::mem::size_of;
fn main() {
    println!("size_of::<u8>() = {}", size_of::<u8>());
    println!("size_of::<u16>() = {}", size_of::<u16>());
    println!("size_of::<i32>() = {}", size_of::<i32>());
    println!("size_of::<bool>() = {}", size_of::<bool>());
    println!("size_of::<String>() = {}", size_of::<String>());
    println!("size_of::<Ticket>() = {}", size_of::<Ticket>());

    // 引用
    println!("size_of::<&u8>() = {}", size_of::<&u8>());
    println!("size_of::<&mut u8>() = {}", size_of::<&mut u8>());
    println!("size_of::<&u16>() = {}", size_of::<&u16>());
    println!("size_of::<&String>() = {}", size_of::<&String>());

    // 内存对齐
    println!("size_of::<Stu1>() = {}", size_of::<Stu1>());  // 8 = 4(u32) + 1(u8) + 3(padding)
    println!("size_of::<Stu2>() = {}", size_of::<Stu2>());  // 12 = 4(u32) + 1(u8) + 3(padding) + 4(u32)
    println!("size_of::<Stu3>() = {}", size_of::<Stu3>());  // 2 = 1(u8) + 1(u8)
}
