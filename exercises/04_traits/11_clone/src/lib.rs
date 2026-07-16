// TODO: add the necessary `Clone` implementations (and invocations)
//  使代码编译通过。

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    (ticket, ticket.summary())
}

pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}
