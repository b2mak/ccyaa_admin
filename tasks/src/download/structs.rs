#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Orders {
  pub result: Vec<common::database::structs::NewOrder>,
  pub pagination: Pagination,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
  pub has_next_page: bool,
  pub next_page_cursor: Option<String>,
  pub next_page_url: Option<String>,
}
