#![allow(unused)]

use std::net::SocketAddr;

use axum::{
    Router, 
    response::{Html, IntoResponse},
    routing::get, extract::{Query, Path},
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_all = Router::new().merge(routes_hello());

    // region: --- Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap()
    // endregion: --- Start Server
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g., `/hello?name=John`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello - {params:#?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g., `/hello2/Mark`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello2 - {name:#?}", "HANDLER");

    Html(format!("Hello2 <strong>{name}</strong>"))

}
