// @generated automatically by Diesel CLI.

diesel::table! {
    timer_runs (id) {
        id -> Integer,
        user -> Text,
        working_time_secs -> Integer,
        breaking_time_secs -> Integer,
        date -> Date,
    }
}
