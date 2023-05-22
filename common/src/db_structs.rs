use super::schema::orders;
use diesel::prelude::*;

mod address;
mod discount_line;
mod form_submission;
mod fulfillment;
mod internal_note;
mod line_item;
mod monetary_value;
mod shipping_line;

pub type Address = address::Address;
pub type MonetaryValue = monetary_value::MonetaryValue;
pub type InternalNote = internal_note::InternalNote;
pub type ShippingLine = shipping_line::ShippingLine;
pub type DiscountLine = discount_line::DiscountLine;
pub type FormSubmission = form_submission::FormSubmission;
pub type Fulfillment = fulfillment::Fulfillment;
pub type VariantOption = line_item::VariantOption;
pub type Customization = line_item::Customization;
pub type LineItem = line_item::LineItem;

#[derive(Insertable, serde::Serialize, serde::Deserialize, Debug)]
#[diesel(table_name = orders)]
#[serde(rename_all = "camelCase")]
pub struct NewOrder {
  #[diesel(column_name = squarespace_id)]
  pub id: String,
  pub order_number: String,
  pub created_on: String,
  pub modified_on: String,
  pub channel: String,
  pub testmode: bool,
  pub customer_email: String,
  pub billing_address: Address,
  pub shipping_address: Option<Address>,
  pub fulfillment_status: String,
  pub line_items: Vec<LineItem>,
  pub internal_notes: Vec<InternalNote>,
  pub shipping_lines: Vec<ShippingLine>,
  pub discount_lines: Vec<DiscountLine>,
  pub form_submission: Option<Vec<FormSubmission>>,
  pub fulfillments: Vec<Fulfillment>,
  pub subtotal: MonetaryValue,
  pub shipping_total: MonetaryValue,
  pub discount_total: MonetaryValue,
  pub tax_total: MonetaryValue,
  pub refunded_total: MonetaryValue,
  pub grand_total: MonetaryValue,
  pub channel_name: String,
  pub external_order_reference: Option<String>,
  pub fulfilled_on: Option<String>,
  pub price_tax_interpretation: Option<String>,
}

#[derive(
  serde::Serialize,
  serde::Deserialize,
  Queryable,
  Debug,
  Clone,
)]
pub struct Order {
  pub id: i32,
  pub squarespace_id: String,
  pub order_number: String,
  pub created_on: String,
  pub modified_on: String,
  pub channel: String,
  pub testmode: bool,
  pub customer_email: String,
  pub billing_address: Address,
  pub shipping_address: Option<Address>,
  pub fulfillment_status: String,
  pub line_items: Vec<LineItem>,
  pub internal_notes: Vec<InternalNote>,
  pub shipping_lines: Vec<ShippingLine>,
  pub discount_lines: Vec<DiscountLine>,
  pub form_submission: Option<Vec<FormSubmission>>,
  pub fulfillments: Vec<Fulfillment>,
  pub subtotal: MonetaryValue,
  pub shipping_total: MonetaryValue,
  pub discount_total: MonetaryValue,
  pub tax_total: MonetaryValue,
  pub refunded_total: MonetaryValue,
  pub grand_total: MonetaryValue,
  pub channel_name: String,
  pub external_order_reference: Option<String>,
  pub fulfilled_on: Option<String>,
  pub price_tax_interpretation: Option<String>,
}
