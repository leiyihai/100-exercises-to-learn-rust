// TODO: you have something to do in each of the modules in this crate!
mod description;
mod status;
mod title;

// Rust 中一个常见的模式是将代码拆分为多个（私有）模块
// 然后在 crate 的根目录重新导出这些模块的公共部分。
//
// 这向用户隐藏了 crate 的内部结构，同时仍然
// 让你可以自由地组织代码。
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
// 我们不再需要将字段设为私有！
// 由于每个字段都封装了自己的校验逻辑，因此不存在
// `Ticket` 的使用者以破坏结构体不变量的方式修改字段
// 结构体的不变量。
//
// 但要注意：如果你的不变量跨越了多个字段，你就
// 需要确保这些不变量仍然保持，并回到
// 回到将字段设为私有。
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
