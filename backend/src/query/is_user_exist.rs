use tokio_postgres::Client;

use crate::query::common::log_db_error;

pub struct IsUserExistRequest {
    pub email: String,
}

pub struct IsUserExistResponse {
    pub is_user_exist: bool,
    pub email: Option<String>,
}

pub async fn is_user_exist(postgres_client: &Client, req: IsUserExistRequest) -> IsUserExistResponse {
    let search_query_result = postgres_client
        .query_opt(
            "SELECT userId, email FROM users WHERE email = $1 ",
            &[&req.email],
        )
        .await;
    match search_query_result {
        Ok(Some(row)) => {
            println!("\n> user created");
            let email_found: String = row.get(1);
            IsUserExistResponse {
                is_user_exist: true,
                email: Some(email_found),
            }
        }
        Ok(None) => IsUserExistResponse {
            is_user_exist: false,
            email: None,
        },
        Err(err) => {
            log_db_error("is_user_exist", &err);
            IsUserExistResponse {
                is_user_exist: false,
                email: None,
            }
        }
    }
}
