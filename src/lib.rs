

pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/messenger.rs"));  // Use the correct proto file for MessengerService
}
use tonic::transport::{Channel, Endpoint};
use dotenvy::from_path;
use crate::generated::messenger_service_client::MessengerServiceClient;
use crate::generated::MessageRequest;
use tonic::Request;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::env;
use std::path::Path;
use tracing_subscriber;

// Define your connect function
pub async fn connect_to_messenger_service() -> Option<MessengerServiceClient<Channel>> {
    // Initialize tracing or logging
    tracing_subscriber::fmt::init();
    // Load the environment variables from a custom file
    let custom_env_path = Path::new("proto-definitions/.service");
    from_path(custom_env_path).expect("Failed to load environment variables from custom path");

    let messenger_service_addr = env::var("MESSENGER_ADDR").ok()?;

    let messenger_service_endpoint = Endpoint::from_shared(messenger_service_addr.to_string())
        .expect("Invalid messenger service address")
        .keep_alive_while_idle(true)
        .keep_alive_timeout(std::time::Duration::from_secs(200000))
        .timeout(std::time::Duration::from_secs(60));

    match messenger_service_endpoint.connect().await {
        Ok(channel) => {
            println!("Connected to MessengerService.");
            Some(MessengerServiceClient::new(channel))
        },
        Err(e) => {
            println!("Failed to connect to MessengerService: {:?}", e);
            None
        }
    }
}

// Define your MessagingService struct and methods
pub struct MessagingService {
    pub client: Arc<Mutex<MessengerServiceClient<Channel>>>,
    pub tag: String,
}

impl MessagingService {
    pub fn new(client: Arc<Mutex<MessengerServiceClient<Channel>>>, tag: String) -> Self {
        MessagingService { client, tag }
    }

    pub async fn publish_message(&self, message: String, tags: Option<Vec<String>>) 
    -> Result<(), Box<dyn std::error::Error>> {
        let message_request = MessageRequest {
            message_text: message,
            tags: tags.unwrap_or_else(|| vec![self.tag.clone()]),
            gps_coordinates: None,
        };

        let mut client = self.client.lock().await;
        if let Err(e) = client.publish_message(Request::new(message_request)).await {
            println!("Failed to publish message: {:?}", e);
            return Err(Box::new(e));
        }
        Ok(())
    }
}

