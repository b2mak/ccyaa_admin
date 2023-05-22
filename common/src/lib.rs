use crate::schema::orders::fulfillment_status;
use diesel::*;

pub mod db_structs;
pub mod schema;
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

pub fn get_orders(conn: &mut diesel::PgConnection) -> Vec<db_structs::Order> {
  return schema::orders::dsl::orders
    .filter(fulfillment_status.eq("CANCELED"))
    .limit(5)
    .load::<db_structs::Order>(conn)
    .expect("Error loading orders");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
