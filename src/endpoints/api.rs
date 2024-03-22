use crate::endpoints::api_helper::{compare_users, validate_sign_up};
use crate::models::post::{PostForm, PostUpdate};
use crate::models::thread::{ThreadForm, ThreadUpdate};
use crate::models::user::{ClientStoredUser, UserForm, UserLogin};
use crate::queries::{post_query, thread_query, user_query};
use actix_session::Session;
use actix_web::http::header::ContentType;
use actix_web::{options, web, HttpRequest, HttpResponse, Responder};
use log::info;

use super::api_helper::check_auth;

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

    for user in user_query::UserQuery.get_user_by_username(&user_login.username) {
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
                role: user.role,
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
    let list = &user_query::UserQuery.get_user_by_user_id(path.clone());

    if list.len() == 0 {
        return HttpResponse::NotFound().body("user does not exist!");
    }

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(list).unwrap())
}

pub async fn get_user(path: web::Path<String>) -> HttpResponse {
    let list = &user_query::UserQuery.get_user_by_username(&path);

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
        .body(serde_json::to_string(&user_query::UserQuery.get_users()).unwrap())
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

    let posts =
        serde_json::to_string(&post_query::PostQuery.get_posts_by_answer_id(answer_id.clone()))
            .unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(posts)
}

pub async fn get_posts_by_thread_id(thread_id: web::Path<i32>) -> HttpResponse {
    let posts =
        serde_json::to_string(&post_query::PostQuery.get_posts_by_thread_id(thread_id.clone()))
            .unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(posts)
}

pub async fn get_threads() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&thread_query::ThreadQuery.get_threads()).unwrap())
}

pub async fn get_thread_by_id(id: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().content_type(ContentType::json()).body(
        serde_json::to_string(&thread_query::ThreadQuery.get_threads_by_id(id.clone())).unwrap(),
    )
}

pub async fn create_thread(req: HttpRequest, thread_form: web::Json<ThreadForm>) -> HttpResponse {
    let auth_response = check_auth(&req);

    if thread_form.author.eq("") || thread_form.title.eq("") || thread_form.text.eq("") {
        return HttpResponse::BadRequest().body("All fields must be filled!");
    } else if auth_response.status().is_client_error() {
        return auth_response;
    }

    let inserted_thread = thread_query::ThreadQuery.create_thread(&ThreadForm {
        author: thread_form.author.clone(),
        title: thread_form.title.clone(),
        text: thread_form.text.clone(),
        categories: thread_form.categories.clone(),
    });
    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&inserted_thread).unwrap())
}

pub async fn create_post(req: HttpRequest, post_form: web::Json<PostForm>) -> HttpResponse {
    let auth_response = check_auth(&req);

    if post_form.author.eq("") || post_form.title.eq("") || post_form.text.eq("") {
        return HttpResponse::BadRequest().body("All fields must be filled!");
    } else if auth_response.status().is_client_error() {
        return auth_response;
    }

    let inserted_post = post_query::PostQuery.create_post(&PostForm {
        thread_id: post_form.thread_id,
        author: post_form.author.clone(),
        title: post_form.title.clone(),
        text: post_form.text.clone(),
    });

    return HttpResponse::Created()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&inserted_post).unwrap());
}

pub async fn update_post(
    req: HttpRequest,
    post_update: web::Json<PostUpdate>,
    id: web::Path<i32>,
) -> impl Responder {
    let auth_response = check_auth(&req);

    if post_update.title.eq("") || post_update.text.eq("") {
        return HttpResponse::BadRequest().body("");
    } else if auth_response.status().is_client_error() {
        return auth_response;
    }

    let updated_post = post_query::PostQuery.update_post(*id, &post_update);

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&updated_post).unwrap());
}

pub async fn delete_post(req: HttpRequest, id: web::Path<i32>) -> impl Responder {
    let auth_response = check_auth(&req);

    if auth_response.status().is_client_error() {
        return auth_response;
    }

    let deleted_post = post_query::PostQuery.delete_post(*id);

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&deleted_post).unwrap());
}

pub async fn update_thread(
    req: HttpRequest,
    thread_update: web::Json<ThreadUpdate>,
    id: web::Path<i32>,
) -> impl Responder {
    let auth_response = check_auth(&req);

    if thread_update.title.eq("") || thread_update.text.eq("") {
        return HttpResponse::BadRequest().body("");
    } else if auth_response.status().is_client_error() {
        return auth_response;
    }
    let updated_thread = thread_query::ThreadQuery.update_thread(*id, &thread_update);

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&updated_thread).unwrap());
}

pub async fn delete_thread(req: HttpRequest, id: web::Path<i32>) -> impl Responder {
    let auth_response = check_auth(&req);

    if auth_response.status().is_client_error() {
        return auth_response;
    }

    //delete target thread
    let deleted_thread = thread_query::ThreadQuery.delete_thread(*id);

    //delete all posts associated with that thread
    let post_list = post_query::PostQuery.get_posts();
    for post in post_list {
        if deleted_thread.id == post.thread_id {
            post_query::PostQuery.delete_post(post.id);
        }
    }

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&deleted_thread).unwrap());
}

pub async fn update_user(
    req: HttpRequest,
    user_update: web::Json<UserForm>,
    id: web::Path<i32>,
) -> impl Responder {
    let auth_response = check_auth(&req);

    if user_update.username.eq("") || user_update.password.eq("") || user_update.email.eq("") {
        return HttpResponse::BadRequest().body("");
    } else if auth_response.status().is_client_error() {
        return auth_response;
    }

    let updated_user = user_query::UserQuery.update_user(*id, &user_update);

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&updated_user).unwrap());
}

pub async fn delete_user(req: HttpRequest, id: web::Path<i32>) -> impl Responder {
    let auth_response = check_auth(&req);

    if auth_response.status().is_client_error() {
        return auth_response;
    }

    let deleted_user = user_query::UserQuery.delete_user(*id);

    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&deleted_user).unwrap());
}

#[options("/threads")]
pub async fn threads_methods() -> impl Responder {
    HttpResponse::NoContent()
        .append_header(("Allow", "GET, HEAD, POST, DELETE, PUT"))
        .finish()
}
