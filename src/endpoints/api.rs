use actix_session::Session;
use actix_web::{HttpResponse, web};
use crate::endpoints::api_helper::validate_sign_up;
use crate::models::user::{UserForm, UserLogin};

use super::api_helper::validate_sign_in;

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