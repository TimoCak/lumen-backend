use diesel::PgConnection;

pub mod post_query;
pub mod user_query;
pub mod thread_query;

pub(crate) trait DbQuery {
    fn connection(&self) -> PgConnection;
}