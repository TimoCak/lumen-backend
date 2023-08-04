use crate::queries::insert_user::create_user;
use crate::queries::select_users::get_users;
use crate::{
    establish_connection,
    models::user::{User, UserForm},
    queries::select_user::get_user_by_username,
};
use actix_session::Session;
use actix_web::http::header::ContentType;
use actix_web::{web::Json, HttpResponse};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

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

    let _inserted_user = create_user(
        conn,
        user_form.username.as_str(),
        user_form.email.as_str(),
        password.as_str(),
    );

    HttpResponse::Ok().body("post user successed!")
}

/*
sign_in - validator
*/
fn compare_passwords(password: &String, hash_string: &String) -> bool {
    let alg: &[&dyn PasswordVerifier] = &[&Argon2::default()];

    let hash = PasswordHash::new(hash_string).unwrap();

    match hash.verify_password(alg, password) {
        Ok(()) => true,
        Err(_) => false,
    }
}

fn compare_users(
    form_username: &String,
    form_password: &String,
    db_username: &String,
    db_pasword: &String,
) -> bool {
    if form_username.eq(db_username) && compare_passwords(form_password, db_pasword) {
        return true;
    }
    false
}

pub fn validate_sign_in(
    session: Session,
    username: &String,
    password: &String,
) -> HttpResponse {
    #[derive(Serialize, Deserialize, Default)]
    struct ClientStoredUser {
        id: i32,
        username: String,
        email: String,
    }

    if username.eq("") || password.eq("") {
        return HttpResponse::BadRequest().body("please fill out all fields!");
    }

    let mut user = ClientStoredUser::default();

    for user in get_user_by_username(username) {
        if compare_users(username, password, &user.username, &user.password) {
            
            if let Ok(()) = session.insert("userId", &user.username) {
                println!("insertion successed!")
            } else {
                return HttpResponse::InternalServerError().finish();
            }

            if let Ok(()) = session.insert("username", &user.username) {
                println!("insertion successed!")
            } else {
                return HttpResponse::InternalServerError().finish();
            }

            let client_stored_user = ClientStoredUser {
                id: user.id,
                username: user.username,
                email: user.email,
            };
            return HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&client_stored_user).unwrap());
        }
    }
    HttpResponse::Unauthorized().body("username or password is wrong!")
}
