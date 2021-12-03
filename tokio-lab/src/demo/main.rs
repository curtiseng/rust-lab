use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use tokio::runtime::Builder;

// #[tokio::main(flavor = "multi_thread", worker_threads = 2)]
fn main() {
    Builder::new_multi_thread().max_threads(2)
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("tokio-worker-{}", id)
        }).enable_all()
            .build().unwrap()
        .block_on(async {
            loop  {
                work().await; // main thread
                println!("THREAD_NAME: {}, THREAD_ID: {:?}", std::thread::current().name().unwrap(), std::thread::current().id());
                tokio::spawn(work()); // mostly one worker
                tokio::spawn(work()); // another worker
                std::thread::sleep(Duration::from_millis(1000));
            }
        });
    // loop  {
    //     println!("THREAD_NAME: {}, THREAD_ID: {:?}", std::thread::current().name().unwrap(), std::thread::current().id());
    //     tokio::spawn(work());
    //     std::thread::sleep(Duration::from_millis(1000));
    // }
}

async fn work() {
    std::thread::sleep(Duration::from_millis(100));
    println!("WORK_THREAD_NAME: {}, THREAD_ID: {:?}", std::thread::current().name().unwrap(), std::thread::current().id());
}