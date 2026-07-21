use rust_decimal::Decimal;
use tokio_postgres::Client;

use crate::query::common::log_db_error;

pub struct WithdrawBalanceRequest {
    pub user_id: String,
    pub amount: f64,
}

pub struct WithdrawBalanceResponse {
    pub success: bool,
}

pub async fn withdraw_balance(postgres_client: &Client, req: WithdrawBalanceRequest) -> WithdrawBalanceResponse {
    let user_id: i32 = match req.user_id.parse() {
        Ok(id) => id,
        Err(_) => return WithdrawBalanceResponse { success: false },
    };
    let pg_amount = Decimal::from_f64_retain(req.amount).unwrap();
    let withdraw_query_result = postgres_client
        .query_one(
            "UPDATE users SET balance = balance - $1 WHERE userId = $2 AND balance >= $3 RETURNING balance::double precision",
            &[&pg_amount, &user_id, &pg_amount],
        )
        .await;
    match withdraw_query_result {
        Ok(_) => WithdrawBalanceResponse { success: true },
        Err(err) => {
            log_db_error("withdraw_balance", &err);
            WithdrawBalanceResponse { success: false }
        }
    }
}
