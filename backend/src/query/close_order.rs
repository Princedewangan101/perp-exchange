use rust_decimal::Decimal;
use tokio_postgres::Client;
use uuid::Uuid;

use crate::query::common::log_db_error;

pub struct CloseOrderRequest {
    pub user_id: String,
    pub order_id: String,
    pub close_price: f64,
    pub close_type: String,
}

pub struct CloseOrderResponse {
    pub success: bool,
}

pub async fn close_order(postgres_client: &Client, req: CloseOrderRequest) -> CloseOrderResponse {
    let user_id: i32 = match req.user_id.parse() {
        Ok(id) => id,
        Err(_) => return CloseOrderResponse { success: false },
    };
    let order_id: Uuid = match Uuid::parse_str(&req.order_id) {
        Ok(id) => id,
        Err(_) => return CloseOrderResponse { success: false },
    };
    let pg_close_price = Decimal::from_f64_retain(req.close_price).unwrap();
    let close_query_result = postgres_client
        .query_one(
            "UPDATE orders SET close = $1, closeType = $2, status = 'completed' WHERE userId = $3 AND orderId = $4 RETURNING close::double precision",
            &[&pg_close_price, &req.close_type, &user_id, &order_id],
        )
        .await;
    match close_query_result {
        Ok(_row) => CloseOrderResponse { success: true },
        Err(err) => {
            log_db_error("close_order", &err);
            CloseOrderResponse { success: false }
        }
    }
}