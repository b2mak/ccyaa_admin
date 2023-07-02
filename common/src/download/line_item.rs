#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LineItem {
  pub id: String,
  pub variant_id: Option<String>,
  pub sku: String,
  pub weight: f32,
  pub width: f32,
  pub length: f32,
  pub height: f32,
  pub product_id: Option<String>,
  pub product_name: Option<String>,
  pub quantity: i32,
  pub unit_price_paid: crate::database::structs::MonetaryValue,
  pub variant_options: Vec<crate::database::structs::VariantOption>,
  pub customizations: Option<Vec<crate::database::structs::Customization>>,
  pub image_url: String,
  // TODO: I think this can also be an enum
  pub line_item_type: String,
}
