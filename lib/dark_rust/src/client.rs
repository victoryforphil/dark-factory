use reqwest::Method;
use serde_json::Value;

use crate::error::DarkRustError;
use crate::types::{
  OpencodeAttachQuery, OpencodeSessionCommandInput, OpencodeSessionCreateInput,
  OpencodeSessionDirectoryInput, OpencodeSessionPromptInput, OpencodeSessionStateQuery,
  ProductCreateInput, ProductListQuery, ProductUpdateInput, VariantCreateInput,
  VariantListQuery, VariantUpdateInput,
};

#[derive(Debug, Clone)]
pub struct RawApiResponse {
  pub status: u16,
  pub path: String,
  pub body: Value,
}

#[derive(Debug, Clone)]
pub struct DarkCoreClient {
  base_url: String,
  http: reqwest::Client,
}

impl DarkCoreClient {
  pub fn new(base_url: String) -> Self {
    Self {
      base_url: base_url.trim_end_matches('/').to_string(),
      http: reqwest::Client::new(),
    }
  }

  pub async fn service_status(&self) -> Result<RawApiResponse, DarkRustError> {
    self.get("/", None).await
  }

  pub async fn system_health(&self) -> Result<RawApiResponse, DarkRustError> {
    self.get("/system/health", None).await
  }

  pub async fn system_info(&self) -> Result<RawApiResponse, DarkRustError> {
    self.get("/system/info", None).await
  }

  pub async fn system_metrics(&self) -> Result<RawApiResponse, DarkRustError> {
    self.get("/system/metrics", None).await
  }

  pub async fn system_reset_db(&self) -> Result<RawApiResponse, DarkRustError> {
    self.post("/system/reset-db", Value::Null).await
  }

  pub async fn products_list(&self, query: &ProductListQuery) -> Result<RawApiResponse, DarkRustError> {
    let mut query_parts = Vec::new();

    if let Some(cursor) = &query.cursor {
      query_parts.push(("cursor".to_string(), cursor.clone()));
    }

    if let Some(limit) = query.limit {
      query_parts.push(("limit".to_string(), limit.to_string()));
    }

    let query = if query_parts.is_empty() {
      None
    } else {
      Some(query_parts.as_slice())
    };

    self.get("/products/", query).await
  }

  pub async fn products_create(
    &self,
    input: &ProductCreateInput,
  ) -> Result<RawApiResponse, DarkRustError> {
    self.post("/products/", serde_json::to_value(input)?).await
  }

  pub async fn products_get(&self, product_id: &str) -> Result<RawApiResponse, DarkRustError> {
    self.get(&format!("/products/{product_id}"), None).await
  }

  pub async fn products_update(
    &self,
    product_id: &str,
    input: &ProductUpdateInput,
  ) -> Result<RawApiResponse, DarkRustError> {
    self
      .patch(&format!("/products/{product_id}"), serde_json::to_value(input)?)
      .await
  }

  pub async fn products_delete(&self, product_id: &str) -> Result<RawApiResponse, DarkRustError> {
    self.delete(&format!("/products/{product_id}"), None).await
  }

  pub async fn variants_list(&self, query: &VariantListQuery) -> Result<RawApiResponse, DarkRustError> {
    let mut query_parts = Vec::new();

    if let Some(cursor) = &query.cursor {
      query_parts.push(("cursor".to_string(), cursor.clone()));
    }

    if let Some(limit) = query.limit {
      query_parts.push(("limit".to_string(), limit.to_string()));
    }

    if let Some(product_id) = &query.product_id {
      query_parts.push(("productId".to_string(), product_id.clone()));
    }

    if let Some(locator) = &query.locator {
      query_parts.push(("locator".to_string(), locator.clone()));
    }

    if let Some(name) = &query.name {
      query_parts.push(("name".to_string(), name.clone()));
    }

    let query = if query_parts.is_empty() {
      None
    } else {
      Some(query_parts.as_slice())
    };

    self.get("/variants/", query).await
  }

