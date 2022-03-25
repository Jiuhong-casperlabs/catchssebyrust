use futures::stream::StreamExt;
use reqwest_eventsource::{Event, EventSource};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut es = EventSource::get("http://16.162.124.124:9999/events/main");
    while let Some(event) = es.next().await {
        match event {
            Ok(Event::Open) => println!("Connection Open!"),
            Ok(Event::Message(message)) => println!("Message: {:#?}", message),
            Err(err) => {
                println!("Error: {}", err);
                // es.close();
            }
        }
    }
    Ok(())
}
