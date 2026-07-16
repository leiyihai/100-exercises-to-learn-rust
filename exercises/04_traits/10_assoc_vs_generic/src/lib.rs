// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  到 `n` 次幂。
//  trait 定义及其实现应该就足以让
//  测试编译通过。
//
// 建议：你可能会想编写一个通用实现来处理
// 一次处理所有情况。然而，这相当复杂，需要使用
// 额外的 crate（即 `num-traits`）。
// 即便如此，使用简单的宏来避免可能更好
// 高度通用实现的复杂性。查看
// 如果你感兴趣，可以看看《Little book of Rust macros》(https://veykril.github.io/tlborm/)
// interested in learning more about it.
// 但你不需要这样做：分别写三个也是完全可以的
// 手动实现。如果你好奇的话可以进一步探索。

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
