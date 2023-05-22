use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
  let mut conn = common::establish_connection();
  let orders = common::get_orders(&mut conn);
  let orders_json =
    serde_json::to_string(&orders).expect("couldn't serialize orders");
  HttpResponse::Ok().body(orders_json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().service(hello))
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
