use tokio_postgres::Client;

use crate::query::common::log_db_error;

pub struct FindUserRequest {
    pub email: String,
}

#[derive(Debug)]
pub struct FindUserResponse {
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub balance: Option<i64>,
}

pub async fn find_user(postgres_client: &Client, req: FindUserRequest) -> FindUserResponse {
    let search_query_result = postgres_client
        .query_opt(
            "SELECT userId, email, balance::BIGINT FROM users WHERE email = $1",
            &[&req.email],
        )
        .await;
    match search_query_result {
        Ok(Some(row)) => {
            let user_id_found: i32 = row.get(0);
            let email_found: String = row.get(1);
            let balance_found: i64 = row.get(2);
            FindUserResponse {
                user_id: Some(user_id_found.to_string()),
                email: Some(email_found),
                balance: Some(balance_found),
            }
        }
        Ok(None) => FindUserResponse {
            user_id: None,
            email: None,
            balance: None,
        },
        Err(err) => {
            log_db_error("find_user", &err);
            FindUserResponse {
                user_id: None,
                email: None,
                balance: None,
            }
        }
    }
}
