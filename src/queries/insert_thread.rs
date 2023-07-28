use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

use crate::models::thread::{Thread, ThreadForm};

pub fn create_thread(
    conn: &mut PgConnection,
    author: String,
    title: String,
    text: String,
    categories: Vec<String>,
) -> Thread {
    use crate::schema::threads;

    let new_thread = ThreadForm {
        author,
        title,
        text,
        categories,
    };

    diesel::insert_into(threads::table)
        .values(&new_thread)
        .returning(Thread::as_returning())
        .get_result(conn)
        .expect("Error saving new user!")
}
