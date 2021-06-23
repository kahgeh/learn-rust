use tokio::io;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::sleep;
use std::time::Duration;
use std::str;

#[tokio::main]
async fn main()->io::Result<()>{
    let listener = TcpListener::bind("0.0.0.0:6142").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0;1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        println!("received {}({:?}), echoing in 5 seconds", str::from_utf8(&buf[..n]).unwrap() ,&buf[..n]);
                        sleep(Duration::new(5,0)).await;
                        if socket.write_all(&buf[..n]).await.is_err(){
                            eprintln!("fail to copy");
                            return;
                        }
                        println!("echoed");
                    },
                    Err(_) => {
                        return;
                    }
                }
            }
        });
    }
}