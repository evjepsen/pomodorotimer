use crate::core::schema::timer_runs;
use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = timer_runs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TimerRuns {
    pub id: i32,
    pub user: String,
    pub working_time_secs: i32,
    pub breaking_time_secs: i32,
    pub date: NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = timer_runs)]
pub struct NewTimerRun<'a> {
    pub user: &'a str,
    pub working_time_secs: &'a i32,
    pub breaking_time_secs: &'a i32,
    pub date: &'a NaiveDate,
}
