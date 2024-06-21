// trait 定义
trait Greet {
    fn greet(&self) -> String;
}

struct Person {
    name: String,
}

// 为特定结构实现trait
impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, my name is {}", self.name)
    }
}

// 定义trait添加默认实现
trait Greet2 {
    fn greet2(&self) -> String {
        String::from("Hello!")
    }
}

// 未实现greet2,则使用默认实现
impl Greet2 for Person {}

// trait 作为参数
fn greet_person(person: &impl Greet) {
    println!("{}", person.greet());
}

// 或者使用泛型约束
fn greet_person2<T: Greet>(person: &T) {
    println!("{}", person.greet());
}

struct Student {
    id: u32,
    name: String,
}

impl Greet for Student {
    fn greet(&self) -> String {
        format!("Hello, my name is {}, I'm a student", self.name)
    }
}

fn main() {
    // trait 使用
    let person = Person {
        name: String::from("Alice"),
    };
    println!("{}", person.greet());     // 使用自定义
    println!("{}", person.greet2());    // 使用默认实现

    greet_person(&person);  // 使用引用传递
    greet_person2(&person); // 使用引用传递

    let student = Student {
        id: 123,
        name: String::from("Alice"),
    };
    greet_person(&student);  // 只要实现了Greet,都可以作为 greet_person 的参数
}
