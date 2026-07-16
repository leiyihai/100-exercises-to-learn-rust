// 关于 `Sync` 没有太多需要练习的，只是需要记住一点。
fn outro() -> &'static str {
    "I have a good understanding of __!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a good understanding of Send and Sync!");
    }
}
