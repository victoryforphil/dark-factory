use anyhow::{Context, Result, anyhow, bail};
use reqwest::Method;
use serde_json::{Value, json};

use super::opencode_extract::{
    append_query, ensure_success, normalize_path, parse_model_selector, parse_response_body,
};
use super::opencode_server::OpenCodeProvider;

#[derive(Debug)]
struct RawResponse {
    status: u16,
    path: String,
    body: Value,
}

impl OpenCodeProvider {
    fn is_html_payload(value: &Value) -> bool {
        let Some(text) = value.as_str() else {
            return false;
        };

        let normalized = text.trim_start().to_ascii_lowercase();
        normalized.starts_with("<!doctype html") || normalized.starts_with("<html")
    }

    async fn raw_request(
        &self,
        method: Method,
        path: &str,
        query: &[(String, String)],
        body: Option<Value>,
    ) -> Result<RawResponse> {
        let normalized_path = normalize_path(path);
        let mut url = format!("{}{}", self.base_url, normalized_path);
        append_query(&mut url, query);

        let mut request = self.http.request(method, url);

        if let Some((username, password)) = self.basic_auth.as_ref() {
            request = request.basic_auth(username, Some(password));
        }

        if let Some(body_value) = body {
            request = request.json(&body_value);
        }

        let response = request.send().await.with_context(|| {
            format!("OpenCode // HTTP // request failed (path={normalized_path})")
        })?;

        let status = response.status().as_u16();
        let response_text = response.text().await.with_context(|| {
            format!("OpenCode // HTTP // response read failed (path={normalized_path})")
        })?;

        let body = parse_response_body(response_text);
        Ok(RawResponse {
            status,
            path: normalized_path,
            body,
        })
    }

    pub(crate) async fn request_json_with_fallback(
        &self,
        method: Method,
        paths: &[&str],
        query: &[(String, String)],
        body: Option<Value>,
    ) -> Result<Value> {
        let mut first_error: Option<anyhow::Error> = None;

        for path in paths {
            let raw = self
                .raw_request(method.clone(), path, query, body.clone())
                .await?;

            match ensure_success(raw.status, &raw.path, raw.body) {
                Ok(value) => {
                    if Self::is_html_payload(&value) {
                        if first_error.is_none() {
                            first_error = Some(anyhow!(
                                "OpenCode // API // unexpected HTML payload (path={})",
                                path
                            ));
                        }
                        continue;
                    }

                    return Ok(value);
                }
                Err(error) => {
                    if first_error.is_none() {
                        first_error = Some(error);
                    }
                }
            }
        }

        if let Some(error) = first_error {
            return Err(error);
        }

        bail!("OpenCode // HTTP // all fallback paths returned 404 (paths={paths:?})")
    }

    pub async fn send_prompt_with_options(
        &self,
        directory: &str,
        session_id: &str,
        prompt: &str,
        model: Option<&str>,
        agent: Option<&str>,
        no_reply: bool,
    ) -> Result<()> {
        let trimmed = prompt.trim();
        if trimmed.is_empty() {
            bail!("OpenCode // Session // prompt cannot be empty");
        }

        let query = vec![("directory".to_string(), directory.to_string())];
        let mut body = json!({
            "parts": [{
                "type": "text",
                "text": trimmed,
            }],
        });

        if no_reply {
            body["noReply"] = Value::Bool(true);
        }

        if let Some(model) = model.and_then(parse_model_selector) {
            body["model"] = json!({
                "providerID": model.0,
                "modelID": model.1,
            });
        }

        if let Some(agent) = agent
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToString::to_string)
        {
            body["agent"] = Value::String(agent);
        }

        let path_prompt = format!("/session/{session_id}/prompt");
        let path_message = format!("/session/{session_id}/message");
        let _ = self
            .request_json_with_fallback(
                Method::POST,
                &[path_prompt.as_str(), path_message.as_str()],
                &query,
                Some(body),
            )
            .await?;

        Ok(())
    }
}
