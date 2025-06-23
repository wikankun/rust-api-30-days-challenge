use axum::{
    Router,
    extract::{Json, Path},
    response::Json as respJson,
    routing::{delete, get, post, put},
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Mutex;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new()
        .route("/item", get(get_item))
        .route("/item", post(post_item))
        .route("/item/{name}", put(put_item))
        .route("/item/{name}", delete(delete_item));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    name: String,
    stock: u32,
}

static ITEMS: Lazy<Mutex<Vec<Item>>> = Lazy::new(|| Mutex::new(vec![]));

async fn get_item() -> respJson<Value> {
    let items = ITEMS.lock().unwrap();
    respJson(json!({"items": *items}))
}

async fn post_item(Json(payload): Json<Item>) -> respJson<Value> {
    let mut items = ITEMS.lock().unwrap();
    items.push(payload.clone());
    respJson(json!({"item": payload}))
}

async fn put_item(Path(name): Path<String>, Json(payload): Json<Item>) -> respJson<Value> {
    let mut items = ITEMS.lock().unwrap();

    if let Some(item) = items.iter_mut().find(|item| item.name == name) {
        item.stock = payload.stock;

        return respJson(json!({"item": payload}));
    }

    respJson(json!({"error": "Item not found"}))
}

async fn delete_item(Path(name): Path<String>) -> respJson<Value> {
    let mut items = ITEMS.lock().unwrap();
    let len_before = items.len();
    items.retain(|item| item.name != name);

    if items.len() < len_before {
        respJson(json!({"message": "Item deleted"}))
    } else {
        respJson(json!({"message": "Item not found"}))
    }
}
