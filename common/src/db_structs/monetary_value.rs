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
pub struct MonetaryValue {
  // ISO 4217 currency code string.
  pub value: String,
  // Monetary amount with 1,000,000 limit and no markers for the dollar amount.
  // Conforms to the selected ISO currency's precision.
  // (e.g., JPY uses 123, but USD uses 123.00 or 123)
  pub currency: String,
}

impl diesel::deserialize::FromSql<diesel::sql_types::Jsonb, diesel::pg::Pg>
  for MonetaryValue
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
  for MonetaryValue
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
