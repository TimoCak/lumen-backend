use std::env;

use actix_web::{
    middleware::Logger,
    web::{get, scope, post},
    App, HttpServer,
};
use actix_cors::Cors;
use dotenvy::dotenv;
use env_logger::Env;
use lumen_backend::endpoints::api::{hello, sign_up};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
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
            .route("/", get().to(hello))
            .service(
                scope("/api")
                    .route("/hello", get().to(hello))
                    .route("/sign-up", post().to(sign_up))   
            )
    })  
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
