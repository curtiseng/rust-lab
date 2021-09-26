use tokio_modbus::prelude::*;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = "127.0.0.1:502".parse().unwrap();

    let mut ctx = tcp::connect(socket_addr).await?;
    let data = ctx.write_single_register(0x00, 7).await?;
    println!("Response is '{:?}'", data);

    Ok(())
}
