use actix_web::{App, HttpServer};
mod views;
mod to_do;
mod processes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(views::views_factory)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
