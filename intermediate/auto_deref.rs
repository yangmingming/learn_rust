struct Point {
    x: i32,
    y: i32,
}

/*
    Rust 的自动解引用（auto-deref）功能在方法调用和属性访问中简化了代码，使其更易读。以下是一些自动解引用的情况：
    * 访问结构体字段时自动解引用。
    * 调用方法时自动解引用，例如，当你有一个引用类型对象并调用它的方法时，Rust 会自动解引用以便调用适当的方法。
*/
impl Point {
    // 方法接受 self 的不可变引用 &self
    fn print(&self) {
        // 结构体字段访问时,自动解引用
        println!("Point: ({}, {})", self.x, self.y);
    }

    fn eq(&self, x: &i32, y:&i32) -> bool {
        // 结构体字段访问时,自动解引用; 但是引用变量需要通过*手动解引用
        // self.x == x && self.y == y       // help: consider dereferencing the borrow
        (self.x == *x) && (self.y == *y)
    }
}

fn main() {
    let point = Point{ x:1, y:2};
    point.print();
    println!("{}", point.eq(&1, &2));
}
