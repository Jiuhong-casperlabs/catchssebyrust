use futures::stream::StreamExt;
use reqwest_eventsource::{Event, EventSource};
use serde_json::{Result, Value};

// extern crate serde_json;

#[tokio::main]
async fn main() -> Result<()> {
    let mut es = EventSource::get("http://3.208.91.63:9999/events/main");
    while let Some(event) = es.next().await {
        match event {
            Ok(Event::Message(message)) => {
                let data = message.data;
                // println!("data is {}", data);

                let v: Value = serde_json::from_str(&data)?;

                if v["DeployProcessed"]["execution_result"]["Success"] != Value::Null {
                    let deploy_hash = v["DeployProcessed"]["deploy_hash"].as_str().unwrap();

                    let transfromsarray = v["DeployProcessed"]["execution_result"]["Success"]
                        ["effect"]["transforms"]
                        .as_array()
                        .unwrap();

                    let mut my_events;
                    for obj in transfromsarray {
                        if obj["transform"]["WriteCLValue"]["parsed"].is_array() {
                            my_events = obj["transform"]["WriteCLValue"]["parsed"]
                                .as_array()
                                .unwrap();

                            let mut output = false;
                            for event in my_events.clone() {
                                // customize your event here V V V
                                if event["value"] == "account-hash-2293223427d59ebb331ac2221c3fcd1b3656a5cb72be924a6cdc9d52cdb6db0f" {
                                    output= true;
                                }
                            }
                            if output {
                                println!("deploy_hash {} ", deploy_hash);
                                for event in my_events {
                                    println!("{}", event);
                                }
                            }
                        }
                    }
                }
            }
            Ok(_) => {}
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
    Ok(())
}
