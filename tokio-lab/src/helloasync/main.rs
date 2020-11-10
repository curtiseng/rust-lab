#[tokio::main]
async fn main() {
    let op = say_world();
    println!("hello");
    op.await;
}

async fn say_world() {
    println!("world")
}