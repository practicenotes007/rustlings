macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 修复宏调用
    my_macro!();
}