use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc};

fn simple_threading(){
    thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from the spawned thread!",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn wait_for_completion(){
    let handle=thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from the spawned thread!",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn use_closure() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move||{
        println!("Here's a vector: {:?}", v)
    });

    //drop(v);
    handle.join().unwrap();
}

fn send_simple_message(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("sent {}", val); // not possible because val would have been moved
    });

    let received = rx.recv().unwrap();
    print!("received : {}", received);
}

fn send_multiple_messages(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    for received in rx {
        println!("received: {}", received);
    }
}

fn multiple_producer_send_messages(){
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("another"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    for received in rx {
        println!("received: {}", received);
    }
}

fn simple_mutex_usage(){
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m={:?}", m);
}

fn access_mutext_from_multiple_threads(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut count = counter.lock().unwrap();
            *count=*count+1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn main() {
    //simple_threading();
    //wait_for_completion();
    //use_closure();
    //send_simple_message();
    //send_simple_message();
    //send_multiple_messages();
    //multiple_producer_send_messages();
    //simple_mutex_usage();
    access_mutext_from_multiple_threads();
}
