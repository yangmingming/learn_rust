// 在 Rust 中，移动语义会在使用 Add 运算符时消耗（move）其参数。为了避免变量在加法操作后被移动，你可以让你的结构体实现 Copy 和 Clone trait。这样，变量在操作时会被按值复制而不是移动。
#[derive(Debug, Clone, Copy)]
struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

use std::ops::Add;
impl Add for WrappingU32 {
    type Output = Self;

    // 设置 Copy ，不会丢失所有权
    fn add(self, other : Self) -> Self {
        WrappingU32::new(self.value.wrapping_add(other.value))
    }
}

fn main() {
    let x = WrappingU32::new(42);
    let y = WrappingU32::new(31);
    let z = WrappingU32::new(u32::MAX);

    let result = x + y + z;
    println!("{:?} + {:?} + {:?} = {:?}", x, y , z, result);
}
