fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = [0; 100]; // 创建一个包含100个元素的数组，所有元素初始化为0

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}