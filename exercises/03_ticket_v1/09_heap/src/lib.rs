pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  对应类型的正确**栈大小**。
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), todo!());
    }

    #[test]
    fn ticket_size() {
        // 这是一个派手的问题！
        // 这次“直觉”上的答案恰好是正确的答案，
        // 但总的来说，结构体的内存布局是一个更复杂的主题。
        // 如果你好奇，可以查看 Rust Reference 中的 "Type layout" 章节
        // https://doc.rust-lang.org/reference/type-layout.html 了解更多信息。
        assert_eq!(size_of::<Ticket>(), todo!());
    }
}
