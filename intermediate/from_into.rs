use std::convert::TryInto;

#[derive(Debug)]
struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(s: u32) -> WrappingU32 {
        WrappingU32 {
            value: s
        }
    }
}

// 在 Rust 中，.into() 是一个 trait 方法，用于将一个类型转换为另一个类型.具体的目标类型取决于 Rust 的类型推断或者代码中明确指定的类型。
fn main() {
    let wrapping: WrappingU32 = 42.into();
    println!("{:?}", wrapping);
    let wrapping = WrappingU32::from(42);
    println!("{:?}", wrapping);

    let num : u8 = 42;
    // 注: Rust 会自动从小范围转换大范围,因为转换是安全的，没有数据范围的问题
    let num_u16 : u16 = num.into();
    println!("num u16 = {}", num_u16);
    let num_i16 : i16 = num.into();
    println!("num i16 = {}", num_i16);
    let num_u32 : u32 = num.into();
    println!("num u32 = {}", num_u32);
    let num_i32 : i32 = num.into();
    println!("num i32 = {}", num_i32);

    let num: u32 = 42;
    let num: u32 = u32::MAX;
    // 如下转换会造成数据精度丢失, 因此编译会报错
    // let num_u16: u16 = num.into();
    println!("Converted before, u32 num = {}", num);        // u32::MAX时,4294967295

    // 要执行这样的转换，你需要使用 .try_into() 方法，并处理可能的溢出错误，或者使用 as 关键字进行显式的类型转换。
    // as 转换
    let num_u16: u16 = num as u16; // 显式类型转换
    println!("Converted by as successfully: {}", num_u16);  // u32::MAX时,65535,直接截断

    // 尝试将 u32 转换为 u16, 通过 try_into
    let result: Result<u16, _> = num.try_into();
    match result {
        Ok(num_u16) => {
            println!("Converted by try_info successfully: {}", num_u16);
        }
        Err(_) => {
            println!("Conversion by try_info failed: value too large for u16");
        }
    }
}
