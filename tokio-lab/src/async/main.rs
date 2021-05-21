use tokio::runtime;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::thread;
use std::fmt::Debug;
use tokio::runtime::Runtime;

fn main() {

    let mqtt_rt  = Runtime::new().unwrap();
    let rpc_rt  = runtime::Builder::new_current_thread().build().unwrap();
    // Spawn the root task
    mqtt_rt.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        println!("{}", thread::current().name().unwrap());
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                let mut buf = [0; 1024];

                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            println!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        println!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    });
    rpc_rt.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
        println!("{}",thread::current().name().unwrap());
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                let mut buf = [0; 1024];

                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            println!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        println!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    });
}


async fn say_world() {
    println!("world")
}