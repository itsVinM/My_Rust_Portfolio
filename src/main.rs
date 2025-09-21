use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main(){
    let static_files_service = ServeDir::new("static")
        .fallback(
            tower_http::services::ServeFile::new("static/portfolio.html")
        );

    let app = Router::new()
        .fallback_service(static_files_service);

    //Run application with hyper, listining globally on port 3000
    let addr = SocketAddr::from(([0,0,0,0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}