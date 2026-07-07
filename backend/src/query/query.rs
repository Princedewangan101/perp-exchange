use tokio_postgres::Client;

pub struct UserDetails {
    // pub name: String,
    pub email: String,
}
pub struct UserStatusResponse {
    pub is_user_exist: bool,
    pub details: Option<UserDetails>,
}

pub async fn is_user_exist(postgres_client: &Client, email: &str) -> UserStatusResponse {
    let search_query_result = postgres_client
        .query_one("SELECT name, email FROM users WHERE email = $1", &[&email])
        .await;

    match search_query_result {
        Ok(row) => {
            // let name_found:String = row.get(0);
            let email_found:String = row.get(1);
            UserStatusResponse {
                is_user_exist: true,
                details: Some(UserDetails {
                    // name: name_found, 
                    email: email_found
                }),
            }
        },
        Err(err) => {
            eprintln!("\n[ERROR] isUserExist err: {}", err.to_string());
            if err.to_string().contains("0 rows were returned") {
                UserStatusResponse {
                    is_user_exist: false,
                    details: None,
                }
            }else{
                eprintln!("Database system error: {}", err);
                UserStatusResponse {
                    is_user_exist: false,
                    details: None,
                }
            }
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
