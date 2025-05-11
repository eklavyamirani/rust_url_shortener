use std::env;

fn main() {
    dotenvy::from_path("env_setup/.env").ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database at {}", db_url);
}
