use diesel::{SqliteConnection, Connection};
use dotenvy::dotenv;

pub async fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    
    let datatbase_url = ::std::env::var("DATABASE_URL").unwrap();
    SqliteConnection::establish(&datatbase_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", datatbase_url))
}