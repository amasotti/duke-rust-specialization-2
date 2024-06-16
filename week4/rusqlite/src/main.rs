use rusqlite::db;

fn main() {
    if let Err(e) = db::run() {
        eprintln!("Error: {}", e);
    }
}