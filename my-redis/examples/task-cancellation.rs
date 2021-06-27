use tokio::sync::oneshot;
use tokio::time::sleep;
use std::time::Duration;

async fn cancellation_using_select(){
    let (cancellation_tx, cancellation_rx) = oneshot::channel::<u8>();
    let (tx, rx) = oneshot::channel::<u8>();

    let cancellation = tokio::spawn(async {
        println!("sending cancellation request");
        cancellation_tx.send(0);
    });

    let f = tokio::spawn(async {
        sleep(Duration::new(5,0)).await;
        tx.send(0);
    });

    tokio::select! {
        val= rx => {
            println!("Completed task {:?}", val)
        }
        val = cancellation_rx => {
            println!("Cancellation received");
        }
    }

    let _ = cancellation.await.unwrap();
    println!("end of cancellation example, task was not completed");
}

async fn some_operation()-> String {
    sleep(Duration::new(2,0));
    String::from("hello")
}

async fn some_example() {
    let (mut tx1, rx1) =oneshot::channel();
    let (tx2, rx2)=oneshot::channel();

    tokio::spawn(async {
        tokio::select! {
            val = some_operation() => {
                let _ = tx1.send(val);
            }
            _ = tx1.closed() => {
                println!("tx1 closed");
            }
        }
    });

    tokio::spawn(async {
       let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }

}
#[tokio::main]
async fn main() {
    cancellation_using_select().await;
    some_example().await;
}