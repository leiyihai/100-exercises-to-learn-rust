// TODO: implement a multi-threaded version of the `sum` function
//  使用 `spawn` 和 `join`。
//  给定一个整数向量，将其分成两半，
//  在单独的线程中求和每一半。

// 注意：我们无法测试函数的*实现方式*，
// 我们只能验证它是否产生正确的结果。
// 你_可以_通过直接返回 `v.iter().sum()` 来通过这个测试，
// 但这会违背练习的目的。
//
// 提示：你无法让生成的线程_借用_
// 直接使用向量的切片。你需要分配新的
// 为原始向量的每一半分配新的向量。我们之后会看到为什么
// 这在下一个练习中是必要的。
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
