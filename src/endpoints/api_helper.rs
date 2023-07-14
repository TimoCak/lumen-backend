use actix_web::{web::Json, HttpResponse, http::header::ContentType};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::{models::user::{UserForm, User}, establish_connection};
use crate::queries::insert_user::create_user;
use serde_json::to_string;
use crate::queries::select_users::get_users;

/* 
sign_up - validator
*/
pub fn generate_hashed_password(password: &String) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("hash failed!")
        .to_string();
    
    let parsed_hash = PasswordHash::new(&password_hash).expect("parsing hash failed!");
    parsed_hash.to_string()
}

pub fn is_username_unique(username: &String, users: Vec<User>) -> bool {
    for user in users {
        if user.username.eq(username) {
            return false;
        }
    }
    true
}

pub fn validate_sign_up(user_form: Json<UserForm>) -> HttpResponse {

    if user_form.username.eq("") || user_form.email.eq("") || user_form.password.eq("") {
        return HttpResponse::BadRequest().body("fill out all input fields!");
    }
    
    if !is_username_unique(&user_form.username, get_users()) {
        return HttpResponse::BadRequest().body("username is already taken!");
    }

    let password = generate_hashed_password(&user_form.password.to_string());

    let conn = &mut establish_connection();

    let inserted_user = create_user(conn, user_form.username.as_str(), user_form.email.as_str(), password.as_str());

    //eintrag in Datenbank
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(to_string(&inserted_user).unwrap())
}


/*
sign_in - validator
*/
pub fn compare_password(password: String, hash_string: String) -> String {
    todo!()
}