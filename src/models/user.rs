use diesel::{Insertable, Queryable, Selectable};
use serde::{Serialize, Deserialize};


#[derive(Debug)]
pub enum Role {
    Guest,
    User,
    Moderator,
    Admin,
    Owner,
}


#[derive(Deserialize, Serialize)]
pub struct UserForm {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}