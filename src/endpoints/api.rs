use crate::endpoints::api_helper::validate_sign_up;
use crate::establish_connection;
use crate::models::post::PostForm;
use crate::models::user::{UserForm, UserLogin};
use crate::queries::{insert_post, select_posts, select_user, select_users, select_post};
use actix_session::Session;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};

use super::api_helper::validate_sign_in;

//Guest
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("lumen-backend is running!")
}

pub async fn sign_up(user_form: web::Json<UserForm>) -> HttpResponse {
    validate_sign_up(user_form)
}

pub async fn sign_in(session: Session, user_login: web::Json<UserLogin>) -> HttpResponse {
    validate_sign_in(session.clone(), &user_login.username, &user_login.password)
}

pub async fn sign_out(session: Session) -> HttpResponse {
    session.purge();
    HttpResponse::Ok().body("succesfully logged out!")
}

pub async fn get_user_by_id(path: web::Path<i32>) -> HttpResponse {
    let list = &select_user::get_user_by_user_id(path.clone());

    if list.len() == 0 {
        return HttpResponse::NotFound().body("user does not exist!");
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(list).unwrap())
}

pub async fn get_user(path: web::Path<String>) -> HttpResponse {
    let list = &select_user::get_user_by_username(&path);

    if list.len() == 0 {
        return HttpResponse::NotFound().body("user does not exist!");
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(list).unwrap())
}

pub async fn get_users() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&select_users::get_users()).unwrap())
}

pub async fn get_post_by_id(path: web::Path<i32>) -> HttpResponse {

    let list = &select_post::get_post_by_post_id(path.clone());

    if list.len() == 0 {
        return HttpResponse::NotFound().body("post does not exist!");
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(list).unwrap())

}

pub async fn get_posts() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&select_posts::get_posts()).unwrap())
}

//User
pub async fn create_post(session: Session, post_form: web::Json<PostForm>) -> HttpResponse {
    if let Some(user_id) = session.get::<i32>("userId").expect("session getter error!") {
        println!("user: {} is authorized!", user_id);
    } else {
        return HttpResponse::Unauthorized().body("This User is not authorized!");
    }

    if post_form.author.eq("") || post_form.title.eq("") || post_form.text.eq("") {
        return HttpResponse::BadRequest().body("All fields must be filled!");
    }
    let conn = &mut establish_connection();

    let inserted_post = insert_post::create_post(
        conn,
        post_form.author.as_str(),
        post_form.title.as_str(),
        post_form.text.as_str(),
    );
    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&inserted_post).unwrap())
}
