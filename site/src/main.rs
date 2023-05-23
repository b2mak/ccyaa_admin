const PAGE_SIZE: i64 = 20;

#[derive(serde::Deserialize)]
struct OrdersQuery {
  page: Option<usize>,
}

#[actix_web::get("/orders")]
async fn orders(
  info: actix_web::web::Query<OrdersQuery>,
) -> impl actix_web::Responder {
  let mut conn = common::database::establish_connection();
  let page = if info.page.is_some() {
    info.page.unwrap()
  } else {
    0
  };
  let orders = common::database::get_orders(
    &mut conn,
    PAGE_SIZE,
    i64::try_from(page).unwrap(),
  );
  let orders_json =
    serde_json::to_string(&orders).expect("couldn't serialize orders");
  actix_web::HttpResponse::Ok()
    .content_type(actix_web::http::header::ContentType::json())
    .body(orders_json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  actix_web::HttpServer::new(|| actix_web::App::new().service(orders))
    // Setting to 2 workers for now, should change during production
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
