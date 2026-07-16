// TODO: Set `move_forward` to `true` in `ready` when you think you're done with this exercise.
//  随时可以请讲师来验证你的解答！
use channels::data::TicketDraft;
use channels::{launch, Command};
use std::time::Duration;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn a_thread_is_spawned() {
    let sender = launch();
    std::thread::sleep(Duration::from_millis(200));

    sender
        .send(Command::Insert(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        }))
        // 如果线程不再运行，这将会 panic
        // 因为 channel 将被关闭。
        .expect("Did you actually spawn a thread? The channel is closed!");
}

#[test]
fn ready() {
    // 在这个练习中，我们能自动检查的内容很少，
    // 因为我们的服务器没有暴露任何**读取**操作。
    // 我们无法知道插入操作是否真的在执行，以及它们
    // 是否正确执行。
    let move_forward = false;

    assert!(move_forward);
}
