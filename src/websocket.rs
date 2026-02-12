use axum::extract::ws::{Message, WebSocket};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::models::{MessageType, WebSocketMessage};

type Connections = Arc<RwLock<HashMap<Uuid, broadcast::Sender<String>>>>;

static CONNECTIONS: once_cell::sync::Lazy<Connections> = 
    once_cell::sync::Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

pub async fn handle_socket(socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    let user_id = Uuid::new_v4();
    let (tx, mut rx) = broadcast::channel(100);
    
    // Store connection
    CONNECTIONS.write().await.insert(user_id, tx.clone());
    
    // Send welcome message
    let welcome_msg = WebSocketMessage {
        message_type: MessageType::UserJoined,
        review_id: Uuid::new_v4(), // This would be from context in real app
        data: json!({ "user_id": user_id }),
    };
    
    if let Ok(msg) = serde_json::to_string(&welcome_msg) {
        let _ = sender.send(Message::Text(msg)).await;
    }
    
    // Handle incoming messages
    let tx_clone = tx.clone();
    let incoming_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if let Ok(Message::Text(text)) = msg {
                if let Ok(ws_msg) = serde_json::from_str::<WebSocketMessage>(&text) {
                    // Broadcast to all connected clients
                    let _ = tx_clone.send(serde_json::to_string(&ws_msg).unwrap_or_default());
                }
            }
        }
    });
    
    // Handle outgoing messages
    let outgoing_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
    
    // Wait for either task to complete
    tokio::select! {
        _ = incoming_task => {},
        _ = outgoing_task => {},
    }
    
    // Clean up connection
    CONNECTIONS.write().await.remove(&user_id);
}