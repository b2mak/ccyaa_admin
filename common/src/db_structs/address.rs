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
pub struct Address {
  pub first_name: String,
  pub last_name: String,
  pub address_1: String,
  pub address_2: Option<String>,
  pub city: String,
  pub state: Option<String>,
  pub country_code: String,
  pub postal_code: Option<String>,
  pub phone: String,
}

impl diesel::deserialize::FromSql<diesel::sql_types::Jsonb, diesel::pg::Pg>
  for Address
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
  for Address
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
