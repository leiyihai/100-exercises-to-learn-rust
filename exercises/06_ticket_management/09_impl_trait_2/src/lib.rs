// TODO: Rework the signature of `TicketStore::add_ticket` to use a generic type parameter rather
//  而不是 `impl Trait` 语法。

use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    // 使用 `Into<Ticket>` 作为 `ticket` 的类型参数，允许该方法接受任何可以
    // 无误地转换为 `Ticket` 的类型。
    // 这可以让方法的使用更优雅，因为它消除了 `.into()` 的语法噪音
    // 从调用处进行转换。不过，这可能会降低编译器错误消息的质量。
    pub fn add_ticket(&mut self, ticket: impl Into<Ticket>) {
        self.tickets.push(ticket.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    struct TicketDraft {
        pub title: TicketTitle,
        pub description: TicketDescription,
    }

    impl From<TicketDraft> for Ticket {
        fn from(draft: TicketDraft) -> Self {
            Self {
                title: draft.title,
                description: draft.description,
                status: Status::ToDo,
            }
        }
    }

    #[test]
    fn generic_add() {
        let mut store = TicketStore::new();
        // 如果 `add_ticket` 在参数位置使用 `impl Trait` 语法，这将无法编译。
        store.add_ticket::<TicketDraft>(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        });
    }
}
