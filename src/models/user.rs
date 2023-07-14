use diesel::{Queryable, Selectable, Insertable};
use serde::{Serialize, Deserialize};
use std::fmt;


#[derive(Debug)]
pub enum Role {
    Guest,
    User,
    Moderator,
    Admin,
    Owner,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Deserialize, Serialize)]
pub struct UserForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Selectable, Serialize)]
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