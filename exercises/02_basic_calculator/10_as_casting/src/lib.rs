// TODO: based on what you learned in this section, replace `todo!()` with
//  转换后的正确值。

#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        let v: u32 = todo!();
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    fn u8_to_i8() {
        // 编译器足够聪明，知道值 255 无法放入
        // 在 i8 内部，所以它会抛出一个硬错误。我们故意禁用了
        // 这道防护栏，使这种（不好的）转换成为可能。
        // 编译器之所以能发现这个问题，是因为该值是一个字面量。
        // 如果使用的是变量，编译器就无法在编译时捕获到这个问题。
        // catch this at compile time.
        #[allow(overflowing_literals)]
        let x = { 255 as i8 };

        // 你可以使用与上面完全相同的表达式来解决这个问题，
        // 但这会违背练习的目的。相反，使用一个真正的
        // 一个在转换为 `u8` 时等同于 `255` 的 `i8` 值。
        let y: i8 = todo!();

        assert_eq!(x, y);
    }

    #[test]
    fn bool_to_u8() {
        let v: u8 = todo!();
        assert_eq!(true as u8, v);
    }
}
