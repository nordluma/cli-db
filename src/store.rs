use sqlx::{migrate::MigrateDatabase, Connection, Sqlite, SqliteConnection};

use crate::args::Entry;

const DB_URL: &str = "sqlite://cli.db";

#[allow(unused)]
pub struct Store {
    conn: SqliteConnection,
}

impl Store {
    pub async fn init() -> Result<Self, sqlx::Error> {
        Store::create_db_if_not_exist().await;
        let mut conn = SqliteConnection::connect(DB_URL).await?;

        if let Err(e) = Store::create_table(&mut conn).await {
            panic!("Could not create table: {}", e)
        };

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

    async fn create_table(conn: &mut SqliteConnection) -> sqlx::Result<()> {
        match sqlx::query(
            "CREATE TABLE IF NOT EXISTS things (
                id TEXT PRIMARY KEY NOT NULL,
                value TEXT NOT NULL
                );",
        )
        .execute(conn)
        .await
        {
            Ok(res) => {
                dbg!("result: {:?}", res);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub async fn insert(&mut self, entry: Entry) -> sqlx::Result<bool> {
        match sqlx::query(
            "INSERT INTO things (id, value)
                          VALUES ($1, $2)
                          RETURNING *",
        )
        .bind(entry.key)
        .bind(entry.value)
        .fetch_one(&mut self.conn)
        .await
        {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    pub async fn get_all(&mut self) -> sqlx::Result<Vec<Entry>> {
        match sqlx::query_as::<_, Entry>("SELECT * FORM things")
            .fetch_all(&mut self.conn)
            .await
        {
            Ok(entries) => Ok(entries),
            Err(e) => Err(e),
        }
    }
}
