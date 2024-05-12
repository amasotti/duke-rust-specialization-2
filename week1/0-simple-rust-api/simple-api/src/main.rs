use actix_web::{App, HttpServer, middleware::Logger};
use simple_logger::SimpleLogger;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    SimpleLogger::new().init().unwrap();

    HttpServer::new(|| {
        App::new()
            .service(api::greet)
            .service(api::echo)
            .service(api::health)
            .wrap(Logger::default())
    })
    .bind(("localhost", 8085))?
    .run()
    .await
}
