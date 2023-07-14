use actix_web::{HttpResponse, web};
use crate::endpoints::api_helper::validate_sign_up;
use crate::models::user::UserForm;

pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("lumen-backend is running!")
}

pub async fn sign_up(user_form: web::Json<UserForm>) -> HttpResponse {
    validate_sign_up(user_form)
}