mod command;

use crate::command::Command::*;
use mini_redis::client;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    // Create a new channel with a capacity of at most 32.
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };
        tx.send(cmd).await.unwrap();
        // Await the response
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Set {
            key: "foo".to_string(),
            val: b"bar".to_vec(),
            resp: resp_tx,
        };

        tx2.send(cmd).await.unwrap();
        // Await the response
        let res = resp_rx.await;
        println!("GOT = {:?}", res)
    });

    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Get { key, resp } => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                }
                Set { key, val, resp } => {
                    let res = client.set(&key, val.into()).await;
                    let _ = resp.send(res);
                }
            }
        }
    });
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
