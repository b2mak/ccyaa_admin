#[derive(
  diesel::FromSqlRow,
  diesel::AsExpression,
  serde::Serialize,
  serde::Deserialize,
  Debug,
  Clone,
)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
#[serde(rename_all = "camelCase")]
pub struct VariantOption {
  pub value: String,
  pub option_name: String,
}

impl diesel::deserialize::FromSql<diesel::sql_types::Jsonb, diesel::pg::Pg>
  for VariantOption
{
  fn from_sql(
    bytes: diesel::pg::PgValue<'_>,
  ) -> diesel::deserialize::Result<Self> {
    let value = <serde_json::Value as diesel::deserialize::FromSql<
      diesel::sql_types::Jsonb,
      diesel::pg::Pg,
    >>::from_sql(bytes)?;
    return Ok(serde_json::from_value(value)?);
  }
}
impl diesel::serialize::ToSql<diesel::sql_types::Jsonb, diesel::pg::Pg>
  for VariantOption
{
  fn to_sql<'b>(
    &'b self,
    out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
  ) -> diesel::serialize::Result {
    let value = serde_json::to_value(self)?;
    return <serde_json::Value as diesel::serialize::ToSql<
      diesel::sql_types::Jsonb,
      diesel::pg::Pg,
    >>::to_sql(&value, &mut out.reborrow());
  }
}

#[derive(
  diesel::FromSqlRow,
  diesel::AsExpression,
  serde::Serialize,
  serde::Deserialize,
  Debug,
  Clone,
)]
#[diesel(sql_type = diesel::sql_types::Jsonb)]
#[serde(rename_all = "camelCase")]
pub struct Customization {
  pub label: String,
  pub value: String,
}

impl diesel::deserialize::FromSql<diesel::sql_types::Jsonb, diesel::pg::Pg>
  for Customization
{
  fn from_sql(
    bytes: diesel::pg::PgValue<'_>,
  ) -> diesel::deserialize::Result<Self> {
    let value = <serde_json::Value as diesel::deserialize::FromSql<
      diesel::sql_types::Jsonb,
      diesel::pg::Pg,
    >>::from_sql(bytes)?;
    return Ok(serde_json::from_value(value)?);
  }
}
impl diesel::serialize::ToSql<diesel::sql_types::Jsonb, diesel::pg::Pg>
  for Customization
{
  fn to_sql<'b>(
    &'b self,
    out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
  ) -> diesel::serialize::Result {
    let value = serde_json::to_value(self)?;
    return <serde_json::Value as diesel::serialize::ToSql<
      diesel::sql_types::Jsonb,
      diesel::pg::Pg,
    >>::to_sql(&value, &mut out.reborrow());
  }
}

#[derive(
  diesel::prelude::Insertable,
  diesel::prelude::Queryable,
  diesel::prelude::Identifiable,
  serde::Serialize,
  serde::Deserialize,
  Debug,
)]
#[diesel(table_name = crate::schema::line_items)]
#[diesel(belongs_to(Order))]
#[serde(rename_all = "camelCase")]
pub struct LineItem {
  pub id: String,
  pub order_id: Option<String>,
  pub variant_id: Option<String>,
  pub sku: String,
  pub weight: f32,
  pub width: f32,
  pub length: f32,
  pub height: f32,
  pub product_id: Option<String>,
  pub product_name: Option<String>,
  pub quantity: i32,
  pub unit_price_paid: super::MonetaryValue,
  pub variant_options: Vec<VariantOption>,
  pub customizations: Option<Vec<Customization>>,
  pub image_url: String,
  // TODO: I think this can also be an enum
  pub line_item_type: String,
}
