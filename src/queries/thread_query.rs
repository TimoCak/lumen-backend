use diesel::prelude::*;
use crate::schema::threads::dsl::*;
use crate::schema::threads;
use crate::{models::thread::Thread, *};
use crate::establish_connection;

use self::models::thread::ThreadForm;

use super::DbQuery;

pub struct ThreadQuery;

impl DbQuery for ThreadQuery {
    fn connection(&self) -> PgConnection {
        establish_connection()
    }
}

impl ThreadQuery {
    pub fn get_threads(&mut self) -> Vec<Thread> {    
        let results = threads
            .select(Thread::as_select())
            .load(&mut self.connection())
            .expect("error loading threads!");
    
        results
    }

    pub fn get_threads_by_id(&mut self, filter_thread_id: i32) -> Vec<Thread> {
        let results = threads
            .filter(id.eq(filter_thread_id))
            .select(Thread::as_select())
            .load(&mut self.connection())
            .expect("error loading threads!");
    
        results
    }

    pub fn create_thread(&mut self, new_thread: &ThreadForm) -> Thread {
        diesel::insert_into(threads::table)
            .values(new_thread)
            .returning(Thread::as_returning())
            .get_result(&mut self.connection())
            .expect("Error saving new user!")
    }

    pub fn update_thread() {
        todo!()
    }

    pub fn delete_thread() {
        todo!()
    }
}