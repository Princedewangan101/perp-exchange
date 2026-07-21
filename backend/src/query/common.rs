pub fn log_db_error(context: &str, err: &tokio_postgres::Error) {
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
