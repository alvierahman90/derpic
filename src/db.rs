use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;
use std::error::Error;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DERPIC_DATABASE_URL").expect("DERPIC_DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to {}: {e}", database_url))
}

pub fn run_migrations(
    conn: &mut PgConnection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    conn.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
