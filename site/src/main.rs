#[actix_web::get("/")]
async fn hello() -> impl actix_web::Responder {
  let mut conn = common::database::establish_connection();
  let orders = common::database::get_orders(&mut conn);
  let orders_json =
    serde_json::to_string(&orders).expect("couldn't serialize orders");
  actix_web::HttpResponse::Ok().body(orders_json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  actix_web::HttpServer::new(|| actix_web::App::new().service(hello))
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
