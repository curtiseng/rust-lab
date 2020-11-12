use rumqttc::{MqttOptions, AsyncClient, QoS};
use tokio::{task, time};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("mqtt-async", "127.0.0.1", 1883);
    mqttoptions.set_keep_alive(5);
    mqttoptions.set_clean_session(false);

    let (client, mut event_loop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("world", QoS::ExactlyOnce).await.unwrap();

    task::spawn(async move {
        for i in 0..10 {
            client.publish("hello", QoS::AtLeastOnce, false, vec![i; i as usize]).await.unwrap();
            time::delay_for(Duration::from_millis(100)).await;
        }
    });

    loop {
        let notification = event_loop.poll().await.unwrap();
        println!("Received = {:?}", notification);
        tokio::time::delay_for(Duration::from_secs(1)).await;
    }
}