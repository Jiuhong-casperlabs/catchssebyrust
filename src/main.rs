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

                    let pk = v["DeployProcessed"]["account"].as_str().unwrap();
                    // Here is your customized public key
                    if pk == "010e31a03ea026a8e375653573e0120c8cb96699e6c9721ae1ea98f896e6576ac3" {
                        println!("deploy_hash is {}", deploy_hash);
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
