use crate::models::user::{NewUser, Role};
use crate::queries::user_query;
use crate::models::user::{User, UserForm};
use actix_web::{web::Json, HttpResponse};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

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

    if !is_username_unique(&user_form.username, user_query::UserQuery.get_users()) {
        return HttpResponse::BadRequest().body("username is already taken!");
    }

    let password = generate_hashed_password(&user_form.password.to_string());

    let _inserted_user = user_query::UserQuery.create_user(
        &NewUser {
            username: user_form.username.clone(),
            email: user_form.email.clone(),
            password: password,
            role: format!("{}", Role::User)
        }
       
    );

    HttpResponse::Ok().body("post user successed!")
}

/*
sign_in - validator
*/
pub fn compare_passwords(password: &String, hash_string: &String) -> bool {
    let alg: &[&dyn PasswordVerifier] = &[&Argon2::default()];

    let hash = PasswordHash::new(hash_string).unwrap();

    match hash.verify_password(alg, password) {
        Ok(()) => true,
        Err(_) => false,
    }
}

pub fn compare_users(
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
