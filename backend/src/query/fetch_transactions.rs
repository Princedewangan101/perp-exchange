use serde::Serialize;
use tokio_postgres::Client;

use crate::query::common::log_db_error;

pub struct FetchTransactionsRequest {
    pub user_id: String,
}

#[derive(Serialize)]
pub struct Transaction {
    pub transaction_id: i32,
    pub balance: f64,
    pub transaction_type: String,
    pub created_at: Option<String>,
}

#[derive(Serialize)]
pub struct FetchTransactionsResponse {
    pub success: bool,
    pub message: String,
    pub transactions: Option<Vec<Transaction>>,
}

pub async fn fetch_transactions_from_db(
    postgres_client: &Client,
    req: FetchTransactionsRequest,
) -> FetchTransactionsResponse {
    let user_id: i32 = match req.user_id.parse() {
        Ok(id) => id,
        Err(_) => {
            return FetchTransactionsResponse {
                success: false,
                message: "Invalid user ID".to_string(),
                transactions: None,
            };
        }
    };
    let result = postgres_client
        .query(
            "SELECT transactionid, balance::double precision, type, created_at::text FROM transactions WHERE userid = $1",
            &[&user_id],
        )
        .await;
    let rows = match result {
        Ok(v) => v,
        Err(err) => {
            log_db_error("fetch_transactions_from_db", &err);
            return FetchTransactionsResponse {
                success: false,
                message: format!("Database error: {}", err),
                transactions: None,
            };
        }
    };
    let transactions_list = rows
        .iter()
        .map(|row| Transaction {
            transaction_id: row.get(0),
            balance: row.get(1),
            transaction_type: row.get(2),
            created_at: row.get(3),
        })
        .collect();
    return FetchTransactionsResponse {
        success: true,
        message: "Orders fetched successfully".to_string(),
        transactions: Some(transactions_list),
    };
}
