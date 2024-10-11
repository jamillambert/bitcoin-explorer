mod api;
mod db;

fn main() {
    if let Err(e) = db::create_database() {
        eprintln!("Failed to create database: {}", e);
    } else {
        if let Err(e) = db::create_tables() {
            eprintln!("Failed to create tables: {}", e);
        } else {
            println!("database and tables created");
        }
    }
}
