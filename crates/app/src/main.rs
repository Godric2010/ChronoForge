fn main() {

    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    println!("DB: {}", db_url);

    println!("Hello, world!");
}
