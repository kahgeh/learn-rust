use tokio::fs::{File, metadata, remove_file};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::io;
use std::str;
use std::fs::Metadata;

async fn read_5_bytes(file_name: &str)->io::Result<()>{
    let mut f = File::open(file_name).await?;
    let mut buffer = [0;5];

    let n = f.read(&mut buffer).await?;

    println!("File content\n bytes: {:?}\n text: {}\n", buffer,  str::from_utf8(&buffer).unwrap());

    Ok(())
}
async fn read_all(file_name: &str)->io::Result<()>{
    let mut f = File::open(file_name).await?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;
    println!("File content\n bytes: {:?}\n text: {}\n", buffer,  str::from_utf8(&buffer).unwrap());
    Ok(())
}

async fn write_all( text: &str, file_name: &str)->io::Result<()> {
    let mut buffer = File::create(file_name).await?;
    buffer.write_all(text.as_bytes()).await?;
    Ok(())
}

async fn copy(file_name: &str) -> io::Result<()>{
    let mut reader: &[u8] =b"a stream of stuff";
    let mut file = File::create(file_name).await?;

    io::copy(&mut reader, &mut file).await?;
    
    Ok(())
}

#[tokio::main]
async fn main()->io::Result<()>{
    let file_name = "it_has_been_said.txt";
    write_all("ye with little faith, all good", &file_name).await?;

    read_5_bytes(&file_name).await?;

    read_all(&file_name).await?;

    copy(&file_name).await?;

    println!("cleaning up...");
    if let Ok(_) = metadata(file_name).await {
        remove_file(file_name).await?;
        println!("removed {}", file_name);
    }

    Ok(())
}
