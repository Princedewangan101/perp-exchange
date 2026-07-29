use tokio_postgres::Client;

use crate::query::common::log_db_error;

pub struct FetchBalanceRequest {
    pub user_id: String,
}

pub struct FetchBalanceResponse {
    pub success: bool,
    pub balance: Option<f64>,
}

pub async fn fetch_balance(postgres_client: &Client, req: FetchBalanceRequest) -> FetchBalanceResponse {
    let user_id: i32 = match req.user_id.parse() {
        Ok(id) => id,
        Err(_) => {
            // println!("\n>[FETCH_BALANCE] invalid user_id: {}", req.user_id);
            return FetchBalanceResponse {
                success: false,
                balance: None,
            };
        }
    };

    // println!("\n>[FETCH_BALANCE] querying balance for user_id: {}", user_id);

    let result = postgres_client
        .query_opt(
            "SELECT balance::double precision FROM users WHERE userId = $1",
            &[&user_id],
        )
        .await;

    match result {
        Ok(Some(row)) => {
            let balance: f64 = row.get(0);
            // println!("\n>[FETCH_BALANCE] balance found: {}", balance);
            FetchBalanceResponse {
                success: true,
                balance: Some(balance),
            }
        }
        Ok(None) => {
            // println!("\n>[FETCH_BALANCE] no user found with user_id: {}", user_id);
            FetchBalanceResponse {
                success: false,
                balance: None,
            }
        }
        Err(err) => {
            log_db_error("fetch_balance", &err);
            FetchBalanceResponse {
                success: false,
                balance: None,
            }
        }
    }
}
