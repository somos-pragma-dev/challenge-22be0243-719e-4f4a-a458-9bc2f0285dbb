use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn run_migrations(conn: &PgConnection) {
    embedded_migrations::run(conn).expect("Failed to run migrations");
}