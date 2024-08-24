use actix_web::{http::header::ContentType, HttpResponse, Responder};
use chrono::{DateTime, Datelike, Local};
use reqwest::{header, Client};

pub(crate) const DAY_RANGE: u32 = 7;

pub(crate) const URL: &str = "https://newsapi.org/v2";
pub(crate) const LANGUAGE: &str = "en";
pub(crate) const TOPIC: &str = "gaming";

pub(crate) fn calc_starting_news_date(current_day: u32) -> u32 {
    if current_day > DAY_RANGE {
        return current_day - DAY_RANGE;
    }
    1
}

pub async fn get_news() -> impl Responder {
    let local_time: DateTime<Local> = Local::now();
    let request_date = format!(
        "{}-{}-{}",
        local_time.year(),
        local_time.month(),
        calc_starting_news_date(local_time.day())
    );
    let url = format!("{}/everything?q={}&language={}&from={}&sortBy=publishedAt&apiKey=e7ae4bb45c3f443d8710166599bf1119", URL, TOPIC, LANGUAGE, request_date);

    let client = Client::new();

    let response = client
        .get(&url)
        .header(header::USER_AGENT, "Lumen/0.1.0")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}
