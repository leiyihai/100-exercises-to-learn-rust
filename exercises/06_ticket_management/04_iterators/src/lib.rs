use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Let's start sketching our ticket store!
//  第一个任务：在 `TicketStore` 上实现 `IntoIterator`，以允许遍历所有工单
//  使用 `for` 循环遍历其中包含的所有元素。
//
// 提示：在这种情况下，你不必实现 `Iterator` trait。
//   你应该将迭代*委托*给 `TicketStore` 中的 `Vec<Ticket>` 字段。
//   查看标准库中 `Vec` 的文档，找到正确的类型
//   从 `into_iter` 返回。
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

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<_> = store.clone().into_iter().collect();
        assert_eq!(tickets, store.tickets);
    }
}
