use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

use crate::models::user::{NewUser, Role, User};

pub fn create_user(conn: &mut PgConnection, username: &str, email: &str, password: &str) -> User {
    use crate::schema::users;

    let new_user = NewUser {
        username: username.to_string(),
        email: email.to_string(),
        password: password.to_string(),
        role: format!("{}", Role::User),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new user!")
}
