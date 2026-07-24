use std::collections::HashMap;
use std::sync::Arc;
use futures::StreamExt;
use prost::Message;
use rust_decimal::Decimal;
use serde::Deserialize;
use tokio::sync::Mutex;

mod order_book;
mod proto;
use order_book::Market;

// ---- JSON PAYLOADS FOR order.* SUBJECTS ----
#[derive(Deserialize)]
struct LimitPayload {
    order_id: String,
    user_id: String,
    symbol: String,
    quantity: f64,
    side: u32,
    order_type: String,
    leverage: u32,
    price: f64,
    tp: Option<f64>,
    sl: Option<f64>,
}

#[derive(Deserialize)]
struct MarketPayload {
    order_id: String,
    user_id: String,
    symbol: String,
    quantity: f64,
    side: f64,
    tp: Option<f64>,
    sl: Option<f64>,
}

#[derive(Deserialize)]
struct ModifyPayload {
    symbol: String,
    side: u32,
    order_id: String,
    tp: Option<f64>,
    sl: Option<f64>,
}

#[derive(Deserialize)]
struct ClosePayload {
    symbol: String,
    side: u32,
    quantity: f64,
    order_id: String,
    user_id: String,
}

#[derive(Deserialize)]
struct CloseAllPayload {
    user_id: String,
}

#[tokio::main]
async fn main() {
    engine().await;
}

async fn engine() {
    let nats_client = async_nats::connect("127.0.0.1:4222").await.unwrap();
    let markets: Arc<Mutex<HashMap<String, Market>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut subscriber = nats_client.subscribe("order.*").await.unwrap();
    let mut market_sub = nats_client.subscribe("MARKET_ORDER").await.unwrap();

    // HANDLE MARKET_ORDER (PROTOBUF, USED BY BACKEND) IN A SEPARATE TASK
    let nats = nats_client.clone();
    let mkt_markets = markets.clone();
    tokio::spawn(async move {
        let mut sub = market_sub;
        while let Some(msg) = sub.next().await {
            let req = match proto::OrderRequest::decode(msg.payload) {
                Ok(r) => r,
                Err(_) => continue,
            };
            let payload = order_book::MarketPayload {
                user_id: req.user_id,
                order_id: String::new(),
                quantity: Decimal::from_f64_retain(req.quantity).unwrap_or(Decimal::ZERO),
                tp: Decimal::ZERO,
                sl: Decimal::ZERO,
                side: req.side as f64,
            };
            let mut markets = mkt_markets.lock().await;
            let market = markets
                .entry(req.symbol.clone())
                .or_insert_with(|| Market::new(req.symbol.clone()));
            let resp = market.market(payload);
            let reply = proto::OrderResponse {
                message: resp.message,
                quantity: 0.0,
            };
            let mut buf = Vec::new();
            if reply.encode(&mut buf).is_ok() {
                if let Some(reply_subject) = msg.reply {
                    let _ = nats.publish(reply_subject, buf.into()).await;
                }
            }
        }
    });

    // HANDLE order.* SUBJECTS (JSON)
    while let Some(message) = subscriber.next().await {
        let subject = message.subject.as_str();
        let payload_str = match std::str::from_utf8(&message.payload) {
            Ok(s) => s,
            Err(_) => continue,
        };

        match subject {
            "order.limit" => {
                let json: LimitPayload = match serde_json::from_str(payload_str) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let mut markets = markets.lock().await;
                let market = markets
                    .entry(json.symbol.clone())
                    .or_insert_with(|| Market::new(json.symbol.clone()));
                let payload = order_book::LimitOrderEventPayload {
                    user_id: json.user_id,
                    order_id: json.order_id,
                    side: json.side,
                    quantity: json.quantity,
                    symbol: json.symbol,
                    order_type: json.order_type,
                    leverage: json.leverage,
                    price: json.price,
                    tp: json.tp.unwrap_or(0.0),
                    sl: json.sl.unwrap_or(0.0),
                };
                let _ = market.add_limit_order(payload);
            }
            "order.market" => {
                let json: MarketPayload = match serde_json::from_str(payload_str) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let mut markets = markets.lock().await;
                let market = markets
                    .entry(json.symbol.clone())
                    .or_insert_with(|| Market::new(json.symbol.clone()));
                let payload = order_book::MarketPayload {
                    user_id: json.user_id,
                    order_id: json.order_id,
                    quantity: Decimal::from_f64_retain(json.quantity).unwrap_or(Decimal::ZERO),
                    tp: Decimal::from_f64_retain(json.tp.unwrap_or(0.0)).unwrap_or(Decimal::ZERO),
                    sl: Decimal::from_f64_retain(json.sl.unwrap_or(0.0)).unwrap_or(Decimal::ZERO),
                    side: json.side,
                };
                market.market(payload);
            }
            "order.modify" => {
                let json: ModifyPayload = match serde_json::from_str(payload_str) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let mut markets = markets.lock().await;
                if let Some(market) = markets.get_mut(&json.symbol) {
                    let payload = order_book::ModifyPayload {
                        symbol: json.symbol,
                        side: json.side,
                        order_id: json.order_id,
                        has_updated_tp_val: json.tp.is_some(),
                        has_updated_sl_val: json.sl.is_some(),
                        tp: json.tp.and_then(|v| Decimal::from_f64_retain(v)),
                        sl: json.sl.and_then(|v| Decimal::from_f64_retain(v)),
                    };
                    market.modify(payload);
                }
            }
            "order.close" => {
                let json: ClosePayload = match serde_json::from_str(payload_str) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let mut markets = markets.lock().await;
                if let Some(market) = markets.get_mut(&json.symbol) {
                    let payload = order_book::ClosePayload {
                        symbol: json.symbol,
                        side: json.side,
                        quantity: Decimal::from_f64_retain(json.quantity).unwrap_or(Decimal::ZERO),
                        order_id: json.order_id,
                        user_id: json.user_id,
                    };
                    market.close(payload);
                }
            }
            "order.close.all" => {
                let json: CloseAllPayload = match serde_json::from_str(payload_str) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let mut markets = markets.lock().await;
                for market in markets.values_mut() {
                    market.close_all(order_book::CloseAllPayload {
                        user_id: json.user_id.clone(),
                    });
                }
            }
            _ => {
                eprintln!("\n[ERROR] subject not matched, subject: {}", subject)
            }
        }
    }
}