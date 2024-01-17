use diesel::PgConnection;

pub mod insert_thread;
pub mod insert_user;
pub mod post_query;
pub mod user_query;
pub mod select_threads;
pub mod select_user;
pub mod select_users;

pub trait DbQuery {
    fn connection(&self) -> PgConnection;
}