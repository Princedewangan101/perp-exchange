use rust_decimal::Decimal;
use tokio_postgres::Client;

use crate::query::common::log_db_error;

pub struct ModifyOrderRequest {
    pub user_id: String,
    pub order_id: String,
    pub tp: f64,
    pub sl: f64,
}

pub struct ModifyOrderResponse {
    pub success: bool,
    pub tp: Option<f64>,
    pub sl: Option<f64>,
}

pub async fn modify_order(postgres_client: &Client, req: ModifyOrderRequest) -> ModifyOrderResponse {
    let user_id: i32 = match req.user_id.parse() {
        Ok(id) => id,
        Err(_) => return ModifyOrderResponse { success: false, tp: None, sl: None },
    };
    let order_id: i32 = match req.order_id.parse() {
        Ok(id) => id,
        Err(_) => return ModifyOrderResponse { success: false, tp: None, sl: None },
    };
    let pg_tp = Decimal::from_f64_retain(req.tp).unwrap();
    let pg_sl = Decimal::from_f64_retain(req.sl).unwrap();
    let modify_query_response;
    if req.tp != 0.0 && req.sl != 0.0 {
        modify_query_response = postgres_client
            .query_one(
                "UPDATE orders SET tp = $1, sl = $2 WHERE userId = $3 AND orderId = $4 RETURNING tp::double precision, sl::double precision",
                &[&pg_tp, &pg_sl, &user_id, &order_id],
            )
            .await;
        match modify_query_response {
            Ok(row) => {
                let updated_tp: f64 = row.get(0);
                let updated_sl: f64 = row.get(1);
                return ModifyOrderResponse {
                    success: true,
                    tp: Some(updated_tp),
                    sl: Some(updated_sl),
                };
            }
            Err(err) => {
                log_db_error("modify_order (both)", &err);
                return ModifyOrderResponse { success: false, tp: None, sl: None };
            }
        }
    } else if req.tp != 0.0 {
        modify_query_response = postgres_client
            .query_one(
                "UPDATE orders SET tp = $1 WHERE userId = $2 AND orderId = $3 RETURNING tp::double precision, sl::double precision",
                &[&pg_tp, &user_id, &order_id],
            )
            .await;
        match modify_query_response {
            Ok(row) => {
                let updated_tp: f64 = row.get(0);
                let updated_sl: f64 = row.get(1);
                return ModifyOrderResponse {
                    success: true,
                    tp: Some(updated_tp),
                    sl: Some(updated_sl),
                };
            }
            Err(err) => {
                log_db_error("modify_order (tp only)", &err);
                return ModifyOrderResponse { success: false, tp: None, sl: None };
            }
        }
    } else if req.sl != 0.0 {
        modify_query_response = postgres_client
            .query_one(
                "UPDATE orders SET sl = $1 WHERE userId = $2 AND orderId = $3 RETURNING tp::double precision, sl::double precision",
                &[&pg_sl, &user_id, &order_id],
            )
            .await;
        match modify_query_response {
            Ok(row) => {
                let updated_tp: f64 = row.get(0);
                let updated_sl: f64 = row.get(1);
                return ModifyOrderResponse {
                    success: true,
                    tp: Some(updated_tp),
                    sl: Some(updated_sl),
                };
            }
            Err(err) => {
                log_db_error("modify_order (sl only)", &err);
                return ModifyOrderResponse { success: false, tp: None, sl: None };
            }
        }
    } else {
        return ModifyOrderResponse { success: false, tp: None, sl: None };
    }
}
