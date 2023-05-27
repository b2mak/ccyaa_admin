use diesel::RunQueryDsl;

mod download;

#[tokio::main]
async fn main() {
  let bearer_token = "*token*";

  let mut cursor: Option<String> = None;
  let mut next_page_eh = true;
  while next_page_eh {
    let orders = download::orders_call(bearer_token, &cursor)
      .await
      .expect("Request for orders failed");

    println!("Successful call to Orders to {:#?}", orders);
    let mut conn = common::database::establish_connection();
    insert_new_orders_into_db(&orders.result, &mut conn);

    next_page_eh = orders.pagination.has_next_page;
    if next_page_eh {
      cursor = orders.pagination.next_page_cursor;
    }
  }
}

fn insert_new_orders_into_db(
  new_orders: &Vec<common::database::structs::Order>,
  conn: &mut diesel::PgConnection,
) -> () {
  diesel::insert_into(common::schema::orders::table)
    .values(new_orders)
    .execute(conn)
    .expect("Error saving new order");
}
