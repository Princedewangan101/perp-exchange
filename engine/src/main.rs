use std::collections::HashMap;

use futures::StreamExt;
use prost::Message;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::Deserialize;

mod order_book;
mod proto;
use order_book::Market;
use proto::{LimitOrderPayload, LimitOrderResult};

// ---- JSON PAYLOADS FOR order.* SUBJECTS ----
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
    let nats = async_nats::connect("127.0.0.1:4222").await.unwrap();
    let mut markets: HashMap<String, Market> = HashMap::new();



    let sub1 = nats.subscribe("order.*").await.unwrap();
    let mut merged = sub1;

    while let Some(message) = merged.next().await {
        let subject = message.subject.as_str();

        match subject {
            "order.limit" => {
                let payload = match LimitOrderPayload::decode(&message.payload[..]) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                println!("\n> [LIMIT_ORDER_ENGINE_RECV]: order_id:{}, user_id:{}, symbol:{}, quantity:{}, side:{}, price:{}, tp:{:?}, sl:{:?}",
                    payload.order_id, payload.user_id, payload.symbol, payload.quantity, payload.side, payload.price, payload.tp, payload.sl);
                let market = markets
                    .entry(payload.symbol.clone())
                    .or_insert_with(|| Market::new(payload.symbol.clone(), nats.clone()));
                let order_id = payload.order_id.clone();
                let result = market.add_limit_order(order_book::LimitOrderEventPayload {
                    user_id: payload.user_id,
                    order_id: payload.order_id,
                    side: payload.side,
                    quantity: payload.quantity,
                    price: payload.price,
                    tp: payload.tp.unwrap_or(0.0),
                    sl: payload.sl.unwrap_or(0.0),
                });
                let reply = match &result {
                    Ok(resp) => LimitOrderResult {
                        success: true,
                        message: resp.message.clone(),
                        remaining_quantity: resp.remaining_quantity,
                    },
                    Err(resp) => LimitOrderResult {
                        success: false,
                        message: resp.message.clone(),
                        remaining_quantity: resp.remaining_quantity,
                    },
                };
                println!("\n> [LIMIT_ORDER_ENGINE_REPLY]: success:{}, message:{}, order_id:{}, remaining_quantity:{:?}",
                    reply.success, reply.message, order_id, reply.remaining_quantity);
                let mut buf = Vec::new();
                if reply.encode(&mut buf).is_ok() {
                    if let Some(reply_subject) = message.reply {
                        let _ = nats.publish(reply_subject, buf.into()).await;
                    }
                }
            }
            "order.market" => {
                let req = match proto::OrderRequest::decode(&message.payload[..]) {
                    Ok(r) => r,
                    Err(_) => continue,
                };
                let market = markets
                    .entry(req.symbol.clone())
                    .or_insert_with(|| Market::new(req.symbol.clone(), nats.clone()));
                let resp = market.market(order_book::MarketPayload {
                    user_id: req.user_id,
                    order_id: req.order_id,
                    quantity: Decimal::from_f64_retain(req.quantity).unwrap_or(Decimal::ZERO),
                    tp: Decimal::ZERO,
                    sl: Decimal::ZERO,
                    side: req.side as f64,
                });
                println!("\n> [MARKET_ORDER_ENGINE_REPLY]: success:{}, message:{}, price:{}, order_id:{}",
                    resp.success, resp.message, resp.price, resp.order_id);
                let reply = proto::OrderResponse {
                    message: resp.message,
                    quantity: 0.0,
                    price: resp.price.to_f64().unwrap_or(0.0),
                };
                let mut buf = Vec::new();
                if reply.encode(&mut buf).is_ok() {
                    if let Some(reply_subject) = message.reply {
                        let _ = nats.publish(reply_subject, buf.into()).await;
                    }
                }
            }
            "order.modify" => {
                let payload_str = match std::str::from_utf8(&message.payload[..]) {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                let json: ModifyPayload = match serde_json::from_str(payload_str) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                if let Some(market) = markets.get_mut(&json.symbol) {
                    market.modify(order_book::ModifyPayload {
                        symbol: json.symbol,
                        side: json.side,
                        order_id: json.order_id,
                        has_updated_tp_val: json.tp.is_some(),
                        has_updated_sl_val: json.sl.is_some(),
                        tp: json.tp.and_then(|v| Decimal::from_f64_retain(v)),
                        sl: json.sl.and_then(|v| Decimal::from_f64_retain(v)),
                    });
                }
            }
            "order.close" => {
                let payload_str = match std::str::from_utf8(&message.payload[..]) {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                let json: ClosePayload = match serde_json::from_str(payload_str) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                if let Some(market) = markets.get_mut(&json.symbol) {
                    market.close(order_book::ClosePayload {
                        symbol: json.symbol,
                        side: json.side,
                        quantity: Decimal::from_f64_retain(json.quantity).unwrap_or(Decimal::ZERO),
                        order_id: json.order_id,
                        user_id: json.user_id,
                    });
                }
            }
            "order.close.all" => {
                let payload_str = match std::str::from_utf8(&message.payload[..]) {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                let json: CloseAllPayload = match serde_json::from_str(payload_str) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
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
