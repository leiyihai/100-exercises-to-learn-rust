// TODO: 这是一个孤儿规则违规的例子。
//  我们正在为来自 `std` 的外部类型（`u32`）实现外部 trait（`PartialEq`）——
//  违反了孤儿规则。
//  查看编译器的报错信息，熟悉一下它的样子。
//  然后删除下面的代码，继续下一个练习。

impl PartialEq for u32 {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
