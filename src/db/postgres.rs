use super::types::OptionInfo;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub struct PostgresDb {
    pool: Pool<Postgres>,
}

impl PostgresDb {
    pub async fn new() -> anyhow::Result<Self> {
        let db_url = std::env::var("DATABASE_URL")?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url).await?;

        Ok(PostgresDb{
            pool,
        })
    }

    pub async fn test(&mut self) -> anyhow::Result<()> {
        sqlx::query_as!(OptionInfo,
                "
                INSERT INTO option_data (
                    symbol, timestamp, option_type, strike, expiration_date
                )
                VALUES (
                   'beans', '1999-01-08 04:05:06 -8:00', 'call', 10.2, '1999-01-08 04:05:06 -8:00'
                )
                ",
            ).fetch_all(&self.pool).await.unwrap();

        let result = sqlx::query_as!(
            OptionInfo,
            "
            SELECT * FROM option_data
            ",
        ).fetch_all(&self.pool).await.unwrap();

        for r in result {
            dbg!(r);
        }

        Ok(())
    }

    pub async fn add_option_info(&mut self, symbol: &str, data: Vec<OptionInfo>) {
        let symbol = symbol.to_uppercase();

        let result = sqlx::query_as!(
            OptionInfo,
            "
            SELECT * FROM option_data
            ",
        ).fetch_all(&self.pool).await.unwrap();

        for option in data {
            let option_data = sqlx::query_as!(OptionInfo,
                "
                INSERT INTO option_data (
                    symbol, timestamp, option_type, strike, expiration_date
                )
                VALUES (
                   'beans', '1999-01-08 04:05:06 -8:00', 'call', 10.2, '1999-01-08 04:05:06 -8:00'
                )
                ",
            ).fetch_all(&self.pool).await.unwrap();
        }

        // let entry = self.options.entry(symbol).or_insert_with(Vec::new);

        // entry.push(data);
        // if let Err(e) = self.write() {
        //     log::error!("{}", e);
        // }
    }
}


// db.env
// POSTGRES_USER=postgres
// POSTGRES_PASSWORD=postgres
//
// .env
// DATABASE_URL="postgresql://postgres:postgres@localhost:5438/postgres"
