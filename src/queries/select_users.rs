use crate::{models::user::User, *};
use diesel::prelude::*;

/*
select all users for validation if the username is unique!
*/
pub fn get_users() -> Vec<User> {
    use self::schema::users::dsl::*;

    let conn = &mut establish_connection();

    let results = users
        .select(User::as_select())
        .load(conn)
        .expect("Error loading users");

    results
}
