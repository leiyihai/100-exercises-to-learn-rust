use std::sync::mpsc::{Receiver, Sender};
use crate::store::TicketStore;

pub mod data;
pub mod store;

// 参考测试来理解预期的模式。
pub enum Command {
    Insert { todo!() },
    Get { todo!() }
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {}) => {
                todo!()
            }
            Ok(Command::Get {
                todo!()
            }) => {
                todo!()
            }
            Err(_) => {
                // 没有更多的发送者了，所以我们可以安全地退出
                // 并关闭服务器。
                break
            },
        }
    }
}
