pub mod structs;

pub fn establish_connection() -> diesel::PgConnection {
  let database_url = get_database_url();
  return <diesel::PgConnection as diesel::Connection>::establish(
    &database_url,
  )
  .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}

pub fn get_database_url() -> String {
  dotenvy::dotenv().ok();
  return std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}

pub fn get_orders(
  conn: &mut diesel::PgConnection,
  page_size: i64,
  page: i64,
) -> Vec<structs::Order> {
  use diesel::*;
  return crate::schema::orders::table
    .order_by(crate::schema::orders::id.asc())
    .limit(page_size)
    .offset(page_size * page)
    .load::<structs::Order>(conn)
    .expect("Error loading orders");
}
