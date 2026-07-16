/// 如果 `n` 是偶数，返回 `12`，
/// 如果 `n` 能被 `3` 整除，返回 `13`，
/// 否则返回 `17`。
fn magic_number(n: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::magic_number;

    #[test]
    fn one() {
        assert_eq!(magic_number(1), 17);
    }

    #[test]
    fn two() {
        assert_eq!(magic_number(2), 12);
    }

    #[test]
    fn six() {
        assert_eq!(magic_number(6), 12);
    }

    #[test]
    fn nine() {
        assert_eq!(magic_number(9), 13);
    }

    #[test]
    fn high() {
        assert_eq!(magic_number(233), 17);
    }
}
