mod ticket {
    struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        fn new(title: String, description: String, status: String) -> Ticket {
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
    }
}

// TODO: **Exceptionally**, you'll be modifying both the `ticket` module and the `tests` module
//  在这个练习中。
#[cfg(test)]
mod tests {
    // TODO: Add the necessary `pub` modifiers in the parent module to remove the compiler
    //  关于下面 use 语句的错误。
    use super::ticket::Ticket;

    // 不过要小心！我们不希望在修改后这个函数还能编译通过
    // 可见性以使 use 语句编译通过！
    // 一旦你确认它确实无法编译通过，就把它注释掉。
    fn should_not_be_possible() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());

        // 在尝试运行这个练习时，你应该会看到以下错误：
        //
        // error[E0616]: 结构体 `Ticket` 的字段 `description` 是私有的
        //    |
        //    |              assert_eq!(ticket.description, "A description");
        //    |                         ^^^^^^^^^^^^^^^^^^
        //
        // TODO: Once you have verified that the below does not compile,
        //   把该行注释掉以继续下一个练习！
        assert_eq!(ticket.description, "A description");
    }

    fn encapsulation_cannot_be_violated() {
        // 这也应该是不可能的，会出现与上面遇到的类似错误。
        // (It will throw a compilation error only after you have commented the faulty line
        // （在之前的测试中——下一个编译阶段！）
        //
        // 这证明了 `Ticket::new` 现在是获取 `Ticket` 实例的唯一方式。
        // 创建包含非法标题或描述的工单是不可能的！
        //
        // TODO: Once you have verified that the below does not compile,
        //   把这些行注释掉以继续下一个练习！
        let ticket = Ticket {
            title: "A title".into(),
            description: "A description".into(),
            status: "To-Do".into(),
        };
    }
}
