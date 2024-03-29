use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub(crate) struct Post {
    pub id: i32,
    pub thread_id: i32,
    pub answer_id: Option<i32>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct PostUpdate {
    pub title: String,
    pub text: String,
    pub likes: i32,
    pub dislikes: i32,
}
