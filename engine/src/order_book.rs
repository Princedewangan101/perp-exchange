use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::Deserialize;
use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap, HashSet},
};

// ---- INCOMING ORDER FROM THE USER ----
#[derive(Clone, Deserialize)]
pub struct LimitOrderEventPayload {
    pub user_id: String,
    pub order_id: String,
    pub side: u32,             // 1 = BUY, 2 = SELL
    pub quantity: f64,
    pub price: f64,
    pub tp: f64,               // TAKE PROFIT LEVEL
    pub sl: f64,               // STOP LOSS LEVEL
}

// ---- RESPONSE AFTER ADDING A LIMIT ORDER ----
#[derive(Deserialize)]
pub struct AddLimitOrderResponse {
    pub success: bool,
    pub message: String,
    pub remaining_quantity: Option<f64>,   // SOME IF ORDER PARTIALLY FILLED
}

// ---- PAYLOAD TO MODIFY TP/SL OF AN EXISTING ORDER ----
pub struct ModifyPayload {
    pub symbol: String,
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

// ---- PAYLOAD TO CLOSE A SINGLE ORDER ----
pub struct ClosePayload {
    pub symbol: String,
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

// ---- PAYLOAD TO CLOSE ALL ORDERS OF A USER ----
pub struct CloseAllPayload {
    pub user_id: String,
}
pub struct CloseAllResponse {
    pub success: bool,
    pub message: String,
}

// ---- PAYLOAD FOR A MARKET ORDER (FILLS IMMEDIATELY AT BEST PRICE) ----
pub struct MarketPayload {
    pub user_id: String,
    pub order_id: String,
    pub quantity: Decimal,
    pub tp: Decimal,
    pub sl: Decimal,
    pub side: f64,          // 1.0 = BUY, OTHER = SELL
}
pub struct MarketResponse {
    pub success: bool,
    pub price: Decimal,     // PRICE AT WHICH THE ORDER WAS FILLED
    pub order_id: String,
    pub message: String,
}

type UserId = String;
type OrderId = String;
type Price = Decimal;
type Tp = Decimal;
type Sl = Decimal;

// ---- MARKET STATE FOR ONE TRADING SYaMBOL (e.g. BTC) ----
pub struct Market {
    pub symbol: String,
    pub last_price: Decimal,                                            // LAST TRADED PRICE, UPDATED AFTER EVERY FILL
    pub buy_order: BTreeMap<std::cmp::Reverse<Price>, Vec<LimitOrderEventPayload>>,    // BUY ORDERS SORTED HIGHEST-TO-LOWEST PRICE
    pub sell_order: BTreeMap<Price, Vec<LimitOrderEventPayload>>,      // SELL ORDERS SORTED LOWEST-TO-HIGHEST PRICE
    pub user_orders: HashMap<UserId, HashSet<OrderId>>,                // TRACKS WHICH USER OWNS WHICH ORDERS
    pub buy_order_lookup: HashMap<OrderId, (Price, Tp, Sl)>,           // QUICK LOOKUP: ORDER_ID -> (PRICE, TP, SL) FOR BUYS
    pub sell_order_lookup: HashMap<OrderId, (Price, Tp, Sl)>,          // QUICK LOOKUP: ORDER_ID -> (PRICE, TP, SL) FOR SELLS
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

