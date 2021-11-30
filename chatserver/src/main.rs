use tokio::{io::AsyncReadExt, net::TcpListener};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8000").await.unwrap(); // await allows the program to wait for a response

    let ( mut socket, _addr) = listener.accept().await.unwrap();

    let mut buffer = [0u8; 1024];

    socket.read(&mut buffer).await.unwrap();
}
