use std::result::Result::Ok;

use rust_decimal::Decimal;
use serde::Serialize;
use tokio_postgres::Client;

pub struct UserStatusResponse {
    pub is_user_exist: bool,
    pub email: Option<String>,
}
pub struct UserCreationResponse {
    pub success: bool,
    pub id: String,
}

#[derive(Serialize)]
pub struct Order {
    pub order_id: i32,
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

#[derive(Serialize)]
pub struct Transaction {
    pub transaction_id: i32,
    pub balance: f64,
    pub transaction_type: String,
    pub created_at: Option<String>,
}

#[derive(Serialize)]
pub struct FetchTransactionsResponse {
    pub success: bool,
    pub message: String,
    pub transactions: Option<Vec<Transaction>>,
}

#[derive(Debug)]
pub struct User {
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub balance: Option<i64>,
}

pub struct Deposit {
    pub success: bool,
    pub balance: Option<f64>,
}

pub async fn is_user_exist(postgres_client: &Client, email: &str) -> UserStatusResponse {
    let search_query_result = postgres_client
        .query_opt(
            "SELECT userId, email FROM users WHERE email = $1 ",
            &[&email],
        )
        .await;

    match search_query_result {
        Ok(Some(row)) => {
            println!("\n> user created");
            // let name_found:String = row.get(0);
            let email_found: String = row.get(1);
            UserStatusResponse {
                is_user_exist: true,
                email: Some(email_found),
            }
        }
        Ok(None) => UserStatusResponse {
            is_user_exist: false,
            email: None,
        },
        Err(err) => {
            log_db_error("is_user_exist", &err);
            UserStatusResponse {
                is_user_exist: false,
                email: None,
            }
        }
    }
}

pub async fn create_user(
    postgres_client: &Client,
    email: &str,
    password: &str,
) -> UserCreationResponse {
    let insert_query_result = postgres_client
        .query_one(
            "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING userId",
            &[&email, &password],
        )
        .await;

    match insert_query_result {
        Ok(row) => {
            let id: i32 = row.get(0);
            println!("\n> Created user with ID: {}", id);
            UserCreationResponse {
                success: true,
                id: id.to_string(),
            }
        }
        Err(err) => {
            log_db_error("create_user", &err);
            UserCreationResponse {
                success: false,
                id: "".to_string(),
            }
        }
    }
}

pub async fn find_user(postgres_client: &Client, email: &str) -> User {
    let search_query_result = postgres_client
        .query_opt(
            "SELECT userId, email, balance::BIGINT FROM users WHERE email = $1",
            &[&email],
        )
        .await;

    match search_query_result {
        Ok(Some(row)) => {
            // println!("> [SUCCESS] Row found matching email!");

            // let name_found:String = row.get(0);
            let user_id_found: i32 = row.get(0);
            let email_found: String = row.get(1);
            let balance_found: i64 = row.get(2);
            User {
                user_id: Some(user_id_found.to_string()),
                email: Some(email_found),
                balance: Some(balance_found),
            }
        }
        Ok(None) => {
            // println!(
            //     "> [NOTICE] Ok(None) - Database executed successfully, but email '{}' does NOT exist in the users table.",
            //     email
            // );
            User {
                user_id: None,
                email: None,
                balance: None,
            }
        }
        Err(err) => {
            log_db_error("find_user", &err);
            User {
                user_id: None,
                email: None,
                balance: None,
            }
        }
    }
}

pub async fn deposit_balance(postgres_client: &Client, user_id: &str, amount: &f64) -> Deposit {
        println!("\n>[INFO] deposit route , TRIGGERED\n amount: {}", amount);
    let user_id: i32 = match user_id.parse() {
        Ok(id) => id,
        Err(_) => {
            return Deposit {
                success: false,
                balance: None,
            };
        }
    };

    let pg_amount = Decimal::from_f64_retain(*amount).unwrap();

    let post_query_result = postgres_client
        .query_one(
            "UPDATE users SET balance = balance + $1 WHERE userId = $2 RETURNING balance::double precision",
            &[&pg_amount, &user_id],
        )
        .await;

    match post_query_result {
        Ok(row) => {
            println!("\n>[INFO] deposit route , SUCCESS");
            let balance = row.get::<_, f64>(0);
            Deposit {
                success: true,
                balance: Some(balance),
            }
        }
        Err(err) => {
            log_db_error("deposit_balance", &err);
            Deposit {
                success: false,
                balance: None,
            }
        }
    }
}

pub async fn withdraw_balance(postgres_client: &Client, user_id: &str, amount: &f64) -> bool {
    let user_id: i32 = match user_id.parse() {
        Ok(id) => id,
        Err(_) => return false,
    };

    let pg_amount = Decimal::from_f64_retain(*amount).unwrap();

    let withdraw_query_result = postgres_client
        .query_one(
            "UPDATE users SET balance = balance - $1 WHERE userId = $2 AND balance >= $3 RETURNING balance::double precision",
            &[&pg_amount, &user_id, &pg_amount],
        )
        .await;

    match withdraw_query_result {
        Ok(_) => {
            return true;
        }
        Err(err) => {
            log_db_error("withdraw_balance", &err);
            return false;
        }
    }
}

pub async fn limit_order(
    postgres_client: &Client,
    user_id: &str,
    symbol: &str,
    quantity: &f64,
    side: &u32,
    order_type: &str,
    status: String,
    leverage: &u32,
    tp: &f64,
    sl: &f64,
    open: &f64,
) -> Option<String> {
    let pg_side = *side as i16;
    let pg_leverage = *leverage as i16;
    let user_id: i32 = user_id.parse().ok()?;
    let pg_quantity = Decimal::from_f64_retain(*quantity).unwrap();
    let pg_tp = Decimal::from_f64_retain(*tp).unwrap();
    let pg_sl = Decimal::from_f64_retain(*sl).unwrap();
    let pg_open = Decimal::from_f64_retain(*open).unwrap();

    let result;

    if *tp == 0.0 && *sl == 0.0 {
        result = postgres_client
            .query_one(
                "INSERT INTO orders (userId, symbol, quantity, side, type, status, leverage, open) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             RETURNING orderId",
                &[
                    &user_id,
                    &symbol,
                    &pg_quantity,
                    &pg_side,
                    &order_type,
                    &status,
                    &pg_leverage,
                    &pg_open,
                ],
            )
            .await;
    } else if *tp == 0.0 {
        result = postgres_client
            .query_one(
                "INSERT INTO orders (userId, symbol, quantity, side, type, status, leverage, sl, open) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             RETURNING orderId",
                &[
                    &user_id,
                    &symbol,
                    &pg_quantity,
                    &pg_side,
                    &order_type,
                    &status,
                    &pg_leverage,
                    &pg_sl,
                    &pg_open,
                ],
            )
            .await;
    } else if *sl == 0.0 {
        result = postgres_client
            .query_one(
                "INSERT INTO orders (userId, symbol, quantity, side, type, status, leverage, tp, open) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             RETURNING orderId",
                &[
                    &user_id,
                    &symbol,
                    &pg_quantity,
                    &pg_side,
                    &order_type,
                    &status,
                    &pg_leverage,
                    &pg_tp,
                    &pg_open,
                ],
            )
            .await;
    } else {
        result = postgres_client
            .query_one(
                "INSERT INTO orders (userId, symbol, quantity, side, type, status, leverage, tp, sl, open) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             RETURNING orderId",
                &[
                    &user_id,
                    &symbol,
                    &pg_quantity,
                    &pg_side,
                    &order_type,
                    &status,
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
            Some(order_id.to_string())
        }
        Err(err) => {
            log_db_error("limit_order", &err);
            return None;
        }
    }
}

pub async fn modify_order(
    postgres_client: &Client,
    user_id: &str,
    order_id: &str,
    tp: &f64,
    sl: &f64,
) -> Option<(f64, f64)> {
    // println!("\n>[INFO] modify query , TRIGGERED");
    let user_id: i32 = user_id.parse().ok()?;
    let order_id: i32 = order_id.parse().ok()?;

    let pg_tp = Decimal::from_f64_retain(*tp).unwrap();
    let pg_sl = Decimal::from_f64_retain(*sl).unwrap();

    let modify_query_response;

    if *tp != 0.0 && *sl != 0.0 {
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
                // println!("\n>[INFO] modify , SUCCESS");
                return Some((updated_tp, updated_sl));
            }
            Err(err) => {
                log_db_error("modify_order (both)", &err);
                return None;
            }
        }
    } else if *tp != 0.0 {
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
                // println!("\n>[INFO] modify , SUCCESS");
                return Some((updated_tp, updated_sl));
            }
            Err(err) => {
                log_db_error("modify_order (tp only)", &err);
                return None;
            }
        }
    } else if *sl != 0.0 {
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
                // println!("\n>[INFO] modify , SUCCESS");
                return Some((updated_tp, updated_sl));
            }
            Err(err) => {
                log_db_error("modify_order (sl only)", &err);
                return None;
            }
        }
    } else {
        // println!("\n>[INFO] modify query , None\ntp: {} sl: {}", pg_tp, pg_sl);
        return None;
    }
}

