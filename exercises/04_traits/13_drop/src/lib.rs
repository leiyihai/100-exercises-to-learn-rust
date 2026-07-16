// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  除非对其执行了某个操作。
//  你可以从下面的测试中看到预期的 API。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // bomb 在丢弃时应 panic
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // bomb 在丢弃时不应 panic
        // 因为它已经被解除
    }
}
