use openiap_client::{enable_tracing, Client, OpenIAPError, RegisterQueueRequest};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), OpenIAPError> {
    let b = Arc::new(Client::new());
    enable_tracing("openiap=trace", "new");
    let b_clone = Arc::clone(&b);
    b.on_event(Box::new(move |_event| {
        println!("CLI: Event received: {:?}", _event);
        let b = Arc::clone(&b_clone);
        match _event {
            openiap_client::ClientEvent::Connecting => println!("CLI: Client connecting!"),
            openiap_client::ClientEvent::Connected => println!("CLI: Client connected!"),
            openiap_client::ClientEvent::Disconnected(e) => println!("CLI: Client disconnected! {:?}", e),
            openiap_client::ClientEvent::SignedIn => {
                println!("CLI: Client signed in, we can now register queues in OpenIAP!");
                let b = Arc::clone(&b);
                tokio::spawn(async move {
                    match b.register_queue(
                        RegisterQueueRequest::byqueuename("testq"),
                        Box::new(|event| {
                            println!(
                                "Received message from queue {:?} with reply to {:?}: {:?}",
                                event.queuename, event.replyto, event.data
                            );
                        }),
                    )
                    .await {
                        Ok(_) => println!("CLI: Queue registered!"),
                        Err(e) => println!("CLI: Queue registration failed! {:?}", e),                        
                    };
                });
            },
        }
    })).await;
    println!("CLI: Connecting to OpenIAP...");
    b.connect_async("").await?;
    println!("CLI: Connected to OpenIAP!");
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
