async fn say_world(){
    println!("world");
}

#[tokio::main]
pub async fn main() {
    // use_redis_sample().await?;
    let op = say_world();
    print!("hello");
    op.await;
}