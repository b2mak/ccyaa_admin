mod address;
mod discount_line;
mod form_submission;
mod fulfillment;
mod internal_note;
mod line_item;
mod monetary_value;
mod shipping_line;

pub use address::Address;
pub use discount_line::DiscountLine;
pub use form_submission::FormSubmission;
pub use fulfillment::Fulfillment;
pub use internal_note::InternalNote;
pub use line_item::{Customization, LineItem, VariantOption};
pub use monetary_value::MonetaryValue;
pub use shipping_line::ShippingLine;

#[derive(
  diesel::prelude::Insertable,
  serde::Serialize,
  serde::Deserialize,
  Debug,
  diesel::prelude::Queryable,
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
