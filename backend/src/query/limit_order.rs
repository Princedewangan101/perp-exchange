use rust_decimal::Decimal;
use tokio_postgres::Client;

use crate::query::common::log_db_error;

pub struct LimitOrderRequest {
    pub user_id: String,
    pub symbol: String,
    pub quantity: f64,
    pub side: u32,
    pub order_type: String,
    pub status: String,
    pub leverage: u32,
    pub tp: f64,
    pub sl: f64,
    pub open: f64,
}

pub struct LimitOrderResponse {
    pub success: bool,
    pub order_id: Option<String>,
}

pub async fn limit_order(postgres_client: &Client, req: LimitOrderRequest) -> LimitOrderResponse {
    let pg_side = req.side as i16;
    let pg_leverage = req.leverage as i16;
    let user_id: i32 = match req.user_id.parse() {
        Ok(id) => id,
        Err(_) => return LimitOrderResponse { success: false, order_id: None },
    };
    let pg_quantity = Decimal::from_f64_retain(req.quantity).unwrap();
    let pg_tp = Decimal::from_f64_retain(req.tp).unwrap();
    let pg_sl = Decimal::from_f64_retain(req.sl).unwrap();
    let pg_open = Decimal::from_f64_retain(req.open).unwrap();
    let result;
    if req.tp == 0.0 && req.sl == 0.0 {
        result = postgres_client
            .query_one(
                "INSERT INTO orders (userId, symbol, quantity, side, type, status, leverage, open) \
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8) \
              RETURNING orderId",
                &[
                    &user_id,
                    &req.symbol,
                    &pg_quantity,
                    &pg_side,
                    &req.order_type,
                    &req.status,
                    &pg_leverage,
                    &pg_open,
                ],
            )
            .await;
    } else if req.tp == 0.0 {
        result = postgres_client
            .query_one(
                "INSERT INTO orders (userId, symbol, quantity, side, type, status, leverage, sl, open) \
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
              RETURNING orderId",
                &[
                    &user_id,
                    &req.symbol,
                    &pg_quantity,
                    &pg_side,
                    &req.order_type,
                    &req.status,
                    &pg_leverage,
                    &pg_sl,
                    &pg_open,
                ],
            )
            .await;
    } else if req.sl == 0.0 {
        result = postgres_client
            .query_one(
                "INSERT INTO orders (userId, symbol, quantity, side, type, status, leverage, tp, open) \
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) \
              RETURNING orderId",
                &[
                    &user_id,
                    &req.symbol,
                    &pg_quantity,
                    &pg_side,
                    &req.order_type,
                    &req.status,
                    &pg_leverage,
                    &pg_tp,
                    &pg_open,
                ],
            )
            .await;
    } else {
        result = postgres_client
            .query_one(
                "INSERT INTO orders (userId, symbol, quantity, side, type, status, leverage, tp, sl, open) \
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) \
              RETURNING orderId",
                &[
                    &user_id,
                    &req.symbol,
                    &pg_quantity,
                    &pg_side,
                    &req.order_type,
                    &req.status,
                    &pg_leverage,
                    &pg_tp,
                    &pg_sl,
                    &pg_open,
                ],
            )
            .await;
    }
    match result {
        Ok(row) => {
            let order_id: i32 = row.get("orderId");
            println!("\n> [LIMIT_ORDER_DB]: order_id:{order_id}, symbol:{}, quantity:{}, side:{}, type:{}, leverage:{}, tp:{}, sl:{}, open:{}",
                req.symbol, req.quantity, req.side, req.order_type, req.leverage, req.tp, req.sl, req.open);
            LimitOrderResponse {
                success: true,
                order_id: Some(order_id.to_string()),
            }
        }
        Err(err) => {
            log_db_error("limit_order", &err);
            LimitOrderResponse { success: false, order_id: None }
        }
    }
}
