use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn root() ->&'static str {
    "Welcome to FlickPick!"
}