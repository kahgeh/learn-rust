use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::{fs, thread};
use std::time::Duration;
use mywebserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep =b"GET /sleep HTTP/1.1\r\n";
    let (status, filename) = if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("200 OK", "index.html")
    } else if buffer.starts_with(get) {
        ("200 OK", "index.html")
    } else {
        ("404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
                            status,
                            contents.len(),
                            contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
