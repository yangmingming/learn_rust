fn main() {
    let mut vec = Vec::new();
    
    // 插入元素
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    // 访问元素
    println!("The first element is {}", vec[0]);
    
    // 遍历元素
    for element in &vec {
        println!("{}", element);
    }
    
    // 删除元素
    vec.pop();
    
    // 获取 Vec 的长度
    println!("The length of the vector is {}", vec.len());
}
