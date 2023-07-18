use std::env;

use actix_web::{
    middleware::Logger,
    web::{get, scope, post},
    App, HttpServer, cookie::Key,
};
use actix_cors::Cors;
use dotenvy::dotenv;
use env_logger::Env;
use lumen_backend::endpoints::api::{hello, sign_up, sign_in, sign_out};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let secret_key = Key::generate();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL must be set!");
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), secret_key.clone()))
            .route("/", get().to(hello))
            .service(
                scope("/api")
                    .route("/hello", get().to(hello))
                    .route("/sign-up", post().to(sign_up))   
                    .route("/sign-in", post().to(sign_in))
                    .route("/sign-out", post().to(sign_out))
            )
    })  
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
