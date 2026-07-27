use rust_decimal::Decimal;
use tokio_postgres::Client;
use uuid::Uuid;

use crate::query::common::log_db_error;

pub struct LimitOrderRequest {
    pub order_id: String,
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
}

pub async fn limit_order(postgres_client: &Client, req: LimitOrderRequest) -> LimitOrderResponse {
    let pg_side = req.side as i16;
    let pg_leverage = req.leverage as i16;
    let user_id: i32 = match req.user_id.parse() {
        Ok(id) => id,
        Err(_) => return LimitOrderResponse { success: false },
    };
    let order_id: Uuid = match Uuid::parse_str(&req.order_id) {
        Ok(id) => id,
        Err(_) => return LimitOrderResponse { success: false },
    };
    let pg_quantity = Decimal::from_f64_retain(req.quantity).unwrap();
    let pg_tp = Decimal::from_f64_retain(req.tp).unwrap();
    let pg_sl = Decimal::from_f64_retain(req.sl).unwrap();
    let pg_open = Decimal::from_f64_retain(req.open).unwrap();
    let result;
    if req.tp == 0.0 && req.sl == 0.0 {
        result = postgres_client
            .execute(
                "INSERT INTO orders (orderId, userId, symbol, quantity, side, type, status, leverage, open) \
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
                &[
                    &order_id,
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
            .execute(
                "INSERT INTO orders (orderId, userId, symbol, quantity, side, type, status, leverage, sl, open) \
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                &[
                    &order_id,
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
            .execute(
                "INSERT INTO orders (orderId, userId, symbol, quantity, side, type, status, leverage, tp, open) \
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                &[
                    &order_id,
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
            .execute(
                "INSERT INTO orders (orderId, userId, symbol, quantity, side, type, status, leverage, tp, sl, open) \
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
                &[
                    &order_id,
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
        Ok(_) => {
            println!("\n> [LIMIT_ORDER_DB]: order_id:{}, symbol:{}, quantity:{}, side:{}, type:{}, leverage:{}, tp:{}, sl:{}, open:{}",
                req.order_id, req.symbol, req.quantity, req.side, req.order_type, req.leverage, req.tp, req.sl, req.open);
            LimitOrderResponse { success: true }
        }
        Err(err) => {
            log_db_error("limit_order", &err);
            LimitOrderResponse { success: false }
        }
    }
}
