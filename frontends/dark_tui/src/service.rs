use std::collections::HashMap;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, anyhow};
use dark_chat::providers::{ChatProvider, OpenCodeProvider};
use dark_rust::{
    DarkCoreClient, DarkCoreWsClient, DarkRustError, LocatorId, LocatorKind, RawApiResponse,
};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::models::{
    ActorChatMessageRow, ActorRow, DashboardSnapshot, ProductRow, VariantRow, compact_timestamp,
};

const PAGE_LIMIT: u32 = 100;

#[derive(Debug, Clone)]
pub struct DashboardService {
    api: DarkCoreClient,
    ws_api: Option<DarkCoreWsClient>,
    directory: String,
    poll_variants: bool,
}

#[derive(Debug, Clone)]
pub struct SpawnOptions {
    pub providers: Vec<String>,
    pub default_provider: Option<String>,
}

impl DashboardService {
    pub async fn new(base_url: String, directory: String, poll_variants: bool) -> Self {
        let ws_api = DarkCoreWsClient::connect(base_url.clone()).await.ok();

        Self {
            api: DarkCoreClient::new(base_url),
            ws_api,
            directory,
            poll_variants,
        }
    }

    pub fn uses_realtime_transport(&self) -> bool {
        self.ws_api.is_some()
    }

    pub fn directory(&self) -> &str {
        &self.directory
    }

    pub async fn consume_route_mutation_events(&self) -> usize {
        let Some(ws_api) = &self.ws_api else {
            return 0;
        };

        match ws_api.drain_events().await {
            Ok(events) => events
                .into_iter()
                .filter(|event| event.event == "routes.mutated")
                .count(),
            Err(_) => 0,
        }
    }

    pub async fn fetch_snapshot(&self) -> Result<DashboardSnapshot> {
        let products_future = self.fetch_all_products();
        let variants_future = self.fetch_all_variants();
        let actors_future = self.fetch_all_actors();

        let (products_result, variants_result, actors_result) =
            tokio::join!(products_future, variants_future, actors_future);

        let product_records = products_result?;
        let variant_rows = variants_result?
            .into_iter()
            .map(to_variant_row)
            .collect::<Vec<_>>();

        let product_metrics = collect_product_metrics(&variant_rows);

        let mut product_rows = product_records
            .into_iter()
            .map(|record| {
                let metrics = product_metrics
                    .get(record.id.as_str())
                    .cloned()
                    .unwrap_or_default();
                to_product_row(record, metrics)
            })
            .collect::<Vec<_>>();
        product_rows.sort_by(|left, right| left.display_name.cmp(&right.display_name));

        let (actors, runtime_status) = match actors_result {
            Ok(records) => {
                let mut rows = records.into_iter().map(to_actor_row).collect::<Vec<_>>();
                rows.sort_by(|left, right| right.updated_at.cmp(&left.updated_at));

                let status = format!("actors online ({})", rows.len());
                (rows, status)
            }
            Err(error) => {
                let short = summarize_error(&error);
                (Vec::new(), format!("actors offline ({short})"))
            }
        };

        Ok(DashboardSnapshot {
            products: product_rows,
            variants: variant_rows,
            actors,
            runtime_status,
            last_updated: now_label(),
        })
    }

    pub async fn poll_variant(&self, variant_id: &str) -> Result<String> {
        let query = [("poll".to_string(), "true".to_string())];
        let response = self
            .request(
                "POST",
                &format!("/variants/{variant_id}/poll"),
                Some(&query),
                None,
            )
            .await?;
        let _ = ensure_success(response)?;

        Ok(format!("Variant polled: {variant_id}"))
    }

    pub async fn import_variant_actors(
        &self,
        variant_id: &str,
        provider: Option<&str>,
    ) -> Result<String> {
        let request_body = match provider {
            Some(provider) => json!({ "provider": provider }),
            None => json!({}),
        };

        let response = self
            .request(
                "POST",
                &format!("/variants/{variant_id}/actors/import"),
                None,
                Some(request_body),
            )
            .await?;
        let body = ensure_success(response)?;

        let import_data = body
            .get("data")
            .context("Dark TUI // Actors // Missing import result data")?;
        let provider_label = import_data
            .get("provider")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        let discovered = import_data
            .get("discovered")
            .and_then(Value::as_u64)
            .unwrap_or_default();
        let created = import_data
            .get("created")
            .and_then(Value::as_u64)
            .unwrap_or_default();
        let updated = import_data
            .get("updated")
            .and_then(Value::as_u64)
            .unwrap_or_default();

        Ok(format!(
            "Imported actors for {variant_id}: provider={provider_label}, discovered={discovered}, created={created}, updated={updated}"
        ))
    }

