use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(todo!()),
}

// 通过生成服务器线程来启动系统。
// 它返回一个 `Sender` 实例，之后可以使用该实例
// 供一个或多个客户端与服务器交互。
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  进入循环：等待命令出现在
//  命令出现在 channel 中，然后执行它，然后开始等待
//  等待下一个命令。
pub fn server(receiver: Receiver<Command>) {}
