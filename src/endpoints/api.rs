use crate::endpoints::api_helper::validate_sign_up;
use crate::establish_connection;
use crate::models::post::PostForm;
use crate::models::thread::ThreadForm;
use crate::models::user::{User, UserForm, UserLogin};
use crate::queries::{
    insert_post, insert_thread, select_post, select_posts, select_threads, select_user,
    select_users,
};
use actix_session::Session;
use actix_web::http::header::ContentType;
use actix_web::{options, web, HttpResponse};

use super::api_helper::validate_sign_in;
use log::info;

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

pub async fn get_posts_by_answer_id(answer_id: web::Path<i32>) -> HttpResponse {
    if answer_id.is_negative() {
        return HttpResponse::NotFound().body("Resource not found!");
    }
    HttpResponse::Ok().content_type(ContentType::json()).body(
        serde_json::to_string(&select_posts::get_posts_by_answer_id(answer_id.clone())).unwrap(),
    )
}

pub async fn get_posts_by_thread_id(thread_id: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::json()).body(
        serde_json::to_string(&select_posts::get_posts_by_answer_id(thread_id.clone())).unwrap(),
    )
}

pub async fn get_threads() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&select_threads::get_threads()).unwrap())
}

pub async fn get_threads_by_id(id: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&select_threads::get_threads_by_id(id.clone())).unwrap())
}

//User
pub async fn create_thread(session: Session, thread_form: web::Json<ThreadForm>) -> HttpResponse {
    let client_user = &session.get::<User>("clientUser");

    let mut client_user_check = User::default();
    match client_user {
        Ok(v) => match v {
            Some(t) => client_user_check = t.clone(),
            None => info!("NO VALUE CONTAINED!"),
        },
        Err(e) => info!("{}", e),
    }

    info!("USERNAME!: {}", client_user_check.email);

    if client_user_check.username.ne(&thread_form.author) {
        return HttpResponse::Unauthorized().body("User is not authorized!");
    }

    if thread_form.author.eq("") || thread_form.title.eq("") || thread_form.text.eq("") {
        return HttpResponse::BadRequest().body("All fields must be filled!");
    }
    let conn = &mut establish_connection();

    let inserted_thread = insert_thread::create_thread(
        conn,
        thread_form.author.clone(),
        thread_form.title.clone(),
        thread_form.text.clone(),
        thread_form.categories.clone(),
    );
    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&inserted_thread).unwrap())
}

pub async fn create_post(session: Session, post_form: web::Json<PostForm>) -> HttpResponse {
    if let Some(user_id) = session
        .get::<i32>("userId")
        .expect("get session userId error!")
    {
        info!("user_id: {user_id} is set!");
    } else {
        return HttpResponse::Unauthorized().body("This User is not authorized!");
    }

    if let Some(session_username) = session
        .get::<String>("username")
        .expect("get session username error")
    {
        info!("username: {session_username} is set!");
        if !session_username.eq(&post_form.author) {
            return HttpResponse::Unauthorized().body("This User is not authorized!");
        }
    } else {
        return HttpResponse::Unauthorized().body("This User is not authorized!");
    }

    if post_form.author.eq("") || post_form.title.eq("") || post_form.text.eq("") {
        return HttpResponse::BadRequest().body("All fields must be filled!");
    }
    let conn = &mut establish_connection();

    let inserted_post = insert_post::create_post(
        conn,
        post_form.thread_id,
        post_form.author.as_str(),
        post_form.title.as_str(),
        post_form.text.as_str(),
    );
    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&inserted_post).unwrap())
}

#[options("/threads")]
pub async fn threads_methods() -> HttpResponse {
    HttpResponse::NoContent()
        .append_header(("Allow", "GET, HEAD, POST"))
        .finish()
}
