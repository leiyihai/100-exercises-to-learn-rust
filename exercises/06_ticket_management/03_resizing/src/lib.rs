#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // max capacity reached
        assert_eq!(v.capacity(), 2);

        v.push(3); // beyond capacity, needs to resize

        // 你能猜到新的容量是多少吗？
        // 请注意，标准库并不保证
        // 用于调整 vector 大小的算法，因此将来可能会发生变化。
        assert_eq!(v.capacity(), todo!());
    }
}