    // ---- CORE MATCHING ENGINE: ATTEMPT TO FILL A NEW ORDER AGAINST THE ORDER BOOK ----
    // RETURNS THE REMAINING QUANTITY (0.0 = FULLY FILLED, >0.0 = PARTIALLY FILLED)
    fn fill_order(&mut self, payload: LimitOrderEventPayload) -> Option<f64> {
        if payload.side == 1 {
            // BUY ORDER: MATCH AGAINST THE LOWEST-PRICED SELL ORDERS
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
                    // SELL ORDERS AT THIS PRICE EXCEED OUR BUY QUANTITY -> PARTIAL FILL
                    let mut remaining: Vec<LimitOrderEventPayload> = Vec::new();

                    for mut sell_order in lowest_price_sell_orders_vec {
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
                        } else {
                            payload_quantity -= sell_order_quantity
                        }
                    }

                    lowest_price_sell_orders_vec = remaining;
                    self.sell_order.insert(price, lowest_price_sell_orders_vec);

                    return Some(payload_quantity.to_f64().unwrap());
                } else {
                    // SELL ORDERS AT THIS PRICE <= OUR BUY QUANTITY -> FULL CONSUMPTION OF THIS LEVEL
                    for sell_order in lowest_price_sell_orders_vec.iter_mut() {
                        payload_quantity -= Decimal::from_f64_retain(sell_order.quantity).unwrap();
                    }
                    return Some(payload_quantity.to_f64().unwrap());
                }
            } else {
                eprintln!("\n> [ERROR] failed to pop sell order");
                return None;
            }
        } else {
            // SELL ORDER: MATCH AGAINST THE HIGHEST-PRICED BUY ORDERS
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
                    // BUY ORDERS AT THIS PRICE EXCEED OUR SELL QUANTITY -> PARTIAL FILL
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
                        } else {
                            payload_quantity -= buy_order_quantity
                        }
                    }

                    highest_price_buy_orders_vec = remaining;
                    self.buy_order.insert(Reverse(price), highest_price_buy_orders_vec);

                    return Some(payload_quantity.to_f64().unwrap());
                } else {
                    // BUY ORDERS AT THIS PRICE <= OUR SELL QUANTITY -> FULL CONSUMPTION
                    for buy_order in highest_price_buy_orders_vec.iter_mut() {
                        payload_quantity -= Decimal::from_f64_retain(buy_order.quantity).unwrap();
                    }
                    return Some(payload_quantity.to_f64().unwrap());
                }
            } else {
                eprintln!("\n> [ERROR] failed to pop buy order");
                return None;
            }
        }
    }

    // ---- ADD A LIMIT ORDER ----
    // IF THE ORDER CAN BE FILLED AGAINST EXISTING ORDERS -> MATCH IT
    // OTHERWISE PUT IT INTO THE ORDER BOOK
    pub fn add_limit_order(
        &mut self,
        payload: LimitOrderEventPayload,
    ) -> Result<AddLimitOrderResponse, AddLimitOrderResponse> {
        println!("\n> [LIMIT_ORDER_BOOK]: order_id:{}, user_id:{}, side:{}, quantity:{}, price:{}, tp:{}, sl:{}",
            payload.order_id, payload.user_id, payload.side, payload.quantity, payload.price, payload.tp, payload.sl);
        let price = Decimal::from_f64_retain(payload.price).unwrap();

        if payload.side == 1 {
            // BUY ORDER
            if let Some((lowest_sell_order_price, _)) = self.sell_order.first_key_value() {
                if *lowest_sell_order_price > price {
                    // CHEAPEST SELL IS ABOVE OUR BUY PRICE -> NO MATCH, PUT IN BOOK
                    self.buy_order
                        .entry(Reverse(price))
                        .or_insert_with(Vec::new)
                        .push(payload.clone());
                    self.buy_order_lookup.insert(
                        payload.order_id.clone(),
                        (price,
                        Decimal::from_f64_retain(payload.tp).unwrap(),
                        Decimal::from_f64_retain(payload.sl).unwrap()),
                    );
                    self.user_orders
                        .entry(payload.user_id.clone())
                        .or_insert_with(HashSet::new)
                        .insert(payload.order_id.clone());
                    return Ok(AddLimitOrderResponse {
                        success: true,
                        remaining_quantity: None,
                        message: "order in orderbook".to_string(),
                    });
                } else {
                    // THERE ARE SELL ORDERS AT OR BELOW OUR BUY PRICE -> TRY TO FILL
                    let remaining_quantity = self.fill_order(payload.clone()).unwrap_or(0.0);
                    if remaining_quantity > 0.0 {
                        // PARTIALLY FILLED: PUT THE REST INTO THE BOOK
                        let order_id = payload.order_id.clone();
                        let user_id = payload.user_id.clone();
                        let tp = Decimal::from_f64_retain(payload.tp).unwrap();
                        let sl = Decimal::from_f64_retain(payload.sl).unwrap();
                        self.buy_order
                            .entry(Reverse(price))
                            .or_insert_with(Vec::new)
                            .push(LimitOrderEventPayload {
                                quantity: remaining_quantity,
                                ..payload
                            });
                        self.buy_order_lookup.insert(order_id.clone(), (price, tp, sl));
                        self.user_orders
                            .entry(user_id.clone())
                            .or_insert_with(HashSet::new)
                            .insert(order_id);
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
                // EMPTY ORDER BOOK -> PUT BUY ORDER IN THE BOOK
                self.buy_order
                    .entry(Reverse(price))
                    .or_insert_with(Vec::new)
                    .push(payload.clone());
                self.buy_order_lookup.insert(
                    payload.order_id.clone(),
                    (price,
                    Decimal::from_f64_retain(payload.tp).unwrap(),
                    Decimal::from_f64_retain(payload.sl).unwrap()),
                );
                self.user_orders
                    .entry(payload.user_id.clone())
                    .or_insert_with(HashSet::new)
                    .insert(payload.order_id.clone());
                return Ok(AddLimitOrderResponse {
                    success: true,
                    remaining_quantity: None,
                    message: "order in orderbook".to_string(),
                });
            }
        } else {
            // SELL ORDER
            if let Some((lowest_buy_order_price, _)) = self.buy_order.last_key_value() {
                if *lowest_buy_order_price > Reverse(price) {
                    // HIGHEST BUY IS BELOW OUR SELL PRICE -> NO MATCH, PUT IN BOOK
                    let order_id = payload.order_id.clone();
                    let user_id = payload.user_id.clone();
                    let tp = Decimal::from_f64_retain(payload.tp).unwrap();
                    let sl = Decimal::from_f64_retain(payload.sl).unwrap();
                    self.sell_order
                        .entry(price)
                        .or_insert_with(Vec::new)
                        .push(payload);
                    self.sell_order_lookup.insert(order_id.clone(), (price, tp, sl));
                    self.user_orders
                        .entry(user_id.clone())
                        .or_insert_with(HashSet::new)
                        .insert(order_id);
                    return Ok(AddLimitOrderResponse {
                        success: true,
                        remaining_quantity: None,
                        message: "order in orderbook".to_string(),
                    });
                } else {
                    // THERE ARE BUY ORDERS AT OR ABOVE OUR SELL PRICE -> TRY TO FILL
                    let remaining_quantity = self.fill_order(payload.clone()).unwrap_or(0.0);
                    if remaining_quantity > 0.0 {
                        // PARTIALLY FILLED: PUT THE REST INTO THE BOOK
                        let order_id = payload.order_id.clone();
                        let user_id = payload.user_id.clone();
                        let tp = Decimal::from_f64_retain(payload.tp).unwrap();
                        let sl = Decimal::from_f64_retain(payload.sl).unwrap();
                        self.sell_order
                            .entry(price)
                            .or_insert_with(Vec::new)
                            .push(LimitOrderEventPayload {
                                quantity: remaining_quantity,
                                ..payload
                            });
                        self.sell_order_lookup.insert(order_id.clone(), (price, tp, sl));
                        self.user_orders
                            .entry(user_id.clone())
                            .or_insert_with(HashSet::new)
                            .insert(order_id);
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
                // EMPTY ORDER BOOK -> PUT SELL ORDER IN THE BOOK
                let order_id = payload.order_id.clone();
                let user_id = payload.user_id.clone();
                let tp = Decimal::from_f64_retain(payload.tp).unwrap();
                let sl = Decimal::from_f64_retain(payload.sl).unwrap();
                self.sell_order
                    .entry(price)
                    .or_insert_with(Vec::new)
                    .push(payload);
                self.sell_order_lookup.insert(order_id.clone(), (price, tp, sl));
                self.user_orders
                    .entry(user_id.clone())
                    .or_insert_with(HashSet::new)
                    .insert(order_id);
                return Ok(AddLimitOrderResponse {
                    success: true,
                    remaining_quantity: None,
                    message: "order in orderbook".to_string(),
                });
            }
        }
    }

    // ---- MODIFY TP/SL OF AN EXISTING ORDER IN THE LOOKUP MAP ----
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

    // ---- CLOSE A SINGLE ORDER: REMOVE FROM LOOKUP, ORDER TREE, AND USER_ORDERS ----
    pub fn close(&mut self, payload: ClosePayload) -> CloseResponse {
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

    // ---- CLOSE ALL ORDERS FOR A USER: ITERATE AND REMOVE EACH FROM EVERY DATA STRUCTURE ----
    pub fn close_all(&mut self, payload: CloseAllPayload) -> CloseAllResponse {
        if let Some(order_ids) = self.user_orders.remove(&payload.user_id) {
            for order_id in order_ids {
                if let Some((price, _, _)) = self.buy_order_lookup.remove(&order_id) {
                    if let Some(orders) = self.buy_order.get_mut(&Reverse(price)) {
                        orders.retain(|o| o.order_id != order_id);
                        if orders.is_empty() {
                            self.buy_order.remove(&Reverse(price));
                        }
                    }
                }
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

    // ---- MARKET ORDER: IMMEDIATELY FILL AT BEST AVAILABLE PRICES, NO BOOK PLACEMENT ----
    // LOOPS THROUGH PRICE LEVELS UNTIL FULLY FILLED OR LIQUIDITY EXHAUSTED
    pub fn market(&mut self, payload: MarketPayload) -> MarketResponse {
        let mut remaining_quantity = payload.quantity;

        if payload.side == 1.0 {
            // BUY MARKET: CONSUME FROM THE CHEAPEST SELL ORDERS
            while remaining_quantity > Decimal::ZERO {
                if let Some((price, orders)) = self.sell_order.pop_first() {
                    self.last_price = price;
                    let mut total_at_price = Decimal::ZERO;
                    for o in orders.iter() {
                        total_at_price += Decimal::from_f64_retain(o.quantity).unwrap();
                    }

                    if total_at_price > remaining_quantity {
                        // MORE SELLERS AT THIS PRICE THAN WE NEED -> PARTIAL FILL, PUT REST BACK
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
                        // CONSUME ALL SELLERS AT THIS PRICE, CONTINUE TO NEXT LEVEL
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
            // SELL MARKET: CONSUME FROM THE HIGHEST-PRICED BUY ORDERS
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