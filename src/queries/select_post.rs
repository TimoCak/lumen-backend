use diesel::prelude::*;
use crate::{*, models::post::Post};

pub fn get_post_by_post_id(filter_post_id: i32) -> Vec<Post> {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();

    let results: Vec<Post> = posts
        .filter(id.eq(filter_post_id))
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading user!");

    results
}