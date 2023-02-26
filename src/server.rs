use std::net::SocketAddr;

use axum::{routing::post, Router};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = Router::new()
        .merge(handler::user::create_route())
        .merge(handler::book::create_route());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
