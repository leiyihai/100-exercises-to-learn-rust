// 给定一个数字 `n`，返回斐波那契数列中的第 `n+1` 个数。
//
// 斐波那契数列定义如下：
//
// - 序列的第一个数是 0。
// - 序列的第二个数是 1。
// - 之后的每个数都是前两个数之和。
//
// 所以序列是：0, 1, 1, 2, 3, 5, 8, 13, 21，以此类推。
//
// 我们期望 `fibonacci(0)` 返回 `0`，`fibonacci(1)` 返回 `1`，
// `fibonacci(2)` 返回 `1`，以此类推。
pub fn fibonacci(n: u32) -> u32 {
    // TODO: implement the `fibonacci` function
    //
    // 提示：使用 `Vec` 来缓存你已经计算过的结果
    // 这样你就不必多次重新计算它们。
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
