use sqlx::{migrate::MigrateDatabase, Connection, Sqlite, SqliteConnection};

const DB_URL: &str = "sqlite://cli.db";

pub struct Store {
    conn: SqliteConnection,
}

impl Store {
    pub async fn init() -> Result<Self, sqlx::Error> {
        Store::create_db_if_not_exist().await;
        let conn = SqliteConnection::connect(DB_URL).await?;

        Ok(Store { conn })
    }

    async fn create_db_if_not_exist() {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            eprintln!("Creating database: {}", DB_URL);

            match Sqlite::create_database(DB_URL).await {
                Ok(_) => eprintln!("Database created"),
                Err(e) => panic!("Error: {}", e),
            }
        }
    }
}
