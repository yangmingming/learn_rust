fn main() {
    /*
      遍历范围如下
        1..5 -> [1, 5)
        1..=5 -> [1, 5]
        1.. -> [1, max]
        ..5 -> [min, 5)
        ..=5 -> [min, 5]
    */
    for number in 1..5 {
        println!("The number is: {}", number);
    }

    // 遍历数组,长度编译时确定，不可动态修改
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("The value is: {}", element);
    }

    // 遍历向量
    let vec = vec![100, 200, 300];
    for element in &vec {
        println!("The value is: {}", element);
    }

    // 遍历向量时同时提供索引和值
    // let vec = vec!["a", "b", "c"];
    for (index, value) in vec.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }

    // 循环遍历时修改
    let mut vec_mut = vec![1, 2, 3, 4, 5];
    for element in &mut vec_mut {
        *element *= 2;
    }
    println!("Modified vector: {:?}", vec_mut);

    // 二维数组和矩阵的遍历
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    for row in &matrix {
        print!("Matrix value: ");
        for value in row {
            print!("{} ", value);
        }
        println!("");
    }

    // for循环和模式匹配
    let vec_tup = vec![(1, 2), (3, 4), (5, 6)];
    for (x, y) in vec_tup {
        println!("x: {}, y: {}", x, y);
    }

    // 字符串单子符遍历
    let s = String::from("hello");
    for c in s.chars() {
        println!("Character: {}", c);
    }

    // for 循环搭配迭代器使用

    // for循环搭配迭代器map
    let old_vec = vec![1, 2, 3, 4, 5];
    // map 方法用于对迭代器的每个元素进行变换，并生成一个新的迭代器。
    let map_vec: Vec<i32> = old_vec.iter().map(|x| x * 2).collect();
    println!("After map, vec is: {:?}", map_vec);

    // filter 方法用于根据一个条件筛选迭代器的元素，并生成一个新的迭代器。
    let evens: Vec<i32> = old_vec.clone().into_iter().filter(|&x| x % 2 == 0).collect();
    println!("After filter and collect, vec is: {:?}", evens);

    // 将 map 和 filter 方法链式调用，以对元素进行变换和筛选。
    // 使用 `filter` 筛选出偶数，然后使用 `map` 将其乘以2
    let processed: Vec<i32> = old_vec
        .clone()
        .into_iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 2)
        .collect();
    println!("After map and filter, vec is: {:?}", processed);

    // flat_map 方法将每个元素映射为一个迭代器，然后将这些迭代器扁平化为一个单一的迭代器。
    // 使用 `flat_map` 生成每个数字的倍数
    let multiples: Vec<i32> =old_vec 
        .into_iter()
        .flat_map(|x| vec![x, x * 2, x * 3])
        .collect();
    println!("After flat_map, vec is: {:?}", multiples);

}