  pub async fn variants_create(
    &self,
    input: &VariantCreateInput,
  ) -> Result<RawApiResponse, DarkRustError> {
    self.post("/variants/", serde_json::to_value(input)?).await
  }

  pub async fn variants_get(&self, variant_id: &str) -> Result<RawApiResponse, DarkRustError> {
    self.get(&format!("/variants/{variant_id}"), None).await
  }

  pub async fn variants_update(
    &self,
    variant_id: &str,
    input: &VariantUpdateInput,
  ) -> Result<RawApiResponse, DarkRustError> {
    self
      .patch(&format!("/variants/{variant_id}"), serde_json::to_value(input)?)
      .await
  }

  pub async fn variants_delete(&self, variant_id: &str) -> Result<RawApiResponse, DarkRustError> {
    self.delete(&format!("/variants/{variant_id}"), None).await
  }

  pub async fn opencode_state(&self, directory: &str) -> Result<RawApiResponse, DarkRustError> {
    let query = [("directory".to_string(), directory.to_string())];
    self.get("/opencode/state", Some(&query)).await
  }

  pub async fn opencode_sessions_list(
    &self,
    directory: &str,
  ) -> Result<RawApiResponse, DarkRustError> {
    let query = [("directory".to_string(), directory.to_string())];
    self.get("/opencode/sessions", Some(&query)).await
  }

  pub async fn opencode_sessions_create(
    &self,
    input: &OpencodeSessionCreateInput,
  ) -> Result<RawApiResponse, DarkRustError> {
    self.post("/opencode/sessions", serde_json::to_value(input)?).await
  }

  pub async fn opencode_sessions_get(
    &self,
    session_id: &str,
    query: &OpencodeSessionStateQuery,
  ) -> Result<RawApiResponse, DarkRustError> {
    let mut query_parts = vec![("directory".to_string(), query.directory.clone())];

    if query.include_messages {
      query_parts.push(("includeMessages".to_string(), "true".to_string()));
    }

    self
      .get(&format!("/opencode/sessions/{session_id}"), Some(query_parts.as_slice()))
      .await
  }

  pub async fn opencode_sessions_attach(
    &self,
    session_id: &str,
    query: &OpencodeAttachQuery,
  ) -> Result<RawApiResponse, DarkRustError> {
    let mut query_parts = vec![("directory".to_string(), query.directory.clone())];

    if let Some(model) = &query.model {
      query_parts.push(("model".to_string(), model.clone()));
    }

    if let Some(agent) = &query.agent {
      query_parts.push(("agent".to_string(), agent.clone()));
    }

    self
      .get(
        &format!("/opencode/sessions/{session_id}/attach"),
        Some(query_parts.as_slice()),
      )
      .await
  }

  pub async fn opencode_sessions_command(
    &self,
    session_id: &str,
    input: &OpencodeSessionCommandInput,
  ) -> Result<RawApiResponse, DarkRustError> {
    self
      .post(
        &format!("/opencode/sessions/{session_id}/command"),
        serde_json::to_value(input)?,
      )
      .await
  }

  pub async fn opencode_sessions_prompt(
    &self,
    session_id: &str,
    input: &OpencodeSessionPromptInput,
  ) -> Result<RawApiResponse, DarkRustError> {
    self
      .post(
        &format!("/opencode/sessions/{session_id}/prompt"),
        serde_json::to_value(input)?,
      )
      .await
  }

  pub async fn opencode_sessions_abort(
    &self,
    session_id: &str,
    input: &OpencodeSessionDirectoryInput,
  ) -> Result<RawApiResponse, DarkRustError> {
    self
      .post(
        &format!("/opencode/sessions/{session_id}/abort"),
        serde_json::to_value(input)?,
      )
      .await
  }

  pub async fn opencode_sessions_delete(
    &self,
    session_id: &str,
    directory: &str,
  ) -> Result<RawApiResponse, DarkRustError> {
    let query = [("directory".to_string(), directory.to_string())];

    self
      .delete(&format!("/opencode/sessions/{session_id}"), Some(&query))
      .await
  }

