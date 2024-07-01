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

    // 可变引用迭代
    for element in &mut vec {
        *element += 1;
    }
    println!("{:?}", vec);
    
    // 删除元素
    vec.pop();
    
    // 获取 Vec 的长度
    println!("The length of the vector is {}", vec.len());

    // 定义时初始化
    let vec = vec![1, 2, 3];

    // 索引的类型必须是 usize
    let n:u32 = 2;   // slice indices are of type `usize` or ranges of `usize`
    let n = n as usize;
    println!("index [{}] = {}", n, vec[n]);

    // 手动指定存储空间
    let mut numbers = Vec::with_capacity(3);
    // 通过实验可以确认,capacity是按照一定的增长因子增长的
    for i in 1..5 {
        numbers.push(i);
        println!("push {}, len = {}, capacity = {}", i, numbers.len(), numbers.capacity());
    }
    println!("{:?}", numbers);

/*
          +---------+--------+----------+
    Stack | pointer | length | capacity |
          |  |      |   2    |    3     |
          +--|------+--------+----------+
             |
             |
             v
           +---+---+---+
    Heap:  | 1 | 2 | ? |
           +---+---+---+
*/

}
