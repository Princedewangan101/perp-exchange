use rust_decimal::Decimal;
use serde::Serialize;
use tokio_postgres::Client;

use crate::query::common::log_db_error;

pub struct UpdateBalanceRequest {
    pub order_id: String,
    pub user_id: String,
    pub close_price: f64,
}

#[derive(Serialize)]
pub struct UpdateBalanceResponse {
    pub success: bool,
    pub balance: Option<f64>,
}

pub async fn update_balance(postgres_client: &Client, req: UpdateBalanceRequest) -> UpdateBalanceResponse {
    let order_id: i32 = match req.order_id.parse() {
        Ok(id) => id,
        Err(_) => {
            return UpdateBalanceResponse {
                success: false,
                balance: None,
            };
        }
    };
    let user_id: i32 = match req.user_id.parse() {
        Ok(id) => id,
        Err(_) => {
            return UpdateBalanceResponse {
                success: false,
                balance: None,
            };
        }
    };
    let order_result = postgres_client
        .query_opt(
            "SELECT open::double precision, side, leverage FROM orders WHERE orderId = $1 AND userId = $2",
            &[&order_id, &user_id],
        )
        .await;
    match order_result {
        Ok(Some(row)) => {
            let open_price_found: f64 = row.get(0);
            let side_found: u32 = row.get::<_, i16>(1) as u32;
            let leverage_found: u32 = row.get::<_, i16>(2) as u32;
            let is_profit: bool;
            let sum: f64;
            if side_found == 0 {
                let diff = open_price_found - req.close_price;
                if diff > 0.0 {
                    is_profit = true
                } else {
                    is_profit = false
                }
                sum = diff.abs() * leverage_found as f64
            } else {
                let diff = req.close_price - open_price_found;
                if diff > 0.0 {
                    is_profit = true
                } else {
                    is_profit = false
                }
                sum = diff.abs() * leverage_found as f64
            }
            let pg_sum = Decimal::from_f64_retain(sum).unwrap_or_default();
            if is_profit {
                let balance_update_query_response = postgres_client
                    .query_one(
                        "UPDATE users SET balance = balance + $1 WHERE userId = $2 RETURNING balance::double precision",
                        &[&pg_sum, &user_id],
                    )
                    .await;
                match balance_update_query_response {
                    Ok(row) => {
                        return UpdateBalanceResponse {
                            success: true,
                            balance: Some(row.get::<_, f64>(0)),
                        };
                    }
                    Err(err) => {
                        log_db_error("update_balance (profit)", &err);
                        return UpdateBalanceResponse {
                            success: false,
                            balance: None,
                        };
                    }
                }
            } else {
                let balance_update_query_response = postgres_client
                    .query_one(
                        "UPDATE users SET balance = balance - $1 WHERE userId = $2 AND balance >= $3 RETURNING balance::double precision",
                        &[&pg_sum, &user_id, &pg_sum],
                    )
                    .await;
                match balance_update_query_response {
                    Ok(row) => {
                        return UpdateBalanceResponse {
                            success: true,
                            balance: Some(row.get::<_, f64>(0)),
                        };
                    }
                    Err(err) => {
                        log_db_error("update_balance (loss)", &err);
                        return UpdateBalanceResponse {
                            success: false,
                            balance: None,
                        };
                    }
                }
            }
        }
        Ok(None) => {
            return UpdateBalanceResponse {
                success: false,
                balance: None,
            };
        }
        Err(err) => {
            log_db_error("update_balance (fetch order)", &err);
            return UpdateBalanceResponse {
                success: false,
                balance: None,
            };
        }
    }
}
