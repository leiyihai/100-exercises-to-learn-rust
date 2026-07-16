// TODO: Add the necessary trait bounds to `min` so that it compiles successfully.
//   请参考 `std::cmp` 模块的文档，了解你可能需要的 trait 的更多信息。
//
// 注意：有多种 trait bound 可以让编译器满意，但它们伴随着
// 不同的_语义_。我们将在课程后面讨论有序
// 集合（例如 BTreeMap）。

/// 返回两个值中的最小值。
pub fn min<T>(left: T, right: T) -> T {
    if left <= right {
        left
    } else {
        right
    }
}
