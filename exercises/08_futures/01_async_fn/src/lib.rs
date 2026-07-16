use tokio::net::TcpListener;

// TODO: write an echo server that accepts incoming TCP connections and
//  将接收到的数据回显给客户端。
//  `echo` 在处理完一个连接后不应返回，而应该
//  继续接受新的连接。
//
// 提示：你应该依赖 `tokio` 的结构体和方法来实现回显服务器。
// 具体来说：
// - 使用 `tokio::net::TcpListener::accept` 处理下一个传入连接
// - 使用 `tokio::net::TcpStream::split` 从套接字获取读取端和写入端
// - 使用 `tokio::io::copy` 将数据从读取端复制到写入端
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_echo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(echo(listener));

        let requests = vec!["hello", "world", "foo", "bar"];

        for request in requests {
            let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (mut reader, mut writer) = socket.split();

            // 发送请求
            writer.write_all(request.as_bytes()).await.unwrap();
            // 关闭套接字的写入端
            writer.shutdown().await.unwrap();

            // 读取响应
            let mut buf = Vec::with_capacity(request.len());
            reader.read_to_end(&mut buf).await.unwrap();
            assert_eq!(&buf, request.as_bytes());
        }
    }
}
