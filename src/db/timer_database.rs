use crate::core::models::{NewTimerRun, TimerRuns};
use chrono::prelude::*;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    conn.run_pending_migrations(MIGRATIONS)
        .expect("Could not run migrations");

    conn
}

pub fn create_timer_run(
    conn: &mut SqliteConnection,
    user: &str,
    working_time_secs: &i32,
    breaking_time_secs: &i32,
) {
    use crate::core::schema::timer_runs;

    let local: NaiveDate = Local::now().date_naive();

    let new_run = NewTimerRun {
        user,
        working_time_secs,
        date: &local,
        breaking_time_secs,
    };

    diesel::insert_into(timer_runs::table)
        .values(&new_run)
        .execute(conn)
        .expect("Error saving new timer run");
}

pub fn get_timer_runs(conn: &mut SqliteConnection, username: &str) -> Vec<TimerRuns> {
    use crate::core::schema::timer_runs::dsl::*;

    let results = timer_runs
        .filter(user.eq(username))
        .select(TimerRuns::as_select())
        .load(conn)
        .expect("Error loading timer runs");

    results
}

pub fn get_users(conn: &mut SqliteConnection) -> Vec<String> {
    use crate::core::schema::timer_runs::dsl::*;
    timer_runs
        .select(user)
        .distinct()
        .load(conn)
        .expect("Error loading timer runs")
}
