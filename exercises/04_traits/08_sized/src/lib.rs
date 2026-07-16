pub fn example() {
    // 尝试获取 str（或任何其他 DST）的大小
    // 通过 `std::mem::size_of` 获取大小会导致编译时错误。
    //
    // TODO: Comment out the following line and move on to the next exercise.
    std::mem::size_of::<str>();
}
