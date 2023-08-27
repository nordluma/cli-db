use sqlx::{migrate::MigrateDatabase, Connection, FromRow, Sqlite, SqliteConnection};

use crate::args::Entry;

const DB_URL: &str = "sqlite://cli.db";

#[derive(FromRow)]
pub struct DbEntry {
    pub id: String,
    pub value: String,
}

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
                value TEXT NOT NULL);",
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

    pub async fn get_all(&mut self) -> sqlx::Result<Vec<DbEntry>> {
        match sqlx::query_as::<_, DbEntry>("SELECT * FROM things")
            .fetch_all(&mut self.conn)
            .await
        {
            Ok(entries) => Ok(entries),
            Err(e) => Err(e),
        }
    }

    pub async fn get_entry(&mut self, key: &str) -> sqlx::Result<Option<DbEntry>> {
        match sqlx::query_as::<_, DbEntry>(
            "SELECT * FROM things
            WHERE id = $1",
        )
        .bind(key)
        .fetch_optional(&mut self.conn)
        .await
        {
            Ok(entry) => Ok(entry),
            Err(e) => Err(e),
        }
    }

    pub async fn delete(&mut self, key: &str) -> sqlx::Result<()> {
        match sqlx::query("DELETE FROM things WHERE id = $1")
            .bind(key)
            .execute(&mut self.conn)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
