use rumqttc::{MqttOptions, AsyncClient, QoS, LastWill};
use tokio::{task, time};
use std::time::Duration;
use log::info;
use log4rs;

/// 模拟两个设备
/// A设备 pub 10条消息, 并订阅

#[tokio::main(worker_threads = 3)]
async fn main() {
    log4rs::init_file("mqtt-lab/log4rs.yaml", Default::default()).unwrap();
    // 上报客户端
    let mut client_report_options = MqttOptions::new("mqtt-async", "127.0.0.1", 1883);
    let will = LastWill::new("topic", "I'm reporter, good bye",QoS::AtMostOnce, false);
    client_report_options.set_keep_alive(5)
        .set_clean_session(false)
        .set_last_will(will);
    let (reporter, mut event_loop) = AsyncClient::new(client_report_options, 10);

    reporter.subscribe("topic", QoS::AtMostOnce).await.unwrap();

    task::spawn(async move {
        for i in 0..10 {
            info!("publish: {}", i.to_string().as_str());
            reporter.publish("topic", QoS::AtMostOnce, false, String::from("{\"hello\" : \"world-\"") + &i.to_string() + "}").await.unwrap();
            time::sleep(Duration::from_millis(1000)).await;
        }
    });

    loop {
        let notification = event_loop.poll().await.unwrap();
        println!("Received = {:?}", notification);
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}