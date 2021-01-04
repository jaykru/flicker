use actix_web::{web, App, HttpResponse, HttpRequest, HttpServer, Responder};
mod bulb;
use bulb::Bulb;
use std::sync::Mutex;

async fn status(b: web::Data::<Mutex<Bulb>>, _req: HttpRequest) -> impl Responder {
    let mut b = b.lock().unwrap();
    HttpResponse::Ok().body(format!("{:#?}", &mut b.status()))
}

async fn flip(b: web::Data::<Mutex<Bulb>>, _req: HttpRequest) -> impl Responder {
    let mut b = b.lock().unwrap();
    let new = &mut b.flip(); // flips the status and returns the new status

    HttpResponse::Ok().body(format!("flipped to {:#?}", new))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let b = web::Data::new(Mutex::new(bulb::bulb()));
    HttpServer::new(move || {
        App::new()
            .app_data(b.clone())
            .service(web::resource("/flip").route(web::put().to(flip)))
            .service(web::resource("/").route(web::get().to(status)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}