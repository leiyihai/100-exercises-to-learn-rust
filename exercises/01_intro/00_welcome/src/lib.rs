// 这是一个 Rust 文件。它是一个扩展名为 `.rs` 的纯文本文件。
//
// 与大多数现代编程语言一样，Rust 支持注释。你现在看到的就是注释！
// 注释会被编译器忽略；你可以利用它们为代码添加说明和
// explanations.
// Rust 中有多种编写注释的方式，每种都有自己的用途。
// 现在我们使用最常见的一种：行注释。
// 从 `//` 到行末的所有内容都被视为注释。

// 练习中将包含 `TODO`、`todo!()` 或 `__` 标记，以引起你对需要编写代码的行的注意
// where you need to write code.
// 你需要用你自己的代码替换这些标记来完成练习。
// 有时只需要写一行代码，有时则需要写更长的段落。
// longer sections.
//
// 如果你在某个练习上卡住超过 10 分钟，找讲师帮忙！我们随时为你提供帮助！
// 你也可以在 `solutions` git 分支中找到所有练习的解答。
fn greeting() -> &'static str {
    // TODO: 修复我 👇
    "I'm ready to __!"
}

// 你的解答将由一组测试自动验证。
// 你可以直接在终端中调用 `cargo test` 命令来运行这些测试，
// 从这个练习目录的根目录运行。这就是 `wr` 命令在底层为你做的事情。
// under the hood.
//
// Rust 允许你将测试与代码写在一起。
// `#[cfg(test)]` 属性告诉编译器仅在
// 运行测试时（即当你运行 `cargo test` 时）编译以下代码。
// 你将在课程的后面部分学习更多关于属性和测试的内容。
// 现在，你只需要知道要寻找 `#[cfg(test)]` 属性来找到那些将验证你的解答正确性的测试
// that will be verifying the correctness of your solutions!
//
// ⚠️ **不要修改测试** ⚠️
// 它们是为了帮助你验证解答。你只应修改被测试的代码，
// tested, not the tests themselves.
#[cfg(test)]
mod tests {
    use crate::greeting;

    #[test]
    fn test_welcome() {
        assert_eq!(greeting(), "I'm ready to learn Rust!");
    }
}
