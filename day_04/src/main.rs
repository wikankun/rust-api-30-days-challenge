use axum::{
    extract::Json,
    routing::post,
    Router,
};
use serde::{Deserialize};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/echo", post(get_name));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct User {
    name: String,
    email: String,
}

async fn get_name(Json(payload): Json<User>) -> String {
    payload.name
}
