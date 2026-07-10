use tokio_postgres::{Client};

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
            "UPDATE users SET balance = balance - $1 WHERE userId = &2 AND balance >= &3",
            &[&amount, &user_id, &amount],
        )
        .await;

    match withdraw_query_result {
        Ok(row) => {
            return true;
        }
        Err(err) => {
            eprintln!("\n[ERROR] withdraw_balance : {}", err);
            return false;
        }
    }
}

// postgres_client.execute("INSERT INTO users (name, email, password) VALUES ($1, $2, $3)", &[&])

// // --- Start Transaction for atomic CRUD Operations ---
// let transaction = client.transaction().await?;
// println!("\nTransaction started.");

// // CREATE: Insert a new todo item
// transaction.execute("INSERT INTO todos (task) VALUES ($1)", &[&"Learn Neon with Rust"]).await?;
// println!("CREATE: Row inserted.");

// // READ: Retrieve the new todo item
// let row = transaction.query_one("SELECT task FROM todos WHERE task = $1", &[&"Learn Neon with Rust"]).await?;
// let task: &str = row.get(0);
// println!("READ: Fetched task - '{}'", task);

// // UPDATE: Modify the todo item
// transaction.execute("UPDATE todos SET task = $1 WHERE task = $2", &[&"Master Neon with Rust!", &"Learn Neon with Rust"]).await?;
// println!("UPDATE: Row updated.");

// // DELETE: Remove the todo item
// transaction.execute("DELETE FROM todos WHERE task = $1", &[&"Master Neon with Rust!"]).await?;
// println!("DELETE: Row deleted.");

// --- Commit Transaction ---
// transaction.commit().await?;
// println!("Transaction committed successfully.\n");
