
use diesel::prelude::*;
use crate::establish_connection;
use crate::models::post::{Post, PostForm, PostUpdate};
use crate::schema::posts;
use crate::schema::posts::dsl::*;

use super::DbQuery;

pub struct PostQuery;

impl DbQuery for PostQuery {
    fn connection(&self) -> PgConnection {
        establish_connection()
    }
}

impl PostQuery {

    pub fn create_post(&mut self, data: &PostForm) -> Post {
        diesel::insert_into(posts::table)
        .values(data)
        .returning(Post::as_returning())
        .get_result(&mut self.connection())
        .expect("Error saving new post!")
    }

    pub fn get_post(&mut self, filter_post_id: i32) -> Vec<Post>{
        let results: Vec<Post> = posts
            .filter(id.eq(filter_post_id))
            .select(Post::as_select())
            .load(&mut self.connection())
            .expect("Error loading post!");
    
        results
    }

    pub fn get_posts(&mut self) -> Vec<Post> {
        let results = posts
            .select(Post::as_select())
            .load(&mut self.connection())
            .expect("Error loading posts");
    
        results
    }

    pub fn get_posts_by_answer_id(&mut self, filter_answer_id: i32) -> Vec<Post> {
        let results = posts
            .filter(id.eq(filter_answer_id))
            .select(Post::as_select())
            .load(&mut self.connection())
            .expect("error loading posts!");

        results
    }

    pub fn get_posts_by_thread_id(&mut self, filter_thread_id: i32) -> Vec<Post> {
        let results: Vec<Post> = posts
            .filter(thread_id.eq(filter_thread_id))
            .select(Post::as_select())
            .load(&mut self.connection())
            .expect("Error loading user!");
    
        results
    }
    
    pub fn update_post(&mut self, filter_post_id: i32, data: &PostUpdate) -> Post {
        diesel::update(posts.find(filter_post_id))
           .set((
                posts::title.eq(data.title.to_owned()),
                posts::text.eq(data.text.to_owned()),
                posts::likes.eq(data.likes.to_owned()),
                posts::dislikes.eq(data.dislikes.to_owned())
           ))
           .returning(Post::as_returning())
           .get_result(&mut self.connection())
           .expect("Error updating post")
    }

    pub fn delete_post(&mut self, filter_post_id: i32) -> Post {
        diesel::delete(posts.filter(id.eq(filter_post_id)))
            .returning(Post::as_returning())
            .get_result(&mut self.connection())
            .expect("Error deleting posts")
    }
}