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

    pub fn update_thread(&mut self, filter_thread_id: i32, data: &Thread) -> Thread {
        diesel::update(threads.find(filter_thread_id))
           .set((
                threads::title.eq(data.title.to_owned()),
                threads::text.eq(data.text.to_owned()),
                threads::categories.eq(data.categories.to_owned()),
                threads::likes.eq(data.likes.to_owned()),
                threads::dislikes.eq(data.dislikes.to_owned()),
           ))
           .returning(Thread::as_returning())
           .get_result(&mut self.connection())
           .expect("Error updating post")
    }

    pub fn delete_thread(&mut self, filter_thread_id: i32) -> Thread {
        diesel::delete(threads.filter(id.eq(filter_thread_id)))
        .returning(Thread::as_returning())
        .get_result(&mut self.connection())
        .expect("Error deleting posts")
    }
}