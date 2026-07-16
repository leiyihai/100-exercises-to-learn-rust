//! TODO: get the code to compile by **re-ordering** the statements
//!  在 `example` 函数中。你不允许修改
//!  `spawner` 函数，也不修改 `example` 中每行代码的作用。
//!   如果需要，你可以将现有语句包裹在 `{}` 块中。
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
