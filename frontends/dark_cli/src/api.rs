use reqwest::Method;
use serde_json::Value;

use crate::errors::DarkCliError;

#[derive(Debug)]
pub struct ApiResponse {
  pub status: u16,
  pub path: String,
  pub body: Value,
}

pub struct ApiClient {
  base_url: String,
  http: reqwest::Client,
}

impl ApiClient {
  pub fn new(base_url: String) -> Self {
    Self {
      base_url: base_url.trim_end_matches('/').to_string(),
      http: reqwest::Client::new(),
    }
  }

  pub async fn get(
    &self,
    path: &str,
    query: Option<Vec<(String, String)>>,
  ) -> Result<ApiResponse, DarkCliError> {
    self.request(Method::GET, path, query, None).await
  }

  pub async fn post(&self, path: &str, body: Value) -> Result<ApiResponse, DarkCliError> {
    self.request(Method::POST, path, None, Some(body)).await
  }

  pub async fn delete(
    &self,
    path: &str,
    query: Option<Vec<(String, String)>>,
  ) -> Result<ApiResponse, DarkCliError> {
    self.request(Method::DELETE, path, query, None).await
  }

  async fn request(
    &self,
    method: Method,
    path: &str,
    query: Option<Vec<(String, String)>>,
    body: Option<Value>,
  ) -> Result<ApiResponse, DarkCliError> {
    let path_normalized = normalize_path(path);
    let mut url = format!("{}{}", self.base_url, path_normalized);
    let method_label = method.as_str().to_string();

    if let Some(query_values) = query.as_ref() {
      append_query(&mut url, query_values);
    }

    let mut request = self.http.request(method, url);

    if let Some(body_value) = body {
      request = request
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(body_value.to_string());
    }

    let response = request.send().await.map_err(|source| DarkCliError::Http {
      method: method_label.clone(),
      path: path_normalized.clone(),
      source,
    })?;

    let status = response.status().as_u16();
    let response_text = response.text().await.map_err(|source| DarkCliError::Http {
      method: method_label,
      path: path_normalized.clone(),
      source,
    })?;

    let body = parse_body(response_text);

    Ok(ApiResponse {
      status,
      path: path_normalized,
      body,
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
      _ => {
        encoded.push_str(&format!("%{byte:02X}"));
      }
    }
  }

  encoded
}