    pub async fn init_product(&self) -> Result<String> {
        let locator = LocatorId::from_host_path(Path::new(&self.directory), LocatorKind::Local)
            .map(|parsed| parsed.to_locator_id())?;
        let display_name = Some(directory_name(&self.directory));

        let response = self
            .request(
                "POST",
                "/products/",
                None,
                Some(json!({
                    "locator": locator,
                    "displayName": display_name,
                })),
            )
            .await?;
        let body = ensure_success(response)?;

        let product_id = body
            .get("data")
            .and_then(|value| value.get("id"))
            .and_then(Value::as_str)
            .context("Dark TUI // Init Product // Missing id in response")?;

        Ok(format!("Product initialized: {product_id}"))
    }

    pub async fn fetch_spawn_options(&self) -> Result<SpawnOptions> {
        let response = self.request("GET", "/system/providers", None, None).await?;
        let body = ensure_success(response)?;

        let default_provider = body
            .get("data")
            .and_then(|value| value.get("defaultProvider"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        let mut providers = body
            .get("data")
            .and_then(|value| value.get("enabledProviders"))
            .and_then(Value::as_array)
            .map(|values| {
                values
                    .iter()
                    .filter_map(Value::as_str)
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        if let Some(default) = default_provider.clone() {
            if !providers.iter().any(|provider| provider == &default) {
                providers.push(default);
            }
        }

        if providers.is_empty() {
            return Err(anyhow::anyhow!(
                "Dark TUI // Actors // No enabled providers configured"
            ));
        }

        Ok(SpawnOptions {
            providers,
            default_provider,
        })
    }

    pub async fn create_session(
        &self,
        provider: &str,
        initial_prompt: Option<&str>,
    ) -> Result<String> {
        let variants = self.fetch_all_variants().await?;
        let default_variant = variants
            .into_iter()
            .find(|variant| {
                variant.locator.ends_with(&self.directory)
                    || variant.name.as_deref() == Some("default")
            })
            .context("Dark TUI // Actors // No variant available to spawn actor")?;

        let response = self
            .request(
                "POST",
                "/actors/",
                None,
                Some(json!({
                    "variantId": default_variant.id,
                    "provider": provider,
                    "title": format!("Dark TUI // {}", directory_name(&self.directory)),
                    "description": "Spawned from dark_tui",
                })),
            )
            .await?;
        let body = ensure_success(response)?;

        let actor_id = body
            .get("data")
            .and_then(|value| value.get("id"))
            .and_then(Value::as_str)
            .context("Dark TUI // Actors // Missing actor id in response")?
            .to_string();

        if let Some(prompt) = initial_prompt
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            let actor = self.fetch_actor_row(&actor_id).await?;
            self.send_actor_prompt(&actor, prompt, None, None)
                .await
                .with_context(|| {
                    format!(
                        "Dark TUI // Chat // Failed to send initial prompt for actor {actor_id}"
                    )
                })?;
        }

        Ok(actor_id)
    }

    pub async fn fetch_actor_messages(
        &self,
        actor: &ActorRow,
        n_last_messages: Option<u32>,
    ) -> Result<Vec<ActorChatMessageRow>> {
        let context = required_actor_opencode_context(actor, "fetch messages")?;
        let provider = OpenCodeProvider::new(context.base_url);
        let messages = provider
            .list_messages(&context.directory, &context.session_id, n_last_messages)
            .await
            .context("Dark TUI // Chat // Failed to fetch OpenCode session messages")?;

        Ok(messages
            .into_iter()
            .map(|message| ActorChatMessageRow {
                role: message.role,
                text: message.text,
                created_at: message.created_at.unwrap_or_else(|| "-".to_string()),
            })
            .collect())
    }

    pub async fn send_actor_prompt(
        &self,
        actor: &ActorRow,
        prompt: &str,
        model: Option<&str>,
        agent: Option<&str>,
    ) -> Result<()> {
        let trimmed = prompt.trim();
        if trimmed.is_empty() {
            return Err(anyhow!(
                "Dark TUI // Actors // Prompt cannot be empty"
            ));
        }

        let context = required_actor_opencode_context(actor, "send prompt")?;
        let provider = OpenCodeProvider::new(context.base_url);
        provider
            .send_prompt_with_options(
                &context.directory,
                &context.session_id,
                trimmed,
                model,
                agent,
                false,
            )
            .await
            .context("Dark TUI // Chat // Failed to send OpenCode session prompt")?;

        Ok(())
    }

    pub async fn fetch_actor_chat_options(&self, actor: &ActorRow) -> Result<(Vec<String>, Vec<String>)> {
        if let Some(context) = actor_opencode_context(actor) {
            let provider = OpenCodeProvider::new(context.base_url);
            let models = provider
                .list_models(&context.directory)
                .await
                .unwrap_or_default();
            let agents = provider
                .list_agents(&context.directory)
                .await
                .unwrap_or_default();
            return Ok((models, agents));
        }

        Ok((Vec::new(), Vec::new()))
    }

    pub async fn build_attach_command(&self, session_id: &str) -> Result<String> {
        let response = self
            .request("GET", &format!("/actors/{session_id}/attach"), None, None)
            .await?;
        let body = ensure_success(response)?;
        let attach_data = body
            .get("data")
            .context("Dark TUI // Actors // Missing data envelope in attach response")?;

        let command = attach_data
            .get("command")
            .or_else(|| attach_data.get("attachCommand"))
            .and_then(Value::as_str)
            .with_context(|| {
                format!(
                    "Dark TUI // Actors // Missing attach command in response (data={attach_data})"
                )
            })?;

        Ok(command.to_string())
    }

    async fn request(
        &self,
        method: &str,
        path: &str,
        query: Option<&[(String, String)]>,
        body: Option<Value>,
    ) -> Result<RawApiResponse> {
        if let Some(ws_api) = &self.ws_api {
            if let Ok(response) = ws_api.request_raw(method, path, query, body.clone()).await {
                return Ok(response);
            }
        }

        self.api
            .request_raw(method, path, query, body)
            .await
            .map_err(Into::into)
    }

    async fn fetch_all_products(&self) -> Result<Vec<ProductRecord>> {
        let mut cursor: Option<String> = None;
        let mut products: Vec<ProductRecord> = Vec::new();

        loop {
            let mut query = vec![("limit".to_string(), PAGE_LIMIT.to_string())];
            if let Some(cursor) = cursor.clone() {
                query.push(("cursor".to_string(), cursor));
            }

            let response = self
                .request("GET", "/products/", query_slice_or_none(&query), None)
                .await?;

            let body = ensure_success(response)?;
            let payload: ApiListEnvelope<ProductRecord> = serde_json::from_value(body)
                .context("Dark TUI // Products // Unable to decode product list")?;

            let batch = payload.data.unwrap_or_default();
            let page_len = batch.len();
            if page_len == 0 {
                break;
            }

            cursor = batch.last().map(|product| product.id.clone());
            products.extend(batch.into_iter());

            if page_len < PAGE_LIMIT as usize || cursor.is_none() {
                break;
            }
        }

        Ok(products)
    }

    async fn fetch_all_variants(&self) -> Result<Vec<VariantRecord>> {
        let mut cursor: Option<String> = None;
        let mut variants: Vec<VariantRecord> = Vec::new();

        loop {
            let mut query = vec![
                ("limit".to_string(), PAGE_LIMIT.to_string()),
                ("poll".to_string(), self.poll_variants.to_string()),
            ];
            if let Some(cursor) = cursor.clone() {
                query.push(("cursor".to_string(), cursor));
            }

            let response = self
                .request("GET", "/variants/", query_slice_or_none(&query), None)
                .await?;

            let body = ensure_success(response)?;
            let payload: ApiListEnvelope<VariantRecord> = serde_json::from_value(body)
                .context("Dark TUI // Variants // Unable to decode variant list")?;

            let batch = payload.data.unwrap_or_default();
            let page_len = batch.len();
            if page_len == 0 {
                break;
            }

            cursor = batch.last().map(|variant| variant.id.clone());
            variants.extend(batch.into_iter());

            if page_len < PAGE_LIMIT as usize || cursor.is_none() {
                break;
            }
        }

        Ok(variants)
    }

    async fn fetch_all_actors(&self) -> Result<Vec<ActorRecord>> {
        let mut cursor: Option<String> = None;
        let mut actors: Vec<ActorRecord> = Vec::new();

        loop {
            let mut query = vec![("limit".to_string(), PAGE_LIMIT.to_string())];
            if let Some(cursor) = cursor.clone() {
                query.push(("cursor".to_string(), cursor));
            }

            let response = self
                .request("GET", "/actors/", query_slice_or_none(&query), None)
                .await?;

            let body = ensure_success(response)?;
            let payload: ApiListEnvelope<ActorRecord> = serde_json::from_value(body)
                .context("Dark TUI // Actors // Unable to decode actor list")?;

            let batch = payload.data.unwrap_or_default();
            let page_len = batch.len();
            if page_len == 0 {
                break;
            }

            cursor = batch.last().map(|actor| actor.id.clone());
            actors.extend(batch.into_iter());

            if page_len < PAGE_LIMIT as usize || cursor.is_none() {
                break;
            }
        }

        Ok(actors)
    }

    async fn fetch_actor_row(&self, actor_id: &str) -> Result<ActorRow> {
        self.fetch_all_actors()
            .await?
            .into_iter()
            .find(|actor| actor.id == actor_id)
            .map(to_actor_row)
            .with_context(|| format!("Dark TUI // Actors // Actor not found: {actor_id}"))
    }
}

#[derive(Debug, Clone, Default)]
struct ProductMetrics {
    variant_total: usize,
    variant_dirty: usize,
    variant_drift: usize,
    variants_with_git: usize,
}

fn collect_product_metrics(variants: &[VariantRow]) -> HashMap<String, ProductMetrics> {
    let mut metrics = HashMap::<String, ProductMetrics>::new();

    for variant in variants {
        let entry = metrics.entry(variant.product_id.clone()).or_default();
        entry.variant_total += 1;

        if variant.is_dirty {
            entry.variant_dirty += 1;
        }

        if variant.ahead > 0 || variant.behind > 0 {
            entry.variant_drift += 1;
        }

        if variant.has_git {
            entry.variants_with_git += 1;
        }
    }

    metrics
}

fn to_product_row(record: ProductRecord, metrics: ProductMetrics) -> ProductRow {
    let display_name = record
        .display_name
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| locator_tail(&record.locator));

    let branch = record
        .git_info
        .as_ref()
        .and_then(|git| git.branch.clone())
        .unwrap_or_else(|| "-".to_string());
    let repo_name = record
        .git_info
        .as_ref()
        .and_then(|git| git.repo_name.clone())
        .unwrap_or_else(|| "-".to_string());

    let status = if metrics.variant_total == 0 {
        "empty"
    } else if metrics.variant_dirty > 0 {
        "dirty"
    } else if metrics.variant_drift > 0 {
        "drift"
    } else if metrics.variants_with_git == 0 {
        "unknown"
    } else {
        "clean"
    };

    ProductRow {
        id: record.id,
        display_name,
        locator: record.locator,
        branch,
        repo_name,
        updated_at: compact_timestamp(&record.updated_at),
        status: status.to_string(),
        variant_total: metrics.variant_total,
        variant_dirty: metrics.variant_dirty,
        variant_drift: metrics.variant_drift,
    }
}

fn to_variant_row(record: VariantRecord) -> VariantRow {
    let git_info = record.git_info.unwrap_or_default();
    let status = git_info.status.unwrap_or_default();

    let ahead = status.ahead.unwrap_or(0);
    let behind = status.behind.unwrap_or(0);
    let clean = status.clean;

    let git_state = match clean {
        Some(false) => "dirty",
        Some(true) if ahead > 0 || behind > 0 => "drift",
        Some(true) => "clean",
        None => "unknown",
    }
    .to_string();

    let has_git = clean.is_some() || git_info.branch.is_some();
    let is_dirty = matches!(clean, Some(false));

    let worktree = match git_info.is_linked_worktree {
        Some(true) => "linked",
        Some(false) => "main",
        None => "-",
    }
    .to_string();

    VariantRow {
        id: record.id,
        product_id: record.product_id,
        locator: record.locator,
        name: record.name.unwrap_or_else(|| "default".to_string()),
        branch: git_info.branch.unwrap_or_else(|| "-".to_string()),
        git_state,
        has_git,
        is_dirty,
        ahead,
        behind,
        worktree,
        last_polled_at: record
            .git_info_last_polled_at
            .as_deref()
            .map(compact_timestamp)
            .unwrap_or_else(|| "-".to_string()),
        updated_at: compact_timestamp(&record.updated_at),
    }
}

fn to_actor_row(record: ActorRecord) -> ActorRow {
    ActorRow {
        id: record.id,
        variant_id: record.variant_id,
        title: record
            .title
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "Untitled actor".to_string()),
        description: record
            .description
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "-".to_string()),
        provider: record.provider,
        provider_session_id: record.provider_session_id,
        status: record.status,
        directory: record
            .working_locator
            .strip_prefix("@local://")
            .map(ToString::to_string)
            .unwrap_or(record.working_locator),
        connection_info: record.connection_info.unwrap_or(Value::Null),
        created_at: compact_timestamp(&record.created_at),
        updated_at: compact_timestamp(&record.updated_at),
    }
}

