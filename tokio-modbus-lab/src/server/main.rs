use futures::future;
use std::{thread, time::Duration};

use tokio_modbus::prelude::*;
use tokio_modbus::server;

struct ModBusServer;

impl server::Service for ModBusServer {
    type Request = Request;
    type Response = Response;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        match req {
            Request::ReadInputRegisters(_addr, cnt) => {
                let mut registers = vec![0; cnt as usize];
                registers[2] = 0x77;
                future::ready(Ok(Response::ReadInputRegisters(registers)))
            }
            _ => unimplemented!(),
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = "127.0.0.1:5502".parse().unwrap();
    let _server = thread::spawn(move || {
        server::tcp::Server::new(socket_addr).serve(|| Ok(ModBusServer));
    });
    // Give the server some time for stating up
    thread::sleep(Duration::from_secs(1));

    println!("Connecting client...");
    let mut ctx = tcp::connect(socket_addr).await?;
    println!("Reading input registers...");
    let rsp = ctx.read_input_registers(0x00, 7).await?;
    println!("The result is '{:?}'", rsp);

    Ok(())
}
