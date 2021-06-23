use mini_redis::client;
use bytes::Bytes;
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>
    }
}

#[tokio::main]
async fn main(){
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();
    let mut client = client::connect("0.0.0.0:6379").await.unwrap();

    let manager = tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get {key, resp} => {
                    let val = client.get(&key).await;
                    let _ = resp.send(val); // swallow error, and no await required for oneshot, will succeed or fail right away

                },
                Set {key, val, resp} =>{
                    let val = client.set(&key, val).await;
                    let _ = resp.send(val);
                }
            }
        }
    });

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Get {
            key: "hello".to_string(),
            resp: resp_tx
        };

        tx.send(cmd).await.unwrap();
        let resp = resp_rx.await.unwrap();
        println!("GOT = {:?}", resp);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Set {
            key: "hello".to_string(),
            val: "world".into(),
            resp: resp_tx
        };
        tx2.send(cmd).await.unwrap();
        let resp = resp_rx.await.unwrap();
        println!("GOT = {:?}", resp);
    });


    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}