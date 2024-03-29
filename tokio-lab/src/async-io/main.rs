use std::io;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> io::Result<()> {
    let path = Path::new(".");
    println!("{:?}", path.as_os_str());

    let file = std::fs::File::create("tokio-lab/data.txt").expect("create failed");
    println!("文件创建成功:{:?}", file);
    let mut f = File::open("tokio-lab/foo.txt").await?;
    let mut buffer = [0; 10];
    // read up to 10 bytes
    let n = f.read(&mut buffer[..]).await?;
    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}
