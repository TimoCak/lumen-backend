use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

use crate::models::post::{Post, PostForm};

pub fn create_post(conn: &mut PgConnection, author: &str, title: &str, text: &str) -> Post {
    use crate::schema::posts;

    let new_post = PostForm {
        author: author.to_string(),
        title: title.to_string(),
        text: text.to_string(),
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post!")
}