  async fn get(
    &self,
    path: &str,
    query: Option<&[(String, String)]>,
  ) -> Result<RawApiResponse, DarkRustError> {
    self.request(Method::GET, path, query, None).await
  }

  async fn post(&self, path: &str, body: Value) -> Result<RawApiResponse, DarkRustError> {
    self.request(Method::POST, path, None, Some(body)).await
  }

  async fn patch(&self, path: &str, body: Value) -> Result<RawApiResponse, DarkRustError> {
    self.request(Method::PATCH, path, None, Some(body)).await
  }

  async fn delete(
    &self,
    path: &str,
    query: Option<&[(String, String)]>,
  ) -> Result<RawApiResponse, DarkRustError> {
    self.request(Method::DELETE, path, query, None).await
  }

  async fn request(
    &self,
    method: Method,
    path: &str,
    query: Option<&[(String, String)]>,
    body: Option<Value>,
  ) -> Result<RawApiResponse, DarkRustError> {
    let path_normalized = normalize_path(path);
    let mut url = format!("{}{}", self.base_url, path_normalized);
    let method_label = method.as_str().to_string();

    if let Some(query_values) = query {
      append_query(&mut url, query_values);
    }

    let mut request = self.http.request(method, url);

    if let Some(body_value) = body {
      request = request
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(body_value.to_string());
    }

    let response = request.send().await.map_err(|source| DarkRustError::Http {
      method: method_label.clone(),
      path: path_normalized.clone(),
      source,
    })?;

    let status = response.status().as_u16();
    let response_text = response.text().await.map_err(|source| DarkRustError::Http {
      method: method_label,
      path: path_normalized.clone(),
      source,
    })?;

    Ok(RawApiResponse {
      status,
      path: path_normalized,
      body: parse_body(response_text),
    })
  }
}

fn normalize_path(path: &str) -> String {
  if path.starts_with('/') {
    path.to_string()
  } else {
    format!("/{path}")
  }
}

fn parse_body(response_text: String) -> Value {
  if response_text.trim().is_empty() {
    return Value::Null;
  }

  serde_json::from_str(&response_text).unwrap_or(Value::String(response_text))
}

fn append_query(url: &mut String, query: &[(String, String)]) {
  if query.is_empty() {
    return;
  }

  let mut first = true;

  for (key, value) in query {
    if first {
      url.push('?');
      first = false;
    } else {
      url.push('&');
    }

    url.push_str(&url_encode(key));
    url.push('=');
    url.push_str(&url_encode(value));
  }
}

fn url_encode(value: &str) -> String {
  let mut encoded = String::new();

  for byte in value.as_bytes() {
    match byte {
      b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
        encoded.push(*byte as char)
      }
      _ => encoded.push_str(&format!("%{byte:02X}")),
    }
  }

  encoded
}

#[cfg(test)]
mod tests {
  use super::{append_query, normalize_path, parse_body, url_encode};
  use serde_json::json;

  #[test]
  fn normalizes_relative_paths() {
    assert_eq!(normalize_path("system/health"), "/system/health");
    assert_eq!(normalize_path("/system/health"), "/system/health");
  }

  #[test]
  fn encodes_query_values() {
    assert_eq!(url_encode("dark factory"), "dark%20factory");
    assert_eq!(url_encode("a/b?c"), "a%2Fb%3Fc");
  }

  #[test]
  fn appends_query_pairs() {
    let mut url = "http://localhost:4150/products".to_string();
    let query = [
      ("cursor".to_string(), "abc 123".to_string()),
      ("limit".to_string(), "50".to_string()),
    ];

    append_query(&mut url, &query);

    assert_eq!(url, "http://localhost:4150/products?cursor=abc%20123&limit=50");
  }

  #[test]
  fn parses_response_json() {
    let parsed = parse_body("{\"ok\":true}".to_string());
    assert_eq!(parsed, json!({ "ok": true }));
  }

  #[test]
  fn preserves_plain_text_response() {
    let parsed = parse_body("not-json".to_string());
    assert_eq!(parsed, json!("not-json"));
  }
}
