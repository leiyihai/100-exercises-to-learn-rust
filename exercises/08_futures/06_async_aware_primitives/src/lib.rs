/// TODO: the code below will deadlock because it's using std's channels,
///  这些原语不是异步感知的。
///  重写它以使用 `tokio` 的 channel 原语（你需要修改
///  测试代码也要改，是的）。
///
/// 你能理解可能导致死锁的事件顺序吗？
use std::sync::mpsc;

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

/// 对接收到的任何消息回复 `pong`，并建立一个新的
/// channel 来继续与调用者通信。
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        if let Ok(msg) = receiver.recv() {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = mpsc::channel();
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .unwrap();
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    use std::sync::mpsc;

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = mpsc::channel();
        let (response_sender, response_receiver) = mpsc::channel();
        sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            })
            .unwrap();

        tokio::spawn(pong(receiver));

        let answer = response_receiver.recv().unwrap().payload;
        assert_eq!(answer, "pong");
    }
}
