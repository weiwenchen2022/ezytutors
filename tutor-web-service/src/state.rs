use sqlx::postgres::PgPool;

use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<i32>,
    pub pg_pool: PgPool,
}
