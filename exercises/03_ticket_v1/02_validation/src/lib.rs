struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    // TODO: implement the `new` function.
    //  应满足以下要求：
    //   - 只允许 `To-Do`、`In Progress` 和 `Done` 这三种状态。
    //   - `title` 和 `description` 字段不应为空。
    //   - `title` 长度最多为 50 字节。
    //   - `description` 长度最多为 500 字节。
    //  如果不满足任何要求，该方法应 panic。
    //  你可以在测试中找到所需的 panic 消息。
    //
    // 你将需要使用在之前练习中学到的知识，
    // 以及一些 `String` 方法。使用 Rust 标准库的文档
    // 来找到最合适的选项 -> https://doc.rust-lang.org/std/string/struct.String.html
    fn new(title: String, description: String, status: String) -> Self {
        todo!();
        Self {
            title,
            description,
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
