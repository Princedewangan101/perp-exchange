use rust_decimal::Decimal;
use tokio_postgres::Client;

use crate::query::common::log_db_error;

pub struct DepositBalanceRequest {
    pub user_id: String,
    pub amount: f64,
}

pub struct DepositBalanceResponse {
    pub success: bool,
    pub balance: Option<f64>,
}

pub async fn deposit_balance(postgres_client: &Client, req: DepositBalanceRequest) -> DepositBalanceResponse {
    println!("\n>[INFO] deposit route , TRIGGERED\n amount: {}", req.amount);
    let user_id: i32 = match req.user_id.parse() {
        Ok(id) => id,
        Err(_) => {
            return DepositBalanceResponse {
                success: false,
                balance: None,
            };
        }
    };
    let pg_amount = Decimal::from_f64_retain(req.amount).unwrap();
    let post_query_result = postgres_client
        .query_one(
            "UPDATE users SET balance = balance + $1 WHERE userId = $2 RETURNING balance::double precision",
            &[&pg_amount, &user_id],
        )
        .await;
    match post_query_result {
        Ok(row) => {
            println!("\n>[INFO] deposit route , SUCCESS");
            let balance = row.get::<_, f64>(0);
            DepositBalanceResponse {
                success: true,
                balance: Some(balance),
            }
        }
        Err(err) => {
            log_db_error("deposit_balance", &err);
            DepositBalanceResponse {
                success: false,
                balance: None,
            }
        }
    }
}