#[derive(Debug, Clone)]
struct ActorOpenCodeContext {
    base_url: String,
    directory: String,
    session_id: String,
}

fn actor_opencode_context(actor: &ActorRow) -> Option<ActorOpenCodeContext> {
    let provider = actor.provider.trim().to_ascii_lowercase();
    if !provider.starts_with("opencode") {
        return None;
    }

    let session_id = actor.provider_session_id.clone()?;
    let connection = actor.connection_info.as_object();

    let base_url = connection
        .and_then(|value| {
            value
                .get("serverUrl")
                .or_else(|| value.get("server_url"))
                .or_else(|| value.get("baseUrl"))
                .and_then(Value::as_str)
        })
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)?;

    let directory = connection
        .and_then(|value| value.get("directory").and_then(Value::as_str))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(|| actor.directory.clone());

    Some(ActorOpenCodeContext {
        base_url,
        directory,
        session_id,
    })
}

fn required_actor_opencode_context(actor: &ActorRow, action: &str) -> Result<ActorOpenCodeContext> {
    actor_opencode_context(actor).ok_or_else(|| {
        anyhow!(
            "Dark TUI // Chat // Cannot {action} for actor {}: direct OpenCode session connection is required",
            actor.id
        )
    })
}

fn query_slice_or_none(query: &[(String, String)]) -> Option<&[(String, String)]> {
    if query.is_empty() { None } else { Some(query) }
}

