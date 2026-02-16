use std::collections::HashMap;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use dark_rust::types::{
    ActorAttachQuery, ActorCreateInput, ActorListQuery, ActorMessage, ActorMessageInput,
    ActorMessagesQuery, ProductCreateInput, ProductListQuery, VariantImportActorsInput,
    VariantListQuery,
};
use dark_rust::{DarkCoreClient, DarkRustError, LocatorId, LocatorKind, RawApiResponse};
use serde::Deserialize;
use serde_json::Value;

use crate::models::{
    ActorChatMessageRow, ActorRow, DashboardSnapshot, ProductRow, VariantRow, compact_timestamp,
};

const PAGE_LIMIT: u32 = 100;

#[derive(Debug, Clone)]
pub struct DashboardService {
    api: DarkCoreClient,
    directory: String,
    poll_variants: bool,
}

#[derive(Debug, Clone)]
pub struct SpawnOptions {
    pub providers: Vec<String>,
    pub default_provider: Option<String>,
}

impl DashboardService {
    pub fn new(base_url: String, directory: String, poll_variants: bool) -> Self {
        Self {
            api: DarkCoreClient::new(base_url),
            directory,
            poll_variants,
        }
    }

    pub fn directory(&self) -> &str {
        &self.directory
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
        let response = self.api.variants_poll(variant_id, Some(true)).await?;
        ensure_success(response)?;

        Ok(format!("Variant polled: {variant_id}"))
    }

    pub async fn import_variant_actors(
        &self,
        variant_id: &str,
        provider: Option<&str>,
    ) -> Result<String> {
        let response = self
            .api
            .variants_import_actors(
                variant_id,
                &VariantImportActorsInput {
                    provider: provider.map(ToString::to_string),
                },
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
            .api
            .products_create(&ProductCreateInput {
                locator,
                display_name,
            })
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
        let response = self.api.system_providers().await?;
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
            .api
            .actors_create(&ActorCreateInput {
                variant_id: default_variant.id,
                provider: provider.to_string(),
                title: Some(format!("Dark TUI // {}", directory_name(&self.directory))),
                description: Some("Spawned from dark_tui".to_string()),
                metadata: None,
            })
            .await?;
        let body = ensure_success(response)?;

        let actor_id = body
            .get("data")
            .and_then(|value| value.get("id"))
            .and_then(Value::as_str)
            .context("Dark TUI // Actors // Missing actor id in response")?;

        if let Some(prompt) = initial_prompt
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            let response = self
                .api
                .actors_send_message(
                    actor_id,
                    &ActorMessageInput {
                        prompt: prompt.to_string(),
                        no_reply: Some(false),
                        model: None,
                        agent: None,
                    },
                )
                .await?;
            let _ = ensure_success(response)?;
        }

        Ok(actor_id.to_string())
    }

    pub async fn fetch_actor_messages(
        &self,
        actor_id: &str,
        n_last_messages: Option<u32>,
    ) -> Result<Vec<ActorChatMessageRow>> {
        let response = self
            .api
            .actors_list_messages(actor_id, &ActorMessagesQuery { n_last_messages })
            .await?;
        let body = ensure_success(response)?;

        let records: Vec<ActorMessage> = serde_json::from_value(
            body.get("data")
                .cloned()
                .unwrap_or(Value::Array(Vec::new())),
        )
        .context("Dark TUI // Actors // Unable to decode actor message list")?;

        Ok(records.into_iter().map(to_actor_chat_message_row).collect())
    }

    pub async fn send_actor_prompt(&self, actor_id: &str, prompt: &str) -> Result<()> {
        let trimmed = prompt.trim();
        if trimmed.is_empty() {
            return Err(anyhow::anyhow!(
                "Dark TUI // Actors // Prompt cannot be empty"
            ));
        }

        let response = self
            .api
            .actors_send_message(
                actor_id,
                &ActorMessageInput {
                    prompt: trimmed.to_string(),
                    no_reply: Some(false),
                    model: None,
                    agent: None,
                },
            )
            .await?;
        let _ = ensure_success(response)?;
        Ok(())
    }

    pub async fn build_attach_command(&self, session_id: &str) -> Result<String> {
        let response = self
            .api
            .actors_attach(
                session_id,
                &ActorAttachQuery {
                    model: None,
                    agent: None,
                },
            )
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

    async fn fetch_all_products(&self) -> Result<Vec<ProductRecord>> {
        let mut cursor: Option<String> = None;
        let mut products: Vec<ProductRecord> = Vec::new();

        loop {
            let response = self
                .api
                .products_list(&ProductListQuery {
                    cursor: cursor.clone(),
                    limit: Some(PAGE_LIMIT),
                    include: None,
                })
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
            let response = self
                .api
                .variants_list(&VariantListQuery {
                    cursor: cursor.clone(),
                    limit: Some(PAGE_LIMIT),
                    poll: Some(self.poll_variants),
                    ..VariantListQuery::default()
                })
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
            let response = self
                .api
                .actors_list(&ActorListQuery {
                    cursor: cursor.clone(),
                    limit: Some(PAGE_LIMIT),
                    provider: None,
                    ..ActorListQuery::default()
                })
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
        status: record.status,
        directory: record
            .working_locator
            .strip_prefix("@local://")
            .map(ToString::to_string)
            .unwrap_or(record.working_locator),
        created_at: compact_timestamp(&record.created_at),
        updated_at: compact_timestamp(&record.updated_at),
    }
}

fn to_actor_chat_message_row(record: ActorMessage) -> ActorChatMessageRow {
    let text = record
        .text
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "(no text content)".to_string());

    ActorChatMessageRow {
        role: record.role,
        text,
        created_at: compact_timestamp(&record.created_at),
    }
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
    status: String,
    working_locator: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    description: Option<String>,
    created_at: String,
    updated_at: String,
}
