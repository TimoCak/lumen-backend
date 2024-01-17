use crate::endpoints::api_helper::{compare_users, validate_sign_up};
use crate::establish_connection;
use crate::models::post::PostForm;
use crate::models::thread::ThreadForm;
use crate::models::user::{ClientStoredUser, UserForm, UserLogin};
use crate::queries::select_user::get_user_by_username;
use crate::queries::{ insert_thread, post_query, select_threads, select_user,
    select_users,
};
use actix_session::Session;
use actix_web::http::header::{ContentType, Header};
use actix_web::{options, web, HttpRequest, HttpResponse};
use actix_web_httpauth::headers::authorization::{Authorization, Basic};
use log::info;

//Guest
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("lumen-backend is running!")
}

pub async fn sign_up(user_form: web::Json<UserForm>) -> HttpResponse {
    validate_sign_up(user_form)
}

pub async fn sign_in(session: Session, user_login: web::Json<UserLogin>) -> HttpResponse {
    if user_login.username.eq("") || user_login.password.eq("") {
        return HttpResponse::BadRequest().body("please fill out all fields!");
    }

    let mut found = false;

    let mut client_stored_user = ClientStoredUser::default();

    for user in get_user_by_username(&user_login.username) {
        if compare_users(
            &user_login.username,
            &user_login.password,
            &user.username,
            &user.password,
        ) {
            found = true;

            client_stored_user = ClientStoredUser {
                id: user.id,
                username: user.username,
                email: user.email,
                password: user.password,
            };

            break;
        }
    }

    if found {
        info!("\nUSER: {:?}", client_stored_user.clone());
        match session.insert("client_user", client_stored_user.clone()) {
            Ok(_) => info!("INSERTION SUCCESSED! {:?}", session.entries()),
            Err(e) => info!("INSERTION FAILED! {}", e),
        };
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&client_stored_user).unwrap());
    }
    HttpResponse::Unauthorized().body("username or password is wrong!")
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
    let list = &post_query::PostQuery.get_post(path.clone());

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
        .body(serde_json::to_string(&post_query::PostQuery.get_posts()).unwrap())
}

pub async fn get_posts_by_answer_id(answer_id: web::Path<i32>) -> HttpResponse {
    if answer_id.is_negative() {
        return HttpResponse::NotFound().body("Resource not found!");
    }

    let posts = serde_json::to_string(&post_query::PostQuery.get_posts_by_answer_id(answer_id.clone())).unwrap();
    HttpResponse::Ok().content_type(ContentType::json()).body(posts)
}

pub async fn get_posts_by_thread_id(thread_id: web::Path<i32>) -> HttpResponse {
    let posts = serde_json::to_string(&post_query::PostQuery.get_posts_by_thread_id(thread_id.clone())).unwrap();
    HttpResponse::Ok().content_type(ContentType::json()).body(posts)
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


pub async fn create_thread(req: HttpRequest, thread_form: web::Json<ThreadForm>) -> HttpResponse {
    let auth = Authorization::<Basic>::parse(&req).expect("parsed basic auth credentials");
    let user = get_user_by_username(&auth.as_ref().user_id().to_string());

    let username_db = &user.get(0).unwrap().username;
    let password_db = &user.get(0).unwrap().password;

    if auth.as_ref().password().unwrap().ne(password_db) || auth.as_ref().user_id().ne(username_db)
    {
        return HttpResponse::Unauthorized().body("username or password is invalid!");
    }

    if username_db.ne(&thread_form.author) {
        return HttpResponse::Unauthorized().body("user does not match the author!");
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

pub async fn create_post(req: HttpRequest, post_form: web::Json<PostForm>) -> HttpResponse {
    let auth = Authorization::<Basic>::parse(&req).expect("parsed basic auth credentials");
    let user = get_user_by_username(&auth.as_ref().user_id().to_string());

    let username_db = &user.get(0).unwrap().username;
    let password_db = &user.get(0).unwrap().password;

    if auth.as_ref().password().unwrap().ne(password_db) || auth.as_ref().user_id().ne(username_db)
    {
        return HttpResponse::Unauthorized().body("username or password is invalid!");
    }

    if username_db.ne(&post_form.author) {
        return HttpResponse::Unauthorized().body("user does not match the author!");
    }

    if post_form.author.eq("") || post_form.title.eq("") || post_form.text.eq("") {
        return HttpResponse::BadRequest().body("All fields must be filled!");
    }

    let inserted_post = post_query::PostQuery.create_post(&PostForm {
        thread_id: post_form.thread_id,
        author: post_form.author.clone(),
        title: post_form.title.clone(),
        text: post_form.text.clone(),
    });

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
