// TODO: Fix the compiler error.
fn main() {
    let mut x = 3; // 修改: 将 x 声明为可变的

    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}