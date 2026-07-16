// 使用 `while` 循环重写阶乘函数。
pub fn factorial(n: u32) -> u32 {
    // `todo!()` 宏是一个占位符，编译器
    // 将其解释为“我稍后再处理这个”，从而
    // suppressing type errors.
    // 它在运行时 panic。
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
