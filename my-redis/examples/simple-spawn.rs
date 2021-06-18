
#[tokio::main]
async fn main(){
    let v =vec![1,2,3];
    let val = 1;
    let handle = tokio::spawn(async move {
        println!("{:?} {}", v, val);
        "return value"
    });
    println!("main flow val {}", val); // ok as val implement copy trait
    //println!("main flow v {:?}", v); // fail because v reference have already moved
    let out = handle.await.unwrap();
    println!("GOT {}", out);
}