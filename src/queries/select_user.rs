use crate::{models::user::User, *};
use diesel::prelude::*;

pub fn get_user_by_username(filter_username: &String) -> Vec<User> {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let results: Vec<User> = users
        .filter(username.eq(filter_username))
        .select(User::as_select())
        .load(connection)
        .expect("Error loading user!");

    results
}

pub fn get_user_by_user_id(filter_user_id: i32) -> Vec<User> {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let results: Vec<User> = users
        .filter(id.eq(filter_user_id))
        .select(User::as_select())
        .load(connection)
        .expect("Error loading user!");

    results
}
