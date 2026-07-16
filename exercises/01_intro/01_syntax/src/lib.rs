// TODO: fix the function signature below to make the tests pass.
//  务必阅读编译器的错误信息——Rust 编译器是你在这门课程中的结对编程
//  伙伴，它会经常引导你朝正确的方向前进！
//
// 输入参数应与返回类型具有相同的类型。
fn compute(a, b) -> u32 {
    // 不要修改函数体。
    a + b * 2
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 5);
    }
}