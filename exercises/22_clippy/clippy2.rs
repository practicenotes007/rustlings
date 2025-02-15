fn main() {
    let mut res = 42;
    let option = Some(12);
    // 修复Clippy lint，使用if let来处理Option
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}