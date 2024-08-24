use actix_cors::Cors;
use actix_session::{
    config::{BrowserSession, CookieContentSecurity},
    storage::CookieSessionStore,
    SessionMiddleware,
};
use actix_web::{
    cookie::{Key, SameSite},
    middleware::Logger,
    web::{delete, get, post, put, scope},
    App, HttpServer,
};
use dotenvy::dotenv;
use env_logger::Env;
use lumen_backend::endpoints::{
    api::{
        create_post, create_thread, delete_post, delete_thread, delete_user, get_post_by_id,
        get_posts, get_posts_by_answer_id, get_posts_by_thread_id, get_thread_by_id, get_threads,
        get_user, get_user_by_id, get_users, hello, sign_in, sign_out, sign_up, update_post,
        update_thread, update_user,
    },
    news_api::get_news,
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let secret_key = Key::generate();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    /*    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap(); */

    HttpServer::new(move || {
        //let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL must be set!");
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .supports_credentials()
            .allow_any_header()
            .max_age(3600);

        //using openssl for HTTP/2 compatibality

        App::new()
            .wrap(cors)
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name(String::from("lumen-cookie"))
                    .cookie_secure(true)
                    .session_lifecycle(BrowserSession::default())
                    .cookie_same_site(SameSite::None)
                    .cookie_content_security(CookieContentSecurity::Private)
                    .cookie_http_only(true)
                    .build(),
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/", get().to(hello))
            .service(
                scope("/api")
                    .route("/hello", get().to(hello))
                    .route("/sign-up", post().to(sign_up))
                    .route("/sign-in", post().to(sign_in))
                    .route("/sign-out", post().to(sign_out))
                    .route("/user/{username}", get().to(get_user))
                    .route("/users", get().to(get_users))
                    .route("/users/{user_id}", get().to(get_user_by_id))
                    .route("/users/{user_id}", put().to(update_user))
                    .route("/users/{user_id}", delete().to(delete_user))
                    .route("/posts", get().to(get_posts))
                    .route("/posts", post().to(create_post))
                    .route("/posts/{post_id}", get().to(get_post_by_id))
                    .route(
                        "/posts/answers/{answer_id}",
                        get().to(get_posts_by_answer_id),
                    )
                    .route(
                        "/posts/threads/{thread_id}",
                        get().to(get_posts_by_thread_id),
                    )
                    .route("/posts/{post_id}", put().to(update_post))
                    .route("/posts/{post_id}", delete().to(delete_post))
                    .route("/threads", get().to(get_threads))
                    .route("/threads", post().to(create_thread))
                    .route("/threads/{thread_id}", get().to(get_thread_by_id))
                    .route("/threads/{thread_id}", put().to(update_thread))
                    .route("/threads/{thread_id}", delete().to(delete_thread))
                    .route("/news", get().to(get_news)),
            )
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
