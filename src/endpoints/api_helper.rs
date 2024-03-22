use crate::models::user::{NewUser, Role};
use crate::models::user::{User, UserForm};
use crate::models::ErrorResponse;
use crate::queries::user_query;
use actix_web::http::header::{self, Header};
use actix_web::{web::Json, HttpRequest, HttpResponse};
use actix_web_httpauth::headers::authorization::{Authorization, Basic};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::DateTime;
use std::time::{SystemTime, UNIX_EPOCH};

/*
sign_up - validator
*/
pub(crate) fn generate_hashed_password(password: &String) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("hash failed!")
        .to_string();

    let parsed_hash = PasswordHash::new(&password_hash).expect("parsing hash failed!");
    parsed_hash.to_string()
}

pub(crate) fn is_username_unique(username: &String, users: Vec<User>) -> bool {
    for user in users {
        if user.username.eq(username) {
            return false;
        }
    }
    true
}

pub(crate) fn validate_sign_up(user_form: Json<UserForm>) -> HttpResponse {
    if user_form.username.eq("") || user_form.email.eq("") || user_form.password.eq("") {
        return HttpResponse::BadRequest().body("fill out all input fields!");
    }

    if !is_username_unique(&user_form.username, user_query::UserQuery.get_users()) {
        return HttpResponse::BadRequest().body("username is already taken!");
    }

    let password = generate_hashed_password(&user_form.password.to_string());

    let _inserted_user = user_query::UserQuery.create_user(&NewUser {
        username: user_form.username.clone(),
        email: user_form.email.clone(),
        password: password,
        role: format!("{}", Role::User),
    });

    HttpResponse::Ok().body("post user successed!")
}

/*
sign_in - validator
*/
pub(crate) fn compare_passwords(password: &String, hash_string: &String) -> bool {
    let alg: &[&dyn PasswordVerifier] = &[&Argon2::default()];

    let hash = PasswordHash::new(hash_string).unwrap();

    match hash.verify_password(alg, password) {
        Ok(()) => true,
        Err(_) => false,
    }
}

pub(crate) fn compare_users(
    form_username: &String,
    form_password: &String,
    db_username: &String,
    db_password: &String,
) -> bool {
    if form_username.eq(db_username) && compare_passwords(form_password, db_password) {
        return true;
    }
    false
}

//Base Authorization helper
pub(crate) fn check_auth(req: &HttpRequest) -> HttpResponse {
    let auth = Authorization::<Basic>::parse(req).expect("parsed basic auth credentials");
    let user = user_query::UserQuery.get_user_by_username(&auth.as_ref().user_id().to_string());
    let mut error_response: ErrorResponse = ErrorResponse {
        timestamp: DateTime::from_timestamp(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .try_into()
                .unwrap(),
            0,
        )
        .unwrap()
        .to_string(),
        status: 401,
        message: "user is not correctly authorized for this action".to_string(),
        path: req.path().to_string(),
    };

    if let Some(user_db) = user.get(0) {
        if auth.as_ref().password().unwrap().ne(&user_db.password)
            || auth.as_ref().user_id().ne(&user_db.username)
        {
            return HttpResponse::Unauthorized()
                .insert_header(header::ContentType(mime::APPLICATION_JSON))
                .body(serde_json::to_string(&error_response).unwrap());
        }
    } else {
        error_response.message = "User is not registered!".to_string();
        return HttpResponse::Unauthorized().body(serde_json::to_string(&error_response).unwrap());
    }

    return HttpResponse::Ok().finish();
}
