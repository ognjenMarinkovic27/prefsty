use prefsty::persistence::PgDB;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("Provide a DATABASE_URL to run prefsty");

    let pool = PgPool::connect(&db_url).await?;
    let db = PgDB::new(pool);

    Ok(())
}
