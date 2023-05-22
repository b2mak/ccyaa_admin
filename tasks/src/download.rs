use common::structs;

pub async fn orders_call(
  bearer_token: &str,
  cursor: &Option<String>,
) -> Result<structs::Orders, Box<dyn std::error::Error>> {
  let mut url: String =
    "https://api.squarespace.com/1.0/commerce/orders".to_owned();
  match cursor {
    Some(x) => url.push_str(&format!("?cursor={}", x)),
    None => (),
  }
  let client = reqwest::Client::new();
  let response = client
    .get(url)
    .header(reqwest::header::USER_AGENT, "CCYAA Order Syncer")
    .header(reqwest::header::CONTENT_TYPE, "application/json")
    .header(
      reqwest::header::AUTHORIZATION,
      format!("Bearer {}", bearer_token),
    )
    .send()
    .await?;

  match response.status() {
    reqwest::StatusCode::OK => {
      let text = response.text().await?;
      // Some Deserializer.
      let jd = &mut serde_json::Deserializer::from_str(&text);
      let result: Result<structs::Orders, _> =
        serde_path_to_error::deserialize(jd);

      if result.is_err() {
        println!("Body that has error:");
        println!("{}", text);
      }

      return Ok(result.unwrap());
    }
    reqwest::StatusCode::UNAUTHORIZED => {
      panic!("Invalid API token");
    }
    _ => {
      panic!("Unexpected status code: {}", response.status());
    }
  };
}
