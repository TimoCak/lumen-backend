use actix_web::{
    get,
    middleware::Logger,
    post,
    web::{get, scope, post},
    App, HttpServer, HttpResponse,
};
use env_logger::Env;
use lumen_backend::endpoints::api::hello;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));


    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/", get().to(hello))
            .service(
                scope("/api")
                    .route("/hello", get().to(hello))   
            )
    })  
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
