use std::io;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::path::Path;

#[tokio::main]
async fn main() -> io::Result<()> {
    let path = Path::new(".");
    println!("{:?}", path.as_os_str());

    let file = std::fs::File::create("data.txt").expect("create failed");
    println!("文件创建成功:{:?}",file);
    let mut f = File::open("./foo.txt").await?;
    let mut buffer = [0; 10];
    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;
    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}