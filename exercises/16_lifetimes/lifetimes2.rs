// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz"); // 将 string2 的声明移到外部作用域
    let result;
    {
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
}