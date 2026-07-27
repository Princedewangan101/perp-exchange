use serde::Serialize;
use tokio_postgres::Client;

use crate::query::common::log_db_error;

pub struct FetchOrdersRequest {
    pub user_id: String,
}

#[derive(Serialize)]
pub struct Order {
    pub order_id: String,
    pub symbol: String,
    pub quantity: f64,
    pub side: u32,
    pub order_type: String,
    pub status: String,
    pub tp: Option<f64>,
    pub sl: Option<f64>,
    pub open: f64,
    pub close: Option<f64>,
    pub close_type: Option<String>,
    pub pnl: Option<f64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize)]
pub struct FetchOrdersResponse {
    pub success: bool,
    pub message: String,
    pub orders: Option<Vec<Order>>,
}

pub async fn fetch_orders_from_db(postgres_client: &Client, req: FetchOrdersRequest) -> FetchOrdersResponse {
    let user_id: i32 = match req.user_id.parse() {
        Ok(id) => id,
        Err(_) => {
            return FetchOrdersResponse {
                success: false,
                message: "Invalid user ID".to_string(),
                orders: None,
            };
        }
    };
    let result = postgres_client
        .query(
            "SELECT orderId::text, symbol, quantity::double precision, side::int2, type, status, tp::double precision, sl::double precision, open::double precision, close::double precision, closeType, pnl::double precision, created_at::text, updated_at::text FROM orders WHERE userId = $1",
            &[&user_id],
        )
        .await;
    let rows = match result {
        Ok(v) => v,
        Err(err) => {
            log_db_error("fetch_orders_from_db", &err);
            return FetchOrdersResponse {
                success: false,
                message: format!("Database error: {}", err),
                orders: None,
            };
        }
    };
    let orders_list = rows
        .iter()
        .map(|row| Order {
            order_id: row.get(0),
            symbol: row.get(1),
            quantity: row.get(2),
            side: row.get::<_, i16>(3) as u32,
            order_type: row.get(4),
            status: row.get(5),
            tp: row.get(6),
            sl: row.get(7),
            open: row.get(8),
            close: row.get(9),
            close_type: row.get(10),
            pnl: row.get(11),
            created_at: row.get(12),
            updated_at: row.get(13),
        })
        .collect();
    return FetchOrdersResponse {
        success: true,
        message: "Orders fetched successfully".to_string(),
        orders: Some(orders_list),
    };
}