use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, 
    net::TcpListener,
    sync::broadcast,
};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8000").await.unwrap(); // await allows the program to wait for a response

    let (tx, rx) = broadcast::channel::<String>(20);

    loop{
        let ( mut socket, _addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {

            let (reader,  mut writer) = socket.split();

            let mut reader = BufReader::new(reader);

            let mut line: String = String::new();

            loop{
    

            let bytes_read = reader.read_line(&mut line).await.unwrap();
            if bytes_read == 0{
            break;
            }

            writer.write_all(line.as_bytes()).await.unwrap();
            line.clear();

            }
        });
    }
}
