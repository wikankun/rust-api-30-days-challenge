use axum::{
    routing::get,
    extract::{Path},
    Router,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/hello/{name}", get(get_name));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_name(Path(name): Path<String>) -> String {
    name
}
