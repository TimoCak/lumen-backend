use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub(crate) enum Role {
    User,
    Moderator,
    Admin,
    Developer,
    Owner,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone, Default)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Default, Clone, fmt::Debug)]
pub struct ClientStoredUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
}
