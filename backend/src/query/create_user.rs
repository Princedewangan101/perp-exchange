use tokio_postgres::Client;

use crate::query::common::log_db_error;

pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

pub struct CreateUserResponse {
    pub success: bool,
    pub id: String,
}

pub async fn create_user(postgres_client: &Client, req: CreateUserRequest) -> CreateUserResponse {
    let insert_query_result = postgres_client
        .query_one(
            "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING userId",
            &[&req.email, &req.password],
        )
        .await;
    match insert_query_result {
        Ok(row) => {
            let id: i32 = row.get(0);
            println!("\n> Created user with ID: {}", id);
            CreateUserResponse {
                success: true,
                id: id.to_string(),
            }
        }
        Err(err) => {
            log_db_error("create_user", &err);
            CreateUserResponse {
                success: false,
                id: "".to_string(),
            }
        }
    }
}
