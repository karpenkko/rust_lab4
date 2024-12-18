use warp::Filter;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use crate::handlers::handle_connection;

mod handlers;

type Users = Arc<Mutex<HashMap<String, String>>>; 

#[tokio::main]
async fn main() {
    let users: Users = Arc::new(Mutex::new(HashMap::new()));
    let tx = Arc::new(Mutex::new(broadcast::channel(100).0));

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_users(users.clone()))
        .and(with_broadcast(tx.clone()))
        .map(|ws: warp::ws::Ws, users, tx| {
            ws.on_upgrade(move |websocket| handle_connection(websocket, users, tx))
        });

    warp::serve(ws_route)
        .run(([127, 0, 0, 1], 8080))
        .await;
}

fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || users.clone())
}

fn with_broadcast(
    tx: Arc<Mutex<broadcast::Sender<String>>>,
) -> impl Filter<Extract = (Arc<Mutex<broadcast::Sender<String>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || tx.clone())
}
