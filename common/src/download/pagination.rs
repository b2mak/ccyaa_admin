#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
  pub has_next_page: bool,
  pub next_page_cursor: Option<String>,
  pub next_page_url: Option<String>,
}
