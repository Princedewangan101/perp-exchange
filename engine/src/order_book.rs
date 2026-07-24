use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap, HashSet},
};

#[derive(Clone, Deserialize)]
struct LimitOrderEventPayload {
    user_id: String,
    order_id: String,
    side: u32,
    quantity: f64,
    symbol: String,
    order_type: String,
    leverage: u32,
    price: f64,
    tp: f64,
    sl: f64,
}

#[derive(Deserialize)]
struct AddLimitOrderResponse {
    success: bool,
    message: String,
    remaining_quantity: Option<f64>,
}

pub struct ModifyPayload {
    pub side: u32,
    pub order_id: String,
    pub has_updated_tp_val: bool,
    pub has_updated_sl_val: bool,
    pub tp: Option<Decimal>,
    pub sl: Option<Decimal>,
}

pub struct ModifyResponse {
    pub success: bool,
    pub tp: Option<Decimal>,
    pub sl: Option<Decimal>,
    pub message: String,
}

pub struct ClosePayload {
    pub side: u32,
    pub quantity: Decimal,
    pub order_id: String,
    pub user_id: String,
}
pub struct CloseResponse {
    pub success: bool,
    pub order_id: Option<String>,
    pub message: String,
}
pub struct CloseAllPayload {
    pub user_id: String,
}
pub struct CloseAllResponse {
    pub success: bool,
    pub message: String,
}
pub struct MarketPayload {
    pub user_id: String,
    pub order_id: String,
    pub quantity: Decimal,
    pub tp: Decimal,
    pub sl: Decimal,
    pub side: f64,
}
pub struct MarketResponse {
    pub success: bool,
    pub price: Decimal,
    pub order_id: String,
    pub message: String,
}

type UserId = String;
type OrderId = String;
type Price = Decimal;
type Tp = Decimal;
type Sl = Decimal;

pub struct Market {
    pub symbol: String,
    pub last_price: Decimal,
    pub buy_order: BTreeMap<std::cmp::Reverse<Price>, Vec<LimitOrderEventPayload>>,
    pub sell_order: BTreeMap<Price, Vec<LimitOrderEventPayload>>,
    pub user_orders: HashMap<UserId, HashSet<OrderId>>,
    pub buy_order_lookup: HashMap<OrderId, (Price, Tp, Sl)>, // on modify the tp sl get updates
    pub sell_order_lookup: HashMap<OrderId, (Price, Tp, Sl)>, // on modify the tp sl get updates
}

