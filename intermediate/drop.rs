struct Resource {
    id: u32,
}

// 在 Rust 中，Drop 是一个非常重要的 trait，它定义了在值离开作用域时执行清理代码的方式。
impl Drop for Resource {
    fn drop(&mut self) {
        println!("Releasing resource with ID: {}", self.id);
    }
}

fn main() {
    let r1 = Resource { id: 1 };
    {
        let r2 = Resource { id: 2 };
        let r3 = Resource { id: 3 };
        // r2 和 r3 会在此作用域结束时被释放
        // 变量存储在栈中，LIFO(后进先出),因此drop时优先释放r3
    }
    // r1 会在这里被释放
}

