use actix_web::{
    get,
    middleware::Logger,
    post,
    web::{get, scope, post},
    App, HttpServer,
};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));


    HttpServer::new(|| {
        App::new()
    })  
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
