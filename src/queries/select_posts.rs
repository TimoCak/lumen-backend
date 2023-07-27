use self::schema::posts::dsl::*;
use crate::{models::post::Post, *};
use diesel::prelude::*;
/*
select all users for validation if the username is unique!
*/
pub fn get_posts() -> Vec<Post> {
    let conn = &mut establish_connection();

    let results = posts
        .select(Post::as_select())
        .load(conn)
        .expect("Error loading posts");

    results
}

pub fn get_posts_by_answer_id(filter_answer_id: i32) -> Vec<Post> {
    let conn = &mut establish_connection();

    let results = posts
        .filter(id.eq(filter_answer_id))
        .select(Post::as_select())
        .load(conn)
        .expect("error loading posts!");

    results
}
