use futures::stream::StreamExt;
use reqwest_eventsource::{Event, EventSource};
use serde_json::Value;

extern crate serde;
extern crate serde_json;

use serde_json::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut es = EventSource::get("http://16.162.124.124:9999/events/main");
    while let Some(event) = es.next().await {
        match event {
            Ok(Event::Message(message)) => {
                let data = message.data;

                let v: Value = serde_json::from_str(&data)?;

                if v["DeployProcessed"]["execution_result"]["Success"] != Value::Null {
                    // customize your code here
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
                            for event in my_events {
                                // customize your event here VVV
                                if event["value"] == "account-hash-2293223427d59ebb331ac2221c3fcd1b3656a5cb72be924a6cdc9d52cdb6db0f" {
                                    output= true;
                                }
                            }
                            if output {
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
