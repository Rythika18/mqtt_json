use serde_json::Value;
use rumqttc::{MqttOptions, Client, QoS};
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    
    let file_content = fs::read_to_string("data_rust.json").expect("Unable to read file");
    let json_data: Value = serde_json::from_str(&file_content).expect("JSON was not well-formatted");

   
    let json_string = serde_json::to_string(&json_data).expect("Failed to convert JSON to string");

    
    let mqtt_options = MqttOptions::new("client_id", "test.mosquitto.org", 1883);
    let (client, mut connection) = Client::new(mqtt_options, 10);

    // Spawn a thread to handle connection events
    thread::spawn(move || {
        for event in connection.iter() {
            println!("{:?}", event);
        }
    });

    // Publish JSON data to MQTT topic every 5 seconds
    loop {
        client.publish("example/topic", QoS::AtLeastOnce, false, json_string.clone().into_bytes()).expect("Failed to publish message");
        println!("Published JSON data to MQTT topic");

        // Sleep for 5 seconds
        thread::sleep(Duration::from_secs(5));
    }
}
