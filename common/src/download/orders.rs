#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Orders {
  pub result: Vec<crate::download::Order>,
  pub pagination: crate::download::Pagination,
}
