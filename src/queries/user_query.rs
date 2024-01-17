use diesel::prelude::*;
use crate::models::user::NewUser;
use crate::{establish_connection, models::user::User};
use crate::schema::users::dsl::*;
use crate::schema::users;

use super::DbQuery;

pub struct UserQuery;

impl DbQuery for UserQuery {
    fn connection(&self) -> PgConnection {
        establish_connection()
    }
}

impl UserQuery {
    pub fn get_user_by_username(&mut self, filter_username: &String) -> Vec<User> {    
        let results: Vec<User> = users
            .filter(username.eq(filter_username))
            .select(User::as_select())
            .load(&mut self.connection())
            .expect("Error loading user!");
    
        results
    }

    pub fn get_user_by_user_id(&mut self, filter_user_id: i32) -> Vec<User> {
        let results: Vec<User> = users
            .filter(id.eq(filter_user_id))
            .select(User::as_select())
            .load(&mut self.connection())
            .expect("Error loading user!");
    
        results
    }

    pub fn get_users(&mut self) -> Vec<User> {
        let results = users
            .select(User::as_select())
            .load(&mut self.connection())
            .expect("Error loading users");
    
        results
    }

    pub fn create_user(&mut self, new_user: &NewUser) -> User {
        diesel::insert_into(users::table)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(&mut self.connection())
            .expect("Error saving new user!")
    }
}