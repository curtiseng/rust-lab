use rumqttc::{MqttOptions, AsyncClient, QoS, LastWill};
use tokio::{task, time};
use std::time::Duration;

/// 模拟两个设备
/// A设备 pub 10条消息, 并订阅

#[tokio::main(max_threads = 3)]
async fn main() {
    // 上报客户端
    let mut client_report_options = MqttOptions::new("mqtt-async", "127.0.0.1", 1883);
    let will = LastWill::new("topic", QoS::ExactlyOnce, "I'm reporter, good bye");
    client_report_options.set_keep_alive(5)
        .set_clean_session(false)
        .set_last_will(will);
    let (reporter, mut event_loop) = AsyncClient::new(client_report_options, 10);

    reporter.subscribe("topic", QoS::ExactlyOnce).await.unwrap();

    task::spawn(async move {
        for i in 0..10 {
            reporter.publish("topic", QoS::ExactlyOnce, false, String::new()).await.unwrap();
            time::delay_for(Duration::from_millis(100)).await;
        }
    });

    loop {
        let notification = event_loop.poll().await.unwrap();
        println!("Received = {:?}", notification);
        tokio::time::delay_for(Duration::from_secs(1)).await;
    }
}