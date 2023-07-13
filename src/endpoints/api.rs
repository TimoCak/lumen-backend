use actix_web::HttpResponse;

pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("lumen-backend is running!")
}