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
pub struct Fulfillment {
  // ISO 8601 UTC date and time string; represents the moment the fulfillment was shipped.
  pub ship_date: String,
  // Name of the carrier handling the shipment.
  pub carrier_name: String,
  // Carrier's level of service for shipping.
  pub service: Option<String>,
  // Carrier's parcel tracking number.
  pub tracking_number: Option<String>,
  // URL provided by the carrier to track the shipment.
  pub tracking_url: String,
}

impl diesel::deserialize::FromSql<diesel::sql_types::Jsonb, diesel::pg::Pg>
  for Fulfillment
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
  for Fulfillment
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
