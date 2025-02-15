// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // 导入is_even函数
    use super::*;

    #[test]
    fn you_can_assert() {
        // 测试偶数
        assert!(is_even(2));
        // 测试奇数
        assert!(!is_even(3));
    }
}
