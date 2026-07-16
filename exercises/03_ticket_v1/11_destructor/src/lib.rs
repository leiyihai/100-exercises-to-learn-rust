// 我们需要更多的机制来为析构函数编写一个合适的练习。
// 我们将在后续章节中讲解 trait 和
// 内部可变性。
fn outro() -> &'static str {
    "I have a basic understanding of __!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}
