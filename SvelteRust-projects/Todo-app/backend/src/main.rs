use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));
    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Cannot bind address");
    println!("{:<12} - Server listening", "http://localhost:8000");
    axum::serve(tcp_listener, app)
        .await
        .expect("Cannot serve app");
}

async fn index() -> String {
    format!("Hello, world!")
}