fn ensure_success(response: RawApiResponse) -> Result<Value> {
    if (200..300).contains(&response.status) {
        return Ok(response.body);
    }

    Err(DarkRustError::ApiStatus {
        status: response.status,
        path: response.path,
        body: response.body,
    }
    .into())
}

fn directory_name(directory: &str) -> String {
    Path::new(directory)
        .file_name()
        .and_then(|name| name.to_str())
        .map(ToString::to_string)
        .unwrap_or_else(|| directory.to_string())
}

fn locator_tail(locator: &str) -> String {
    locator
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(|| locator.to_string())
}

fn summarize_error(error: &anyhow::Error) -> String {
    let message = error.to_string();
    let max_len = 72;

    if message.len() <= max_len {
        return message;
    }

    format!("{}...", &message[..max_len])
}

fn now_label() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    format!("unix:{seconds}")
}

#[derive(Debug, Deserialize)]
struct ApiListEnvelope<T> {
    data: Option<Vec<T>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProductRecord {
    id: String,
    locator: String,
    #[serde(default)]
    display_name: Option<String>,
    #[serde(default)]
    updated_at: String,
    #[serde(default)]
    git_info: Option<ProductGitInfoRecord>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProductGitInfoRecord {
    #[serde(default)]
    repo_name: Option<String>,
    #[serde(default)]
    branch: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VariantRecord {
    id: String,
    product_id: String,
    locator: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    updated_at: String,
    #[serde(default)]
    git_info_last_polled_at: Option<String>,
    #[serde(default)]
    git_info: Option<VariantGitInfoRecord>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct VariantGitInfoRecord {
    #[serde(default)]
    branch: Option<String>,
    #[serde(default)]
    is_linked_worktree: Option<bool>,
    #[serde(default)]
    status: Option<VariantGitStatusRecord>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct VariantGitStatusRecord {
    #[serde(default)]
    clean: Option<bool>,
    #[serde(default)]
    ahead: Option<u64>,
    #[serde(default)]
    behind: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActorRecord {
    id: String,
    variant_id: String,
    provider: String,
    #[serde(default)]
    provider_session_id: Option<String>,
    status: String,
    working_locator: String,
    #[serde(default)]
    connection_info: Option<Value>,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    description: Option<String>,
    created_at: String,
    updated_at: String,
}
