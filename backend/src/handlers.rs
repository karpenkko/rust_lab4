use warp::ws::{Message, WebSocket};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use futures_util::{StreamExt, SinkExt};

type Users = Arc<Mutex<HashMap<String, String>>>;

pub async fn handle_connection(
    ws: WebSocket, 
    users: Users, 
    tx: Arc<Mutex<broadcast::Sender<String>>>,
) {
    let (mut ws_sender, mut ws_receiver) = ws.split();
    let addr = format!("{:?}", ws_sender);

    let mut rx = tx.lock().unwrap().subscribe();
    
    // Broadcast receiver task
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if ws_sender.send(Message::text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Username handling
    let username = if let Some(Ok(msg)) = ws_receiver.next().await {
        if let Ok(username) = msg.to_str() {
            let username = username.to_string();
        
            users.lock().unwrap().insert(addr.clone(), username.clone());
    
            let join_message = serde_json::json!({
                "type": "join",
                "sender": username
            }).to_string();
    
            tx.lock().unwrap().send(join_message).ok();
            
            username
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    };

    // Message handling
    while let Some(Ok(message)) = ws_receiver.next().await {
        if message.is_text() {
            if let Ok(text) = message.to_str() {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(text) {
                    if let Some(t) = json["type"].as_str() {
                        if t == "file" {
                            let broadcast_message = serde_json::json!({
                                "type": "file",
                                "sender": username,
                                "name": json["name"].as_str().unwrap_or("unknown"),
                                "content": json["content"].as_str().unwrap_or("")
                            }).to_string();

                            tx.lock().unwrap().send(broadcast_message).ok();
                        } else if t == "message" {
                            let broadcast_message = serde_json::json!({
                                "type": "message",
                                "sender": username,
                                "content": json["content"].as_str().unwrap_or("")
                            }).to_string();

                            tx.lock().unwrap().send(broadcast_message).ok();
                        }
                    }
                }
            }
        }
    }

    // Remove user when connection closes
    users.lock().unwrap().remove(&addr);
}