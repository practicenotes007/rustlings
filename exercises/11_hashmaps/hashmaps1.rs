// 一个水果篮子以哈希映射的形式定义。键代表水果的名称，值代表该特定水果的数量。
// 你需要在篮子里至少放3种不同的水果（例如苹果、香蕉、芒果），并且所有水果的总数应至少为5。

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: 声明哈希映射。
    let mut basket = HashMap::new();

    // 已经为你准备了两个香蕉 :)
    basket.insert(String::from("banana"), 2);

    // TODO: 在你的篮子里放更多的水果。
    basket.insert(String::from("apple"), 3);
    basket.insert(String::from("mango"), 1);

    basket
}

fn main() {
    // 你可以在这里可选地进行实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}