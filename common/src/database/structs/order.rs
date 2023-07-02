#[derive(
  diesel::prelude::Insertable,
  diesel::prelude::Queryable,
  diesel::prelude::Identifiable,
  serde::Serialize,
  serde::Deserialize,
  Debug,
)]
#[diesel(table_name = crate::schema::orders)]
#[serde(rename_all = "camelCase")]
pub struct Order {
  pub id: String,
  pub order_number: String,
  pub created_on: String,
  pub modified_on: String,
  pub channel: String,
  pub testmode: bool,
  pub customer_email: String,
  pub billing_address: super::Address,
  pub shipping_address: Option<super::Address>,
  pub fulfillment_status: String,
  pub internal_notes: Vec<super::InternalNote>,
  pub shipping_lines: Vec<super::ShippingLine>,
  pub discount_lines: Vec<super::DiscountLine>,
  pub form_submission: Option<Vec<super::FormSubmission>>,
  pub fulfillments: Vec<super::Fulfillment>,
  pub subtotal: super::MonetaryValue,
  pub shipping_total: super::MonetaryValue,
  pub discount_total: super::MonetaryValue,
  pub tax_total: super::MonetaryValue,
  pub refunded_total: super::MonetaryValue,
  pub grand_total: super::MonetaryValue,
  pub channel_name: String,
  pub external_order_reference: Option<String>,
  pub fulfilled_on: Option<String>,
  pub price_tax_interpretation: Option<String>,
}

impl From<&crate::download::Order> for Order {
  fn from(a: &crate::download::Order) -> Self {
    let mut serialized = serde_json::to_value(a).unwrap();
    let obj = serialized.as_object_mut().expect("Value was not an object");
    obj.remove("line_items");
    return serde_json::from_value(serde_json::to_value(&obj).unwrap()).unwrap()
  }
}

