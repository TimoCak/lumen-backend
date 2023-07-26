use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub thread_id: i32,
    pub author: String,
    pub created_at: Option<SystemTime>,
    pub title: String,
    pub text: String,
    pub likes: Option<i32>,
    pub dislikes: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct PostForm {
    pub thread_id: i32,
    pub author: String,
    pub title: String,
    pub text: String,
}