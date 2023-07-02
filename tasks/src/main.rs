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
    insert_new_orders_into_db(orders.result, &mut conn);

    next_page_eh = orders.pagination.has_next_page;
    if next_page_eh {
      cursor = orders.pagination.next_page_cursor;
    }
  }
}

fn insert_new_orders_into_db(
  new_orders: Vec<common::download::Order>,
  conn: &mut diesel::PgConnection,
) -> () {
  let mut database_orders: Vec<common::database::structs::Order> = Vec::new();
  let mut database_line_items: Vec<common::database::structs::LineItem> = Vec::new();
  for order in new_orders.iter() {
    for line_item in order.line_items.iter() {
      database_line_items.push(
        common::database::structs::LineItem::from(line_item, &order.id),
      );
    }
    database_orders.push(common::database::structs::Order::from(order));
  }

  // I think we have to insert the orders first
  // Otherwise line items have no order ids to reference
  diesel::insert_into(common::schema::orders::table)
    .values(database_orders)
    .execute(conn)
    .expect("Error saving new order");

  diesel::insert_into(common::schema::line_items::table)
    .values(database_line_items)
    .execute(conn)
    .expect("Error saving new line_items");
}
