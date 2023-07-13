use diesel::prelude::*;
use lumen_backend::{*, models::user::User};

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("ID: {}", user.id);
        println!("username: {}", user.username);
        println!("password: {}", user.password);
        println!("email: {}", user.email);
    }
}