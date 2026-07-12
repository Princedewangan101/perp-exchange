use std::path::absolute;

use tokio_postgres::Client;

pub struct UserStatusResponse {
    pub is_user_exist: bool,
    pub email: Option<String>,
}
pub struct UserCreationResponse {
    pub success: bool,
    pub id: String,
}

#[derive(Debug)]
pub struct User {
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub balance: Option<i64>,
}

pub struct Deposit {
    pub success: bool,
    pub balance: Option<i64>,
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
            eprintln!("\n[ERROR] isUserExist err: {}", err.to_string());
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
            eprintln!("\n[ERROR] create_user : {}", err);
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
            eprintln!("\n[ERROR] isUserExist err: {}", err.to_string());
            User {
                user_id: None,
                email: None,
                balance: None,
            }
        }
    }
}

pub async fn deposit_balance(postgres_client: &Client, user_id: &str, amount: &str) -> Deposit {
    let post_query_result = postgres_client
        .query_one(
            "UPDATE users SET balance = balance + $1 WHERE id = $2 RETURNING balance",
            &[&amount, &user_id],
        )
        .await;

    match post_query_result {
        Ok(row) => {
            let balance = row.get(0);
            Deposit {
                success: true,
                balance: balance,
            }
        }
        Err(err) => {
            eprintln!("\n[ERROR] deposit_balance : {}", err);
            Deposit {
                success: false,
                balance: None,
            }
        }
    }
}

pub async fn withdraw_balance(postgres_client: &Client, user_id: &str, amount: &str) -> bool {
    let withdraw_query_result = postgres_client
        .query_one(
            "UPDATE users SET balance = balance - $1 WHERE userId = $2 AND balance >= $3",
            &[&amount, &user_id, &amount],
        )
        .await;

    match withdraw_query_result {
        Ok(_) => {
            return true;
        }
        Err(err) => {
            eprintln!("\n[ERROR] withdraw_balance : {}", err);
            return false;
        }
    }
}

pub async fn limit_order(
    postgres_client: &Client,
    user_id: &str,
    symbol: &str,
    quantity: &u32,
    side: &u32,
    order_type: &str,
    status: String,
    leverage: &u32,
    tp: &u64,
    sl: &u64,
    open: &u64,
) -> bool {
    let pg_quantity = *quantity as i32;
    let pg_side = *side as i16;
    let pg_tp = *tp as i64;
    let pg_sl = *sl as i64;
    let pg_open = *open as i64;

    let withdraw_query_result;

    if pg_tp == 0 && pg_sl == 0 {
        withdraw_query_result = postgres_client
            .query_one(
                "INSERT INTO orders (userId, symbol, quantity, side, type, status, leverage, open) 
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
                &[
                    &user_id,
                    &symbol,
                    &pg_quantity,
                    &pg_side,
                    &order_type,
                    &status,
                    &leverage,
                    &pg_open,
                ],
            )
            .await;
    } else if pg_tp == 0 {
        withdraw_query_result = postgres_client
            .query_one(
                "INSERT INTO orders (userId, symbol, quantity, side, type, status, leverage, sl, open) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                &[
                    &user_id,
                    &symbol,
                    &pg_quantity,
                    &pg_side,
                    &order_type,
                    &status,
                    &leverage,
                    &pg_sl,
                    &pg_open,
                ],
            )
            .await;
    } else if pg_sl == 0 {
        withdraw_query_result = postgres_client
            .query_one(
                "INSERT INTO orders (userId, symbol, quantity, side, type, status, leverage, tp, open) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                &[
                    &user_id,
                    &symbol,
                    &pg_quantity,
                    &pg_side,
                    &order_type,
                    &status,
                    &leverage,
                    &pg_tp,
                    &pg_open,
                ],
            )
            .await;
    } else {
        withdraw_query_result = postgres_client
            .query_one(
                "INSERT INTO orders (userId, symbol, quantity, side, type, status, leverage, tp, sl, open) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
                &[
                    &user_id,
                    &symbol,
                    &pg_quantity,
                    &pg_side,
                    &order_type,
                    &status,
                    &leverage,
                    &pg_tp,
                    &pg_sl,
                    &pg_open,
                ],
            )
            .await;
    }

    match withdraw_query_result {
        Ok(_) => true,
        Err(err) => {
            eprintln!("\n[ERROR] limit_order query failed: {}", err);
            false
        }
    }
}

