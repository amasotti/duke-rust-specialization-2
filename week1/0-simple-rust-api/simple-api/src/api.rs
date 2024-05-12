use actix_web::{get, post, HttpResponse, Responder};
use chrono::{Datelike, Utc};
use log::{info, warn};

#[get("/hi")]
async fn greet() -> impl Responder {
    info!("Saying hi to the user");
    HttpResponse::Ok().body("Hi folks! 👋\n")
}

#[post("/echo")]
async fn echo(message: String) -> impl Responder {
    match message.len() {
        0 => {
            warn!("Message is empty");
            HttpResponse::UnprocessableEntity().body("Message is empty\n")
        }
        1..=3 => {
            warn!("Message is too short");
            HttpResponse::UnprocessableEntity().body("Message is too short\n")
        }
        _ => {
            info!("Echoing the message");
            HttpResponse::Ok().body(format!("Echo: {}\n", message))
        }
    }
}

/// Check the health of the service
///
/// ## Example
///
/// ```bash
/// curl -X GET http://localhost:8085/health
/// ```
///
#[get("/health")]
async fn health() -> impl Responder {
    // Check which day of the week we're in
    let now = Utc::now();
    let day = now.weekday();

    match day {
        chrono::Weekday::Sat | chrono::Weekday::Sun => {
            HttpResponse::Ok().body("I'm healthy, but I'm taking a break today!\n")
        }
        chrono::Weekday::Mon => {
            warn!("I'm healthy, but I'm not ready for the week yet!");
            HttpResponse::ImATeapot().body("I'm healthy, but I'm not ready for the week yet!\n")
        }
        _ => HttpResponse::Ok().body("I'm healthy!\n"),
    }
}
