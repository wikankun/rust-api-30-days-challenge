use axum::{
    extract::Query,
    routing::get,
    Router,
};
use serde::{Deserialize};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/sum", get(get_sum));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Numbers {
    a: i32,
    b: i32,
}

async fn get_sum(Query(params): Query<Numbers>) -> String {
    let result = params.a + params.b;
    result.to_string()
}
