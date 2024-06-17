fn main() {
    let mut id = 0;
    let mut count = 0;

    while id < 11 {
        // break 举例
        if id == 10 {
            println!("break if id == 10");
            break;
        }

        // continue 奇数跳过举例
        if id % 2 != 0 {
            println!("continue if id == {}", id);
            id = id + 1;
            continue;
        }

        println!("id is: {}", id);
        id = id + 1;
        count += id;
    }

    println!("Exited the loop, id = {}, count = {}", id, count);
}
