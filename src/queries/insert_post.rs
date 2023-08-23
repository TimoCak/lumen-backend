use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

use crate::models::post::{Post, PostForm};

pub fn create_post(
    conn: &mut PgConnection,
    thread_id: i32,
    author: &str,
    title: &str,
    text: &str,
) -> Post {
    use crate::schema::posts;

    let new_post = PostForm {
        thread_id: thread_id,
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
