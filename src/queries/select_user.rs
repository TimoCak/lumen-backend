use diesel::prelude::*;
use crate::{*, models::user::User};

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