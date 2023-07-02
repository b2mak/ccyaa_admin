#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
  pub id: String,
  pub order_number: String,
  // TODO: parse this into some usable time format instead of pure string
  pub created_on: String,
  pub modified_on: String,
  pub channel: String,
  pub testmode: bool,
  pub customer_email: String,
  pub billing_address: crate::database::structs::Address,
  pub shipping_address: Option<crate::database::structs::Address>,
  // TODO: This can be an enum
  // Value may be: `PENDING`, `FULFILLED`, or `CANCELED`.
  pub fulfillment_status: String,
  pub line_items: Vec<crate::download::LineItem>,
  pub internal_notes: Vec<crate::database::structs::InternalNote>,
  pub shipping_lines: Vec<crate::database::structs::ShippingLine>,
  pub discount_lines: Vec<crate::database::structs::DiscountLine>,
  pub form_submission: Option<Vec<crate::database::structs::FormSubmission>>,
  pub fulfillments: Vec<crate::database::structs::Fulfillment>,
  pub subtotal: crate::database::structs::MonetaryValue,
  pub shipping_total: crate::database::structs::MonetaryValue,
  pub discount_total: crate::database::structs::MonetaryValue,
  pub tax_total: crate::database::structs::MonetaryValue,
  pub refunded_total: crate::database::structs::MonetaryValue,
  pub grand_total: crate::database::structs::MonetaryValue,
  pub channel_name: String,
  pub external_order_reference: Option<String>,
  // TODO: make this a timestamp or something
  pub fulfilled_on: Option<String>,
  // TODO: make this an enum
  // Values may be `EXCLUSIVE` or `INCLUSIVE`.
  pub price_tax_interpretation: String,
}