pub async fn modify_order(postgres_client: &Client, user_id: &str, tp: &u64, sl: &u64) -> bool {
    let pg_tp = *tp as i64;
    let pg_sl = *sl as i64;

    let modify_query_response;

    if pg_tp == 0 {
        modify_query_response = postgres_client
            .query_one(
                "UPDATE users SET sl = $1 WHERE userId = $2",
                &[&pg_sl, &user_id],
            )
            .await;
        match modify_query_response {
            Ok(_) => {
                return true;
            }
            Err(err) => {
                eprintln!("\n[ERROR] modify_query : {}", err);
                return false;
            }
        }
    } else if pg_sl == 0 {
        modify_query_response = postgres_client
            .query_one(
                "UPDATE users SET tp = $1 WHERE userId = $2",
                &[&pg_tp, &user_id],
            )
            .await;
        match modify_query_response {
            Ok(_) => {
                return true;
            }
            Err(err) => {
                eprintln!("\n[ERROR] modify_query : {}", err);
                return false;
            }
        }
    } else {
        modify_query_response = postgres_client
            .query_one(
                "UPDATE users SET tp = $1 AND sl = $2 WHERE userId = $3",
                &[&pg_tp, &pg_sl, &user_id],
            )
            .await;
        match modify_query_response {
            Ok(_) => {
                return true;
            }
            Err(err) => {
                eprintln!("\n[ERROR] modify_query : {}", err);
                return false;
            }
        }
    }
}

pub async fn close_order(
    postgres_client: &Client,
    user_id: &str,
    order_id: &str,
    close_price: &u64,
    close_type: &str,
) -> bool {
    let pg_close_price = *close_price as i64;

    let close_query_result = postgres_client
        .query_one(
            "UPDATE orders SET close = &1 AND closeType = $2 WHERE userId =$3 AND orderId = $4",
            &[&pg_close_price, &close_type, &user_id],
        )
        .await;

    match close_query_result {
        Ok(_) => {
            return true;
        }
        Err(err) => {
            eprintln!("\n[ERROR] close_order : {}", err);
            return false;
        }
    }
}

pub async fn update_balance(
    postgres_client: &Client,
    order_id: &str,
    user_id: &str,
    close_price: &u64,
) -> bool {
    let pg_close_price = *close_price as i64;

    let order_result = postgres_client
        .query_opt(
            "SELECT open, side, leverage  FROM orders WHERE orderId = $1 AND userId = $2",
            &[&order_id, &user_id],
        )
        .await;

    match order_result {
        Ok(Some(row)) => {
            let open_price_found: i64 = row.get(0);
            let side_found: u32 = row.get(1);
            let leverage_found: u32 = row.get(3);

            let is_profit: bool;
            let sum: i64;

            if side_found == 0 {
                // sell side
                let diff = open_price_found - pg_close_price;
                if diff > 0 {
                    is_profit = true
                } else {
                    is_profit = false
                }
                sum =  (diff.abs()) * leverage_found as i64 
            } else {
                // buy side
                let diff = pg_close_price - open_price_found;
                if diff > 0 {
                    is_profit = true
                } else {
                    is_profit = false
                }
                sum =  (diff.abs()) * leverage_found as i64 
            }

            if is_profit {
                let balance_update_query_response = postgres_client
                .query_one(
                    "UPDATE users SET balance = balance + $1 WHERE userId = $2",
                    &[&sum, &user_id],
                )
                .await;
                match balance_update_query_response {
                    Ok(_) => {
                        return true;
                    }
                    Err(err) => {
                        eprintln!("\n[ERROR] modify_query : {}", err);
                        return false;
                    }
                }
            } else {
                let balance_update_query_response = postgres_client
                .query_one(
                    "UPDATE users SET balance = balance - $1 WHERE userId = $2 AND balance >= $3",
                    &[&sum, &user_id, &sum],
                )
                .await;
                match balance_update_query_response {
                    Ok(_) => {
                        return true;
                    }
                    Err(err) => {
                        eprintln!("\n[ERROR] modify_query : {}", err);
                        return false;
                    }
                }
            }
        }
        Ok(None) => return false,
        Err(err) => {
            eprintln!("\n[ERROR] isUserExist err: {}", err.to_string());
            return false;
        }
    }
}