pub async fn close_order(
    postgres_client: &Client,
    user_id: &str,
    order_id: &str,
    close_price: &f64,
    close_type: &str,
) -> bool {
    // println!("\n>[INFO] close query , TRIGGERED");
    let user_id: i32 = match user_id.parse() {
        Ok(id) => id,
        Err(_) => return false,
    };
    let order_id: i32 = match order_id.parse() {
        Ok(id) => id,
        Err(_) => return false,
    };
    let pg_close_price = Decimal::from_f64_retain(*close_price).unwrap();

    let close_query_result = postgres_client
        .query_one(
            "UPDATE orders SET close = $1, closeType = $2, status = 'completed' WHERE userId = $3 AND orderId = $4 RETURNING close::double precision",
            &[&pg_close_price, &close_type, &user_id, &order_id],
        )
        .await;

    match close_query_result {
        Ok(row) => {
            // println!(
            //     "\n>[INFO] close query , SUCCESS\nclose: {}",
            //     row.get::<_, f64>(0)
            // );
            return true;
        }
        Err(err) => {
            log_db_error("close_order", &err);
            return false;
        }
    }
}

#[derive(Serialize)]
pub struct UpdateBalanceResponse {
    pub success: bool,
    pub balance: Option<f64>,
}

pub async fn update_balance(
    postgres_client: &Client,
    order_id: &str,
    user_id: &str,
    close_price: &f64,
) -> UpdateBalanceResponse {
    // println!("\n>[INFO] updatebalance query , TRIGGERED");
    let order_id: i32 = match order_id.parse() {
        Ok(id) => id,
        Err(_) => {
            return UpdateBalanceResponse {
                success: false,
                balance: None,
            };
        }
    };
    let user_id: i32 = match user_id.parse() {
        Ok(id) => id,
        Err(_) => {
            return UpdateBalanceResponse {
                success: false,
                balance: None,
            };
        }
    };

    let order_result = postgres_client
        .query_opt(
            "SELECT open::double precision, side, leverage FROM orders WHERE orderId = $1 AND userId = $2",
            &[&order_id, &user_id],
        )
        .await;

    match order_result {
        Ok(Some(row)) => {
            let open_price_found: f64 = row.get(0);
            let side_found: u32 = row.get::<_, i16>(1) as u32;
            let leverage_found: u32 = row.get::<_, i16>(2) as u32;

            let is_profit: bool;
            let sum: f64;

            if side_found == 0 {
                // sell side
                let diff = open_price_found - *close_price;
                if diff > 0.0 {
                    is_profit = true
                } else {
                    is_profit = false
                }
                sum = diff.abs() * leverage_found as f64
            } else {
                // buy side
                let diff = *close_price - open_price_found;
                if diff > 0.0 {
                    is_profit = true
                } else {
                    is_profit = false
                }
                sum = diff.abs() * leverage_found as f64
            }

            let pg_sum = Decimal::from_f64_retain(sum).unwrap_or_default();

            if is_profit {
                let balance_update_query_response = postgres_client
                    .query_one(
                        "UPDATE users SET balance = balance + $1 WHERE userId = $2 RETURNING balance::double precision",
                        &[&pg_sum, &user_id],
                    )
                    .await;
                match balance_update_query_response {
                    Ok(row) => {
                        // println!("\n>[INFO] updatebalance query , SUCCESS");
                        return UpdateBalanceResponse {
                            success: true,
                            balance: Some(row.get::<_, f64>(0)),
                        };
                    }
                    Err(err) => {
                        log_db_error("update_balance (profit)", &err);
                        return UpdateBalanceResponse {
                            success: false,
                            balance: None,
                        };
                    }
                }
            } else {
                let balance_update_query_response = postgres_client
                .query_one(
                    "UPDATE users SET balance = balance - $1 WHERE userId = $2 AND balance >= $3 RETURNING balance::double precision",
                    &[&pg_sum, &user_id, &pg_sum],
                )
                .await;
                match balance_update_query_response {
                    Ok(row) => {
                        // println!("\n>[INFO] updatebalance query , SUCCESS");
                        return UpdateBalanceResponse {
                            success: true,
                            balance: Some(row.get::<_, f64>(0)),
                        };
                    }
                    Err(err) => {
                        log_db_error("update_balance (loss)", &err);
                        return UpdateBalanceResponse {
                            success: false,
                            balance: None,
                        };
                    }
                }
            }
        }
        Ok(None) => {
            return UpdateBalanceResponse {
                success: false,
                balance: None,
            };
        }
        Err(err) => {
            log_db_error("update_balance (fetch order)", &err);
            return UpdateBalanceResponse {
                success: false,
                balance: None,
            };
        }
    }
}

pub async fn fetch_orders_from_db(postgres_client: &Client, user_id: &str) -> FetchOrdersResponse {
    // println!("\n\n> route: fetch_orders_from_db(), get triggererd.");
    let user_id: i32 = match user_id.parse() {
        Ok(id) => id,
        Err(_) => {
            return FetchOrdersResponse {
                success: false,
                message: "Invalid user ID".to_string(),
                orders: None,
            };
        }
    };
    let result = postgres_client.query(
        "SELECT orderId, symbol, quantity::double precision, side::int2, type, status, tp::double precision, sl::double precision, open::double precision, close::double precision, closeType, pnl::double precision, created_at::text, updated_at::text FROM orders WHERE userId = $1",
        &[&user_id],
    ).await;

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

    // println!("\n\n> route: fetch_orders_from_db(), success,");
    return FetchOrdersResponse {
        success: true,
        message: "Orders fetched successfully".to_string(),
        orders: Some(orders_list),
    };
}

pub async fn fetch_transactions_from_db(
    postgres_client: &Client,
    user_id: &str,
) -> FetchTransactionsResponse {
    // println!("\n>[INFO] fetch_transactions_from_db , get triggered");
    let user_id: i32 = match user_id.parse() {
        Ok(id) => id,
        Err(_) => {
            return FetchTransactionsResponse {
                success: false,
                message: "Invalid user ID".to_string(),
                transactions: None,
            };
        }
    };
    let result = postgres_client
        .query(
            "SELECT transactionid, balance::double precision, type, created_at::text FROM transactions WHERE userid = $1",
            &[&user_id],
        )
        .await;

    let rows = match result {
        Ok(v) => v,
        Err(err) => {
            log_db_error("fetch_transactions_from_db", &err);
            return FetchTransactionsResponse {
                success: false,
                message: format!("Database error: {}", err),
                transactions: None,
            };
        }
    };

    let transactions_list = rows
        .iter()
        .map(|row| Transaction {
            transaction_id: row.get(0),
            balance: row.get(1),
            transaction_type: row.get(2),
            created_at: row.get(3),
        })
        .collect();

    // println!("\n>[INFO] fetch_transactions_from_db , success");
    return FetchTransactionsResponse {
        success: true,
        message: "Orders fetched successfully".to_string(),
        transactions: Some(transactions_list),
    };
}

fn log_db_error(context: &str, err: &tokio_postgres::Error) {
    eprintln!("\n[ERROR] {} failed!", context);

    eprintln!("\n[INFO] Raw Debug Info: {:#?}", err);

    if let Some(db_error) = err.as_db_error() {
        eprintln!("\n--- Postgres Engine Error Details ---");
        eprintln!("Code:       {}", db_error.code().code());
        eprintln!("Severity:   {}", db_error.severity());
        eprintln!("Message:    {}", db_error.message());

        if let Some(detail) = db_error.detail() {
            eprintln!("Detail:     {}", detail);
        }
        if let Some(hint) = db_error.hint() {
            eprintln!("Hint:       {}", hint);
        }
        if let Some(table) = db_error.table() {
            eprintln!("Table:      {}", table);
        }
        if let Some(constraint) = db_error.constraint() {
            eprintln!("Constraint: {}", constraint);
        }
        if let Some(datatype) = db_error.datatype() {
            eprintln!("Data Type:  {}", datatype);
        }
        eprintln!("-------------------------------------");
    }
}
