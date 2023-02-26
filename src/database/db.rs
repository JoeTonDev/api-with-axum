use sea_orm::entity::prelude::*;
use sea_orm::Database;
use tokio::sync::OnceCell;

async fn init_conn() -> DatabaseConnection {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    Database::connect(db_url)
        .await
        .expect("Database connection failed")
}

static CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_conn() -> &'static DatabaseConnection {
    CONN.get_or_init(init_conn).await
}
