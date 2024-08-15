use actix_web::{App, HttpResponse, HttpServer, Responder, Result, web};

#[actix_web::post("/ping")]
pub async fn ping() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().body("pong"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ping))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
