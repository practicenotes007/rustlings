#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // 修复编译器错误，处理 `Some` 和 `None` 的情况
    match optional_point {
        Some(ref p) => println!("Co-ordinates are {},{}", p.x, p.y),
        None => println!("No point found!"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}