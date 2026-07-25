use rust_decimal::Decimal;

use crate::order_book::{LimitOrderEventPayload, Market, MarketPayload};

fn user_id(i: usize) -> String {
    format!("mock_user_{}", (i % 10) + 1)
}

fn order_id(tag: &str, i: usize) -> String {
    format!("mock_{}_{}", tag, i)
}

pub fn seed_limit_orders(market: &mut Market) {
    for i in 0..50 {
        let price = 100.0 + (i as f64) * 2.0;
        let qty = ((i % 5 + 1) as f64) * 0.1;

        let order = LimitOrderEventPayload {
            user_id: user_id(i),
            order_id: order_id("buy", i),
            side: 1,
            quantity: qty,
            symbol: market.symbol.clone(),
            order_type: "limit".to_string(),
            leverage: 1,
            price,
            tp: price * 1.1,
            sl: price * 0.95,
        };
        let _ = market.add_limit_order(order);
    }

    for i in 0..50 {
        let price = 200.0 - (i as f64) * 2.0;
        let qty = ((i % 5 + 1) as f64) * 0.1;

        let order = LimitOrderEventPayload {
            user_id: user_id(i + 50),
            order_id: order_id("sell", i),
            side: 2,
            quantity: qty,
            symbol: market.symbol.clone(),
            order_type: "limit".to_string(),
            leverage: 1,
            price,
            tp: price * 0.9,
            sl: price * 1.05,
        };
        let _ = market.add_limit_order(order);
    }
}

pub fn seed_market_orders(market: &mut Market, count: usize) {
    for i in 0..count {
        let side = if i % 2 == 0 { 1.0 } else { 2.0 };
        let payload = MarketPayload {
            user_id: user_id(i),
            order_id: order_id("mkt", i),
            quantity: Decimal::from_f64_retain(0.5 + (i % 5) as f64 * 0.1).unwrap(),
            tp: Decimal::ZERO,
            sl: Decimal::ZERO,
            side,
        };
        let _ = market.market(payload);
    }
}
