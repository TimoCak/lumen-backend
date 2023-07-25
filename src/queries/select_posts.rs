use diesel::prelude::*;
use crate::{*, models::post::Post};

/*
select all users for validation if the username is unique!
*/
pub fn get_posts() -> Vec<Post> {
    use self::schema::posts::dsl::*;

    let conn = &mut establish_connection();

    let results = posts
        .select(Post::as_select())
        .load(conn)
        .expect("Error loading posts");

    results
}