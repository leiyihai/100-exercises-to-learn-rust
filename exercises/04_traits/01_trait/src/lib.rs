// 定义一个名为 `IsEven` 的 trait，其中包含一个方法 `is_even`，如果 `self` 是
// 偶数则返回 `true`，否则返回 `false`。
//
// 然后为 `u32` 和 `i32` 实现该 trait。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42u32.is_even());
        assert!(!43u32.is_even());
    }

    #[test]
    fn test_i32_is_even() {
        assert!(42i32.is_even());
        assert!(!43i32.is_even());
        assert!(0i32.is_even());
        assert!(!(-1i32).is_even());
    }
}
