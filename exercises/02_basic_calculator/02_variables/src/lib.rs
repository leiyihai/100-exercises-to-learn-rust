// 👇 下面以 `///` 开头的行称为**文档注释**。
//    它们将文档附加到紧随其后的项上。在这里是 `speed` 函数。
//    如果你在这个练习的目录下运行 `cargo doc --open`，Rust 会
//    从这些注释生成 HTML 文档并在浏览器中打开它。

/// 给定一段行程的起点和终点，以及完成它所花费的时间，
/// 计算平均速度。
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: define a variable named `distance` with the right value to get tests to pass
    //  你需要标注 `distance` 的类型吗？为什么需要或不需要？

    // 不要修改下面的行
    distance / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(speed(10, 30, 10), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(speed(10, 31, 10), 2);
    }
}
