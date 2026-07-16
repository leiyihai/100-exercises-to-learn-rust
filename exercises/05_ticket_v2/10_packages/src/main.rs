// 这是一个 `main.rs` 文件，因此 `cargo` 将其视为二进制目标的根。

// TODO: fix this broken import. Create a new library target in the `src` directory.
//   库目标应公开一个名为 `hello_world` 的公共函数，该函数不接受参数
//   不返回任何内容。
use packages::hello_world;

// 这是二进制程序的入口点。
fn main() {
    hello_world();
}
