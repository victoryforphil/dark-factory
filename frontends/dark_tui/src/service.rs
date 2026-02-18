use std::path::Path;

use anyhow::{Context, Result, anyhow};
use dark_chat::providers::{ChatProvider, OpenCodeProvider};
use dark_rust::{DarkCoreClient, LocatorId, LocatorKind, RawApiResponse};
use serde_json::{Value, json};
use tokio::task::JoinSet;

use crate::models::{
    ActorChatMessageRow, ActorRow, DashboardSnapshot, SshHostRow, SshPortForwardRow, TmuxSessionRow,
};
use crate::service_convert::{
    actor_opencode_context, collect_product_metrics, directory_name, ensure_success, now_label,
    query_slice_or_none, required_actor_opencode_context, summarize_error, to_actor_row,
    to_product_row, to_variant_row,
};
use crate::service_wire::{
    ActorRecord, ApiListEnvelope, ProductRecord, SshInfoEnvelope, VariantRecord,
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

#[derive(Debug, Clone, Default)]
pub struct CloneVariantOptions {
    pub name: Option<String>,
    pub target_path: Option<String>,
    pub branch_name: Option<String>,
    pub clone_type: Option<String>,
    pub source_variant_id: Option<String>,
    pub run_async: bool,
}

#[derive(Debug, Clone)]
pub struct SshInfo {
    pub hosts: Vec<SshHostRow>,
    pub port_forwards: Vec<SshPortForwardRow>,
    pub active_forwards: Vec<TmuxSessionRow>,
    pub tmux_sessions: Vec<TmuxSessionRow>,
}

impl DashboardService {
    pub async fn new(base_url: String, directory: String, poll_variants: bool) -> Self {
        Self {
            api: DarkCoreClient::new(base_url),
            directory,
            poll_variants,
        }
    }

    pub fn uses_realtime_transport(&self) -> bool {
        false
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
                rows.sort_by(|left, right| {
                    left.title
                        .to_ascii_lowercase()
                        .cmp(&right.title.to_ascii_lowercase())
                        .then_with(|| left.id.cmp(&right.id))
                });

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

    pub async fn switch_variant_branch(
        &self,
        variant_id: &str,
        branch_name: &str,
    ) -> Result<String> {
        let response = self
            .request(
                "POST",
                &format!("/variants/{variant_id}/branch"),
                None,
                Some(json!({ "branchName": branch_name })),
            )
            .await?;
        let _ = ensure_success(response)?;

        Ok(format!(
            "Variant branch switched: {variant_id} -> {branch_name}"
        ))
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

    pub async fn poll_actor(&self, actor_id: &str) -> Result<String> {
        let response = self
            .request("POST", &format!("/actors/{actor_id}/poll"), None, None)
            .await?;
        let _ = ensure_success(response)?;

        Ok(format!("Actor polled: {actor_id}"))
    }

    pub async fn move_actor(
        &self,
        actor_id: &str,
        source_variant_id: &str,
        target_variant_id: &str,
        target_variant_name: &str,
    ) -> Result<String> {
        if source_variant_id == target_variant_id {
            return Err(anyhow!(
                "Dark TUI // Actors // Move skipped: actor already on variant {target_variant_id}"
            ));
        }

        let actor = self.fetch_actor_row(actor_id).await.with_context(|| {
            format!(
                "Dark TUI // Actors // Unable to read source actor before move (actorId={actor_id})"
            )
        })?;

        let create_response = self
            .request(
                "POST",
                "/actors/",
                None,
                Some(json!({
                    "variantId": target_variant_id,
                    "provider": actor.provider,
                    "title": normalize_actor_optional_text(&actor.title),
                    "description": normalize_actor_optional_text(&actor.description),
                })),
            )
            .await?;
        let create_body = ensure_success(create_response)?;
        let replacement_actor_id = create_body
            .get("data")
            .and_then(|value| value.get("id"))
            .and_then(Value::as_str)
            .context("Dark TUI // Actors // Missing replacement actor id during move")?
            .to_string();

        let terminate_query = [("terminate".to_string(), "true".to_string())];
        let delete_terminated = self
            .request(
                "DELETE",
                &format!("/actors/{actor_id}"),
                Some(&terminate_query),
                None,
            )
            .await;

        match delete_terminated {
            Ok(response) => {
                if ensure_success(response).is_err() {
                    let fallback = self
                        .request("DELETE", &format!("/actors/{actor_id}"), None, None)
                        .await?;
                    let _ = ensure_success(fallback)?;
                }
            }
            Err(_) => {
                let fallback = self
                    .request("DELETE", &format!("/actors/{actor_id}"), None, None)
                    .await?;
                let _ = ensure_success(fallback)?;
            }
        }

        Ok(format!(
            "Moved actor {actor_id}: {source_variant_id} -> {target_variant_name} ({target_variant_id}) as {replacement_actor_id}"
        ))
    }

    pub async fn clone_product_variant(
        &self,
        product_id: &str,
        options: &CloneVariantOptions,
    ) -> Result<String> {
        let mut payload = serde_json::Map::<String, Value>::new();
        if let Some(name) = options.name.clone() {
            payload.insert("name".to_string(), Value::String(name));
        }
        if let Some(target_path) = options.target_path.clone() {
            payload.insert("targetPath".to_string(), Value::String(target_path));
        }
        if let Some(branch_name) = options.branch_name.clone() {
            payload.insert("branchName".to_string(), Value::String(branch_name));
        }
        if let Some(clone_type) = options.clone_type.clone() {
            payload.insert("cloneType".to_string(), Value::String(clone_type));
        }
        if let Some(source_variant_id) = options.source_variant_id.clone() {
            payload.insert(
                "sourceVariantId".to_string(),
                Value::String(source_variant_id),
            );
        }
        payload.insert("runAsync".to_string(), Value::Bool(options.run_async));

        let response = self
            .request(
                "POST",
                &format!("/products/{product_id}/variants/clone"),
                None,
                Some(Value::Object(payload)),
            )
            .await?;
        let body = ensure_success(response)?;

        let variant = body
            .get("data")
            .and_then(|value| value.get("variant"))
            .context("Dark TUI // Clone // Missing variant payload")?;
        let variant_id = variant.get("id").and_then(Value::as_str).unwrap_or("-");
        let variant_name = variant.get("name").and_then(Value::as_str).unwrap_or("-");
        let target_path = body
            .get("data")
            .and_then(|value| value.get("clone"))
            .and_then(|value| value.get("targetPath"))
            .and_then(Value::as_str)
            .unwrap_or("-");

        let async_label = body
            .get("data")
            .and_then(|value| value.get("clone"))
            .and_then(|value| value.get("isAsync"))
            .and_then(Value::as_bool)
            .unwrap_or(false);

        if async_label {
            Ok(format!(
                "Clone queued for {product_id}: {variant_id} ({variant_name}) -> {target_path}"
            ))
        } else {
            Ok(format!(
                "Cloned variant for {product_id}: {variant_id} ({variant_name}) -> {target_path}"
            ))
        }
    }

    pub async fn delete_variant(&self, variant_id: &str, dry: bool) -> Result<String> {
        let query = [(
            "dry".to_string(),
            if dry { "true" } else { "false" }.to_string(),
        )];
        let response = self
            .request(
                "DELETE",
                &format!("/variants/{variant_id}"),
                Some(&query),
                None,
            )
            .await?;
        let _ = ensure_success(response)?;

        if dry {
            Ok(format!(
                "Deleted variant {variant_id}; clone directory kept (dry=true)."
            ))
        } else {
            Ok(format!(
                "Deleted variant {variant_id}; clone directory removed (dry=false)."
            ))
        }
    }

    pub async fn init_product(&self, directory: &str) -> Result<String> {
        let locator = LocatorId::from_host_path(Path::new(directory), LocatorKind::Local)
            .map(|parsed| parsed.to_locator_id())?;
        let display_name = Some(directory_name(directory));

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
        variant_id: &str,
        provider: &str,
        initial_prompt: Option<&str>,
    ) -> Result<String> {
        let response = self
            .request(
                "POST",
                "/actors/",
                None,
                Some(json!({
                    "variantId": variant_id,
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

    pub async fn fetch_ssh_info(&self) -> Result<SshInfo> {
        let response = self.request("GET", "/system/ssh", None, None).await?;
        let body = ensure_success(response)?;
        let payload: SshInfoEnvelope =
            serde_json::from_value(body).context("Dark TUI // SSH // Unable to decode SSH info")?;
        let data = payload
            .data
            .context("Dark TUI // SSH // Missing SSH info data")?;

        let hosts = data
            .hosts
            .into_iter()
            .map(|host| SshHostRow {
                key: host.key,
                host: host.host,
                source: host.source,
                label: host.label,
                user: host.user.unwrap_or_else(|| "-".to_string()),
                port: host
                    .port
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                default_path: host.default_path.unwrap_or_else(|| "-".to_string()),
            })
            .collect::<Vec<_>>();

        let port_forwards = data
            .port_forwards
            .into_iter()
            .map(|forward| SshPortForwardRow {
                name: forward.name,
                host: forward.host.unwrap_or_else(|| "-".to_string()),
                local_port: forward.local_port,
                remote_port: forward.remote_port,
                remote_host: forward.remote_host,
                description: forward.description.unwrap_or_else(|| "-".to_string()),
            })
            .collect::<Vec<_>>();

        let active_forwards = data
            .active_forwards
            .into_iter()
            .map(|session| TmuxSessionRow {
                name: session.name,
                attached: session.attached,
                windows: session.windows,
                current_command: session.current_command,
            })
            .collect::<Vec<_>>();

        let tmux_sessions = data
            .tmux_sessions
            .into_iter()
            .map(|session| TmuxSessionRow {
                name: session.name,
                attached: session.attached,
                windows: session.windows,
                current_command: session.current_command,
            })
            .collect::<Vec<_>>();

        Ok(SshInfo {
            hosts,
            port_forwards,
            active_forwards,
            tmux_sessions,
        })
    }

    pub async fn start_ssh_port_forward(&self, preset_name: &str) -> Result<String> {
        let response = self
            .request(
                "POST",
                "/system/ssh/port-forward",
                None,
                Some(json!({
                    "presetName": preset_name,
                })),
            )
            .await?;
        let body = ensure_success(response)?;
        let data = body
            .get("data")
            .context("Dark TUI // SSH // Missing port-forward data")?;
        let session_name = data
            .get("sessionName")
            .and_then(Value::as_str)
            .unwrap_or("-");
        let host = data.get("host").and_then(Value::as_str).unwrap_or("-");
        let already_running = data
            .get("alreadyRunning")
            .and_then(Value::as_bool)
            .unwrap_or(false);

        if already_running {
            Ok(format!(
                "SSH forward already running: {preset_name} on {host} ({session_name})"
            ))
        } else {
            Ok(format!(
                "SSH forward started: {preset_name} on {host} ({session_name})"
            ))
        }
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

    pub async fn fetch_actor_last_message_previews(
        &self,
        actors: &[ActorRow],
        n_last_messages: u32,
    ) -> Vec<(String, String)> {
        let mut set = JoinSet::new();

        for actor in actors {
            let actor = actor.clone();
            let service = self.clone();
            set.spawn(async move {
                let messages = service
                    .fetch_actor_messages(&actor, Some(n_last_messages))
                    .await
                    .ok()?;

                let last = messages.iter().rev().find_map(|message| {
                    let text = message.text.trim();
                    if text.is_empty() {
                        None
                    } else {
                        Some(text.to_string())
                    }
                })?;

                Some((actor.id, last))
            });
        }

        let mut previews = Vec::new();
        while let Some(joined) = set.join_next().await {
            if let Ok(Some(item)) = joined {
                previews.push(item);
            }
        }

        previews
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
            return Err(anyhow!("Dark TUI // Actors // Prompt cannot be empty"));
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

    pub async fn fetch_actor_chat_options(
        &self,
        actor: &ActorRow,
    ) -> Result<(Vec<String>, Vec<String>)> {
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

fn normalize_actor_optional_text(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed == "-" {
        return None;
    }

    Some(trimmed.to_string())
}
