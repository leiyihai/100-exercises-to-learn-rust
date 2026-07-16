// TODO: based on what we just learned about ownership, it sounds like immutable references
//   非常适合我们的访问器方法。
//   修改 `Ticket` 现有的访问器方法，改为接受引用
//   接受 `self` 作为参数，而不是获取其所有权。

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn title(self) -> String {
        self.title
    }

    pub fn description(self) -> String {
        self.description
    }

    pub fn status(self) -> String {
        self.status
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;

    #[test]
    fn works() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        // 如果你按要求修改了签名，这应该能编译通过：
        // 我们可以一个接一个地调用这些方法，因为它们借用了 `self`
        // 而不是获取其所有权。
        assert_eq!(ticket.title(), "A title");
        assert_eq!(ticket.description(), "A description");
        assert_eq!(ticket.status(), "To-Do");
    }
}