impl Market {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            last_price: Decimal::ZERO,
            buy_order: BTreeMap::new(),
            sell_order: BTreeMap::new(),
            user_orders: HashMap::new(),
            buy_order_lookup: HashMap::new(),
            sell_order_lookup: HashMap::new(),
        }
    }

    fn fill_order(&mut self, payload: LimitOrderEventPayload) -> Option<f64> {
        if payload.side == 1 {
            if let Some((price, mut lowest_price_sell_orders_vec)) = self.sell_order.pop_first() {
                self.last_price = price;
                let mut total_quantity_at_this_price: Decimal = Decimal::ZERO;
                let mut payload_quantity: Decimal =
                    Decimal::from_f64_retain(payload.quantity).unwrap();

                for order in lowest_price_sell_orders_vec.iter() {
                    total_quantity_at_this_price +=
                        Decimal::from_f64_retain(order.quantity).unwrap();
                }

                if total_quantity_at_this_price > payload_quantity {
                    // payload_quantity -= lowest_price_sell_orders_vec.iter().quqntity
                    // notify(payload-userId, orderid, payload-userId , orderid)
                    let mut remaining: Vec<LimitOrderEventPayload> = Vec::new();

                    for mut sell_order in lowest_price_sell_orders_vec {
                        // if let Some(pat) = self.order_lookup.get(&sell_order.order_id) {
                        //     unimplemented!();
                        // }
                        if sell_order.user_id == payload.user_id {
                            remaining.push(sell_order);
                            continue;
                        }
                        if payload_quantity == Decimal::ZERO {
                            remaining.push(sell_order);
                            continue;
                        }
                        let sell_order_quantity =
                            Decimal::from_f64_retain(sell_order.quantity).unwrap();

                        if payload_quantity < sell_order_quantity {
                            sell_order.quantity -= payload_quantity.to_f64().unwrap();
                            payload_quantity = Decimal::ZERO;
                            remaining.push(sell_order);
                            // here we are sending event for both user, because now payload_quantity is 0.0
                            // executed_order_notification_event(sell_order.order_id, sell_order.user_id, payload_quantity, payload.order_id, payload.user_id)  // currently, not coded this fn
                        } else {
                            payload_quantity -= sell_order_quantity
                            // here we will send event for one user because the payload_quantity is still remaining.
                            // executed_order_notification_event(sell_order.order_id, sell_order.user_id, sell_order.quantity, null, null)  // currently, not coded this fn
                        }
                    }

                    lowest_price_sell_orders_vec = remaining;
                    self.sell_order.insert(price, lowest_price_sell_orders_vec);

                    return Some(payload_quantity.to_f64().unwrap());
                } else {
                    for sell_order in lowest_price_sell_orders_vec.iter_mut() {
                        payload_quantity -= Decimal::from_f64_retain(sell_order.quantity).unwrap();
                        // executed_order_notification_event(sell_order.order_id, sell_order.user_id, payload_quantity, payload.order_id, payload.user_id)  // currently, not coded this fn
                    }
                    return Some(payload_quantity.to_f64().unwrap());
                }
            } else {
                eprintln!("\n> [ERROR] failed to pop sell order");
                return None;
            }
        } else {
            if let Some((Reverse(price), mut highest_price_buy_orders_vec)) =
                self.buy_order.pop_last()
            {
                self.last_price = price;
                let mut total_quantity_at_this_price: Decimal = Decimal::ZERO;
                let mut payload_quantity: Decimal =
                    Decimal::from_f64_retain(payload.quantity).unwrap();

                for order in highest_price_buy_orders_vec.iter() {
                    total_quantity_at_this_price +=
                        Decimal::from_f64_retain(order.quantity).unwrap();
                }

                if total_quantity_at_this_price > payload_quantity {
                    let mut remaining: Vec<LimitOrderEventPayload> = Vec::new();

                    for mut buy_order in highest_price_buy_orders_vec {
                        if buy_order.user_id == payload.user_id {
                            remaining.push(buy_order);
                            continue;
                        }
                        if payload_quantity == Decimal::ZERO {
                            remaining.push(buy_order);
                            continue;
                        }
                        let buy_order_quantity =
                            Decimal::from_f64_retain(buy_order.quantity).unwrap();

                        if payload_quantity < buy_order_quantity {
                            buy_order.quantity -= payload_quantity.to_f64().unwrap();
                            payload_quantity = Decimal::ZERO;
                            remaining.push(buy_order);
                            // here we are sending event for both user, because now payload_quantity is 0.0
                            // executed_order_notification_event(sell_order.order_id, sell_order.user_id, payload_quantity, payload.order_id, payload.user_id)  // currently, not coded this fn
                        } else {
                            payload_quantity -= buy_order_quantity
                            // here we will send event for one user because the payload_quantity is still remaining.
                            // executed_order_notification_event(sell_order.order_id, sell_order.user_id, sell_order.quantity, null, null)  // currently, not coded this fn
                        }
                    }

                    highest_price_buy_orders_vec = remaining;
                    self.buy_order.insert(price, highest_price_buy_orders_vec);

                    return Some(payload_quantity.to_f64().unwrap());
                } else {
                    for buy_order in highest_price_buy_orders_vec.iter_mut() {
                        payload_quantity -= Decimal::from_f64_retain(buy_order.quantity).unwrap();
                        // executed_order_notification_event(sell_order.order_id, sell_order.user_id, payload_quantity, payload.order_id, payload.user_id)  // currently, not coded this fn
                    }
                    return Some(payload_quantity.to_f64().unwrap());
                }
            } else {
                eprintln!("\n> [ERROR] failed to pop buy order");
                return None;
            }
        }
    }

    pub fn add_limit_order(
        &mut self,
        payload: LimitOrderEventPayload,
    ) -> Result<AddLimitOrderResponse, AddLimitOrderResponse> {
        // assuming from buy side.
        // will check that the sell orders lowest price is less than a payload price.
        // if NO then there is no one willing to sell for that buy order in that case we will just put buy order in Map.
        // if YES then there is some one willing to sell the for the buy order, will check that the qty of sell order is more than a buy order or not and flip orders.

        let price = Decimal::from_f64_retain(payload.price).unwrap();

        if payload.side == 1 {
            if let Some((lowest_sell_order_price, _)) = self.sell_order.first_key_value() {
                if *lowest_sell_order_price > price {
                    self.buy_order
                        .entry(Reverse(price))
                        .or_insert_with(Vec::new)
                        .push(payload.clone());
                    return Ok(AddLimitOrderResponse {
                        success: true,
                        remaining_quantity: None,
                        message: "order in orderbook".to_string(),
                    });
                } else {
                    let remaining_quantity = self.fill_order(payload.clone()).unwrap_or(0.0);
                    if remaining_quantity > 0.0 {
                        self.buy_order
                            .entry(Reverse(price))
                            .or_insert_with(Vec::new)
                            .push(LimitOrderEventPayload {
                                quantity: remaining_quantity,
                                ..payload
                            });
                        return Ok(AddLimitOrderResponse {
                            success: true,
                            remaining_quantity: Some(remaining_quantity),
                            message: "order filled with remaining qty.".to_string(),
                        });
                    }
                    return Ok(AddLimitOrderResponse {
                        success: true,
                        remaining_quantity: None,
                        message: "order fully filled.".to_string(),
                    });
                }
            } else {
                eprint!("\n[ERROR] failed to convert payload price in Decimal");
                return Ok(AddLimitOrderResponse {
                    success: false,
                    remaining_quantity: None,
                    message: "failed to convert payload price in Decimal.".to_string(),
                });
            }
        } else {
            if let Some((lowest_buy_order_price, _)) = self.buy_order.last_key_value() {
                if *lowest_buy_order_price > Reverse(price) {
                    self.buy_order
                        .entry(Reverse(price))
                        .or_insert_with(Vec::new)
                        .push(payload);
                    return Ok(AddLimitOrderResponse {
                        success: true,
                        remaining_quantity: None,
                        message: "order in orderbook".to_string(),
                    });
                } else {
                    let remaining_quantity = self.fill_order(payload.clone()).unwrap_or(0.0);
                    if remaining_quantity > 0.0 {
                        self.buy_order
                            .entry(Reverse(price))
                            .or_insert_with(Vec::new)
                            .push(LimitOrderEventPayload {
                                quantity: remaining_quantity,
                                ..payload
                            });
                        return Ok(AddLimitOrderResponse {
                            success: true,
                            remaining_quantity: Some(remaining_quantity),
                            message: "order filled with remaining qty.".to_string(),
                        });
                    }
                    return Ok(AddLimitOrderResponse {
                        success: true,
                        remaining_quantity: None,
                        message: "order fully filled.".to_string(),
                    });
                }
            } else {
                eprint!("\n[ERROR] failed to convert payload price in Decimal");
                return Ok(AddLimitOrderResponse {
                    success: false,
                    remaining_quantity: None,
                    message: "failed to convert payload price in Decimal.".to_string(),
                });
            }
        }
    }

    pub fn modify(&mut self, payload: ModifyPayload) -> Option<ModifyResponse> {
        if payload.side == 1 {
            if let Some(order_tuple_ref) = self.buy_order_lookup.get_mut(&payload.order_id) {
                if payload.has_updated_tp_val || payload.has_updated_sl_val {
                    if let Some(tp) = payload.tp {
                        order_tuple_ref.1 = tp;
                    }
                    if let Some(sl) = payload.sl {
                        order_tuple_ref.2 = sl;
                    }
                    Some(ModifyResponse {
                        success: true,
                        tp: Some(order_tuple_ref.1),
                        sl: Some(order_tuple_ref.2),
                        message: "both updated".to_string(),
                    })
                } else if !payload.has_updated_tp_val || payload.has_updated_sl_val {
                    if let Some(sl) = payload.sl {
                        order_tuple_ref.2 = sl;
                    }
                    Some(ModifyResponse {
                        success: true,
                        tp: None,
                        sl: Some(order_tuple_ref.2),
                        message: "sl updated".to_string(),
                    })
                } else if payload.has_updated_tp_val || !payload.has_updated_sl_val {
                    if let Some(tp) = payload.tp {
                        order_tuple_ref.1 = tp;
                    }
                    Some(ModifyResponse {
                        success: true,
                        tp: Some(order_tuple_ref.1),
                        sl: None,
                        message: "".to_string(),
                    })
                } else {
                    Some(ModifyResponse {
                        success: false,
                        tp: None,
                        sl: None,
                        message: "tp updated".to_string(),
                    })
                }
            } else {
                Some(ModifyResponse {
                    success: false,
                    tp: None,
                    sl: None,
                    message: "has no tp, sl value to update".to_string(),
                })
            }
        } else {
            if let Some(order_tuple_ref) = self.sell_order_lookup.get_mut(&payload.order_id) {
                if payload.has_updated_tp_val || payload.has_updated_sl_val {
                    if let Some(tp) = payload.tp {
                        order_tuple_ref.1 = tp;
                    }
                    if let Some(sl) = payload.sl {
                        order_tuple_ref.2 = sl;
                    }
                    Some(ModifyResponse {
                        success: true,
                        tp: Some(order_tuple_ref.1),
                        sl: Some(order_tuple_ref.2),
                        message: "both updated".to_string(),
                    })
                } else if !payload.has_updated_tp_val || payload.has_updated_sl_val {
                    if let Some(sl) = payload.sl {
                        order_tuple_ref.2 = sl;
                    }
                    Some(ModifyResponse {
                        success: true,
                        tp: None,
                        sl: Some(order_tuple_ref.2),
                        message: "sl updated".to_string(),
                    })
                } else if payload.has_updated_tp_val || !payload.has_updated_sl_val {
                    if let Some(tp) = payload.tp {
                        order_tuple_ref.1 = tp;
                    }
                    Some(ModifyResponse {
                        success: true,
                        tp: Some(order_tuple_ref.1),
                        sl: None,
                        message: "".to_string(),
                    })
                } else {
                    Some(ModifyResponse {
                        success: false,
                        tp: None,
                        sl: None,
                        message: "tp updated".to_string(),
                    })
                }
            } else {
                Some(ModifyResponse {
                    success: false,
                    tp: None,
                    sl: None,
                    message: "has no tp, sl value to update".to_string(),
                })
            }
        }
    }

    pub fn close(&mut self, payload: ClosePayload) -> CloseResponse {
        // remove from order lookup and order tree
        if payload.side == 1 {
            if let Some((price, _, _)) = self.buy_order_lookup.remove(&payload.order_id) {
                if let Some(orders) = self.buy_order.get_mut(&Reverse(price)) {
                    orders.retain(|o| o.order_id != payload.order_id);
                    if orders.is_empty() {
                        self.buy_order.remove(&Reverse(price));
                    }
                }
            }
        } else {
            if let Some((price, _, _)) = self.sell_order_lookup.remove(&payload.order_id) {
                if let Some(orders) = self.sell_order.get_mut(&price) {
                    orders.retain(|o| o.order_id != payload.order_id);
                    if orders.is_empty() {
                        self.sell_order.remove(&price);
                    }
                }
            }
        }
        // remove from user_orders set
        if let Some(order_ids) = self.user_orders.get_mut(&payload.user_id) {
            order_ids.remove(&payload.order_id);
            if order_ids.is_empty() {
                self.user_orders.remove(&payload.user_id);
            }
        }
        CloseResponse {
            success: true,
            order_id: Some(payload.order_id),
            message: "order removed".to_string(),
        }
    }

    pub fn close_all(&mut self, payload: CloseAllPayload) -> CloseAllResponse {
        // get all order_ids for this user
        if let Some(order_ids) = self.user_orders.remove(&payload.user_id) {
            for order_id in order_ids {
                // remove from buy side if present
                if let Some((price, _, _)) = self.buy_order_lookup.remove(&order_id) {
                    if let Some(orders) = self.buy_order.get_mut(&Reverse(price)) {
                        orders.retain(|o| o.order_id != order_id);
                        if orders.is_empty() {
                            self.buy_order.remove(&Reverse(price));
                        }
                    }
                }
                // remove from sell side if present
                if let Some((price, _, _)) = self.sell_order_lookup.remove(&order_id) {
                    if let Some(orders) = self.sell_order.get_mut(&price) {
                        orders.retain(|o| o.order_id != order_id);
                        if orders.is_empty() {
                            self.sell_order.remove(&price);
                        }
                    }
                }
            }
        }
        CloseAllResponse {
            success: true,
            message: "all order removed".to_string(),
        }
    }

    pub fn market(&mut self, payload: MarketPayload) -> MarketResponse {
        let mut remaining_quantity = payload.quantity;

        if payload.side == 1.0 {
            while remaining_quantity > Decimal::ZERO {
                if let Some((price, orders)) = self.sell_order.pop_first() {
                    self.last_price = price;
                    let mut total_at_price = Decimal::ZERO;
                    for o in orders.iter() {
                        total_at_price += Decimal::from_f64_retain(o.quantity).unwrap();
                    }

                    if total_at_price > remaining_quantity {
                        let mut new_orders = Vec::new();
                        for mut o in orders {
                            let qty = Decimal::from_f64_retain(o.quantity).unwrap();
                            if qty <= remaining_quantity {
                                remaining_quantity -= qty;
                                self.sell_order_lookup.remove(&o.order_id);
                                if let Some(ids) = self.user_orders.get_mut(&o.user_id) {
                                    ids.remove(&o.order_id);
                                }
                            } else {
                                o.quantity -= remaining_quantity.to_f64().unwrap();
                                remaining_quantity = Decimal::ZERO;
                                new_orders.push(o);
                            }
                        }
                        if !new_orders.is_empty() {
                            self.sell_order.insert(price, new_orders);
                        }
                        break;
                    } else {
                        for o in orders {
                            remaining_quantity -= Decimal::from_f64_retain(o.quantity).unwrap();
                            self.sell_order_lookup.remove(&o.order_id);
                            if let Some(ids) = self.user_orders.get_mut(&o.user_id) {
                                ids.remove(&o.order_id);
                            }
                        }
                    }
                } else {
                    return MarketResponse {
                        success: false,
                        price: self.last_price,
                        order_id: payload.order_id,
                        message: "no liquidity on sell side".to_string(),
                    };
                }
            }
        } else {
            while remaining_quantity > Decimal::ZERO {
                if let Some((Reverse(price), orders)) = self.buy_order.pop_last() {
                    self.last_price = price;
                    let mut total_at_price = Decimal::ZERO;
                    for o in orders.iter() {
                        total_at_price += Decimal::from_f64_retain(o.quantity).unwrap();
                    }

                    if total_at_price > remaining_quantity {
                        let mut new_orders = Vec::new();
                        for mut o in orders {
                            let qty = Decimal::from_f64_retain(o.quantity).unwrap();
                            if qty <= remaining_quantity {
                                remaining_quantity -= qty;
                                self.buy_order_lookup.remove(&o.order_id);
                                if let Some(ids) = self.user_orders.get_mut(&o.user_id) {
                                    ids.remove(&o.order_id);
                                }
                            } else {
                                o.quantity -= remaining_quantity.to_f64().unwrap();
                                remaining_quantity = Decimal::ZERO;
                                new_orders.push(o);
                            }
                        }
                        if !new_orders.is_empty() {
                            self.buy_order.insert(Reverse(price), new_orders);
                        }
                        break;
                    } else {
                        for o in orders {
                            remaining_quantity -= Decimal::from_f64_retain(o.quantity).unwrap();
                            self.buy_order_lookup.remove(&o.order_id);
                            if let Some(ids) = self.user_orders.get_mut(&o.user_id) {
                                ids.remove(&o.order_id);
                            }
                        }
                    }
                } else {
                    return MarketResponse {
                        success: false,
                        price: self.last_price,
                        order_id: payload.order_id,
                        message: "no liquidity on buy side".to_string(),
                    };
                }
            }
        }

        MarketResponse {
            success: true,
            price: self.last_price,
            order_id: payload.order_id,
            message: "market order filled".to_string(),
        }
    }
}
