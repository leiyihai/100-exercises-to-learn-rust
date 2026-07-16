/// 给定一段行程的起点和终点，以及完成该行程所花费的时间，
/// 计算行程的平均速度。
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: Panic with a custom message if `time_elapsed` is 0

    (end - start) / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // 👇 使用 `#[should_panic]` 注解，我们可以断言我们期望被测试的代码 panic。
    //    期望被测试的代码 panic。我们还可以使用 `expected` 检查 panic 消息。
    //    这都是 Rust 内置测试框架的一部分！
    #[should_panic(expected = "The journey took no time at all. That's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
