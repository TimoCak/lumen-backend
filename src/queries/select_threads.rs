use crate::{models::thread::Thread, *};
use diesel::prelude::*;

pub fn get_threads() -> Vec<Thread> {
    use crate::schema::threads::dsl::*;

    let conn = &mut establish_connection();

    let results = threads
        .select(Thread::as_select())
        .load(conn)
        .expect("error loading threads!");

    results
}

pub fn get_threads_by_id(filter_thread_id: i32) -> Vec<Thread> {
    use crate::schema::threads::dsl::*;

    let conn = &mut establish_connection();

    let results = threads
        .filter(id.eq(filter_thread_id))
        .select(Thread::as_select())
        .load(conn)
        .expect("error loading threads!");

    results
}
