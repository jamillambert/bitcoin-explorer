mod api;
mod db;

fn main() {
    db::create_database().unwrap();
    db::create_tables().unwrap();
    println!("database and tables created");
}
