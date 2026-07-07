use axum::{Router, routing::post};

mod handlers;

use handlers::signup::signup;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/api/signup", post(signup));
        

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("> server running on port 5000");
    axum::serve(listener, app).await.unwrap();
}
