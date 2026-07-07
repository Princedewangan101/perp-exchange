use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));

    async fn get_foo() {}
    async fn post_foo() {}
    async fn foo_bar() {}

    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("> server running on port 5000");
    axum::serve(listener, app).await.unwrap();
}
