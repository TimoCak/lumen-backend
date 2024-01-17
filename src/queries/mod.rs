use diesel::PgConnection;

pub mod insert_thread;
pub mod post_query;
pub mod user_query;
pub mod select_threads;
pub mod thread_query;

pub trait DbQuery {
    fn connection(&self) -> PgConnection;
}