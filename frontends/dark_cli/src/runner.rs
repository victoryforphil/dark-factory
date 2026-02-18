use std::collections::BTreeSet;
use std::env;
use std::path::Path;
use std::process::Command as ProcessCommand;

use anyhow::{Context, Result};
use dark_rust::types::{
    ActorAttachQuery, ActorCommandInput, ActorCreateInput, ActorDeleteQuery, ActorListQuery,
    ActorMessageInput, ActorMessagesQuery, ActorUpdateInput, ProductCreateInput,
    ProductIncludeQuery, ProductListQuery, ProductUpdateInput, ProductVariantCloneInput,
    VariantBranchSwitchInput, VariantCreateInput, VariantDeleteQuery, VariantImportActorsInput,
    VariantListQuery, VariantProductConnectInput, VariantProductRelationInput, VariantUpdateInput,
};
use dark_rust::{DarkCoreClient, DarkRustError, LocatorId, LocatorKind, RawApiResponse};
use serde_json::{Value, json};

use crate::cli::{
    ActorMessagesAction, ActorsAction, Cli, Command, IncludeLevel, ProductsAction, ServiceAction,
    SystemAction, VariantsAction,
};

const PRODUCTS_PAGE_LIMIT: u32 = 100;

pub async fn run(cli: Cli, api: &DarkCoreClient) -> Result<()> {
    let response = dispatch(&cli, api).await?;

    if (200..300).contains(&response.status)
        && matches!(
            &cli.command,
            Command::Actors(crate::cli::ActorsCommand {
                action: ActorsAction::Attach { .. }
            })
        )
    {
        run_tmux_attach_from_response(&response.body)?;
        return Ok(());
    }

    let output = crate::output::render(cli.format, &cli.command, &response.body)?;

    if (200..300).contains(&response.status) {
        println!("{output}");
        return Ok(());
    }

    eprintln!("{output}");
    Err(DarkRustError::ApiStatus {
        status: response.status,
        path: response.path,
        body: response.body,
    }
    .into())
}

fn run_tmux_attach_from_response(body: &Value) -> Result<()> {
    let command = extract_attach_command(body)?;
    let session_name = parse_tmux_attach_target(command)?;

    let mut args = Vec::new();
    if env::var_os("TMUX").is_some() {
        args.push("switch-client");
    } else {
        args.push("attach-session");
    }
    args.push("-t");
    args.push(session_name.as_str());

    let status = ProcessCommand::new("tmux")
        .args(&args)
        .status()
        .with_context(|| format!("failed to execute tmux attach for session `{session_name}`"))?;
    if status.success() {
        return Ok(());
    }

    Err(anyhow::anyhow!(
        "tmux attach command failed for session `{session_name}` with status {status}"
    ))
}

fn extract_attach_command(body: &Value) -> Result<&str> {
    body.get("data")
        .and_then(|value| value.get("attachCommand"))
        .or_else(|| body.get("data").and_then(|value| value.get("command")))
        .and_then(Value::as_str)
        .with_context(|| format!("attach response missing attach command: {body}"))
}

fn parse_tmux_attach_target(command: &str) -> Result<String> {
    let tokens: Vec<&str> = command.split_whitespace().collect();
    if tokens.len() < 4 || tokens[0] != "tmux" {
        anyhow::bail!("attach command is not a tmux attach command: {command}");
    }

    if tokens[1] != "attach-session" && tokens[1] != "switch-client" {
        anyhow::bail!("unsupported tmux attach subcommand: {}", tokens[1]);
    }

    let mut iter = tokens.iter().copied();
    while let Some(token) = iter.next() {
        if token == "-t" {
            if let Some(session_name) = iter.next() {
                let trimmed = session_name.trim();
                if !trimmed.is_empty() {
                    return Ok(trimmed.to_string());
                }
            }
        }
    }

    anyhow::bail!("tmux attach command missing -t <session>: {command}")
}

async fn dispatch(cli: &Cli, api: &DarkCoreClient) -> Result<RawApiResponse> {
    match &cli.command {
        Command::Init { path } => {
            let directory = resolve_directory(path.as_deref())?;
            let directory_name = directory_name(&directory)?;
            let locator = LocatorId::from_host_path(directory.as_path(), LocatorKind::Local)
                .map(|parsed| parsed.to_locator_id())?;

            api.products_create(&ProductCreateInput {
                locator,
                display_name: Some(directory_name),
                workspace_locator: None,
            })
            .await
            .map_err(Into::into)
        }
        Command::Info { path } => info_for_directory(path.as_deref(), api, &cli.base_url).await,
        Command::Service(command) => match command.action {
            ServiceAction::Status => api.service_status().await.map_err(Into::into),
        },
        Command::System(command) => match command.action {
            SystemAction::Health => api.system_health().await.map_err(Into::into),
            SystemAction::Info => api.system_info().await.map_err(Into::into),
            SystemAction::Metrics => api.system_metrics().await.map_err(Into::into),
            SystemAction::Providers => api.system_providers().await.map_err(Into::into),
            SystemAction::ResetDb => api.system_reset_db().await.map_err(Into::into),
        },
        Command::Products(command) => match &command.action {
            ProductsAction::List {
                cursor,
                limit,
                include,
            } => {
                if cursor.is_none() && limit.is_none() && include.is_none() {
                    list_all_products(api).await
                } else {
                    api.products_list(&ProductListQuery {
                        cursor: cursor.clone(),
                        limit: *limit,
                        include: map_include(*include),
                    })
                    .await
                    .map_err(Into::into)
                }
            }
            ProductsAction::Create {
                locator,
                display_name,
                workspace_locator,
            } => {
                let locator = normalize_locator_input(locator)?;

                api.products_create(&ProductCreateInput {
                    locator,
                    display_name: display_name.clone(),
                    workspace_locator: workspace_locator
                        .as_deref()
                        .map(normalize_locator_input)
                        .transpose()?,
                })
                .await
                .map_err(Into::into)
            }
            ProductsAction::Get { id, include } => api
                .products_get(id, map_include(*include))
                .await
                .map_err(Into::into),
            ProductsAction::Update {
                id,
                locator,
                display_name,
                workspace_locator,
            } => api
                .products_update(
                    id,
                    &ProductUpdateInput {
                        locator: locator
                            .as_deref()
                            .map(normalize_locator_input)
                            .transpose()?,
                        display_name: display_name.clone(),
                        workspace_locator: workspace_locator
                            .as_deref()
                            .map(normalize_locator_input)
                            .transpose()?,
                    },
                )
                .await
                .map_err(Into::into),
            ProductsAction::Delete { id } => api.products_delete(id).await.map_err(Into::into),
            ProductsAction::Clone {
                product_id,
                name,
                target_path,
                branch_name,
                clone_type,
                source_variant_id,
            } => api
                .product_variants_clone(
                    product_id,
                    &ProductVariantCloneInput {
                        name: name.clone(),
                        target_path: target_path.clone(),
                        branch_name: branch_name.clone(),
                        clone_type: clone_type.clone(),
                        source_variant_id: source_variant_id.clone(),
                    },
                )
                .await
                .map_err(Into::into),
        },
        Command::Variants(command) => match &command.action {
            VariantsAction::List {
                cursor,
                limit,
                product_id,
                locator,
                name,
                poll,
            } => api
                .variants_list(&VariantListQuery {
                    cursor: cursor.clone(),
                    limit: *limit,
                    product_id: product_id.clone(),
                    locator: locator.clone(),
                    name: name.clone(),
                    poll: Some(*poll),
                })
                .await
                .map_err(Into::into),
            VariantsAction::Create {
                locator,
                product_id,
                name,
            } => {
                let locator = normalize_locator_input(locator)?;

                api.variants_create(&VariantCreateInput {
                    locator,
                    name: name.clone(),
                    product: VariantProductRelationInput {
                        connect: VariantProductConnectInput {
                            id: product_id.clone(),
                        },
                    },
                })
                .await
                .map_err(Into::into)
            }
            VariantsAction::Get { id, poll } => {
                api.variants_get(id, Some(*poll)).await.map_err(Into::into)
            }
            VariantsAction::Poll { id, poll } => {
                api.variants_poll(id, Some(*poll)).await.map_err(Into::into)
            }
            VariantsAction::ImportActors { id, provider } => api
                .variants_import_actors(
                    id,
                    &VariantImportActorsInput {
                        provider: provider.clone(),
                    },
                )
                .await
                .map_err(Into::into),
            VariantsAction::Update { id, locator, name } => api
                .variants_update(
                    id,
                    &VariantUpdateInput {
                        locator: locator
                            .as_deref()
                            .map(normalize_locator_input)
                            .transpose()?,
                        name: name.clone(),
                    },
                )
                .await
                .map_err(Into::into),
            VariantsAction::Delete { id } => api
                .variants_delete(id, &VariantDeleteQuery { dry: Some(true) })
                .await
                .map_err(Into::into),
            VariantsAction::Branch { id, branch_name } => api
                .variants_switch_branch(
                    id,
                    &VariantBranchSwitchInput {
                        branch_name: branch_name.clone(),
                    },
                )
                .await
                .map_err(Into::into),
        },
        Command::Actors(command) => match &command.action {
            ActorsAction::List {
                cursor,
                limit,
                variant_id,
                product_id,
                provider,
                status,
            } => api
                .actors_list(&ActorListQuery {
                    cursor: cursor.clone(),
                    limit: *limit,
                    variant_id: variant_id.clone(),
                    product_id: product_id.clone(),
                    provider: provider.clone(),
                    status: status.clone(),
                })
                .await
                .map_err(Into::into),
            ActorsAction::Create {
                variant_id,
                provider,
                title,
                description,
            } => {
                let resolved_provider =
                    resolve_provider_for_spawn(api, provider.as_deref()).await?;

                api.actors_create(&ActorCreateInput {
                    variant_id: variant_id.clone(),
                    provider: resolved_provider,
                    title: title.clone(),
                    description: description.clone(),
                    sub_agents: None,
                    metadata: None,
                })
                .await
                .map_err(Into::into)
            }
            ActorsAction::Get { id } => api.actors_get(id).await.map_err(Into::into),
            ActorsAction::Update {
                id,
                variant_id,
                title,
                description,
            } => api
                .actors_update(
                    id,
                    &ActorUpdateInput {
                        variant_id: variant_id.clone(),
                        title: title.clone(),
                        description: description.clone(),
                        sub_agents: None,
                        metadata: None,
                    },
                )
                .await
                .map_err(Into::into),
            ActorsAction::Delete { id, terminate } => api
                .actors_delete(
                    id,
                    &ActorDeleteQuery {
                        terminate: *terminate,
                    },
                )
                .await
                .map_err(Into::into),
            ActorsAction::Poll { id } => api.actors_poll(id).await.map_err(Into::into),
            ActorsAction::Attach { id, model, agent } => api
                .actors_attach(
                    id,
                    &ActorAttachQuery {
                        model: model.clone(),
                        agent: agent.clone(),
                    },
                )
                .await
                .map_err(Into::into),
            ActorsAction::Messages { action } => match action {
                ActorMessagesAction::Send {
                    id,
                    prompt,
                    no_reply,
                    model,
                    agent,
                } => api
                    .actors_send_message(
                        id,
                        &ActorMessageInput {
                            prompt: prompt.clone(),
                            no_reply: if *no_reply { Some(true) } else { None },
                            model: model.clone(),
                            agent: agent.clone(),
                        },
                    )
                    .await
                    .map_err(Into::into),
                ActorMessagesAction::List {
                    id,
                    n_last_messages,
                } => api
                    .actors_list_messages(
                        id,
                        &ActorMessagesQuery {
                            n_last_messages: *n_last_messages,
                        },
                    )
                    .await
                    .map_err(Into::into),
            },
            ActorsAction::Commands {
                id,
                command,
                args,
                model,
                agent,
            } => api
                .actors_run_command(
                    id,
                    &ActorCommandInput {
                        command: command.clone(),
                        args: args.clone(),
                        model: model.clone(),
                        agent: agent.clone(),
                    },
                )
                .await
                .map_err(Into::into),
        },
    }
}

async fn list_all_products(api: &DarkCoreClient) -> Result<RawApiResponse> {
    let mut cursor: Option<String> = None;
    let mut all_products: Vec<Value> = Vec::new();

    loop {
        let response = api
            .products_list(&ProductListQuery {
                cursor: cursor.clone(),
                limit: Some(PRODUCTS_PAGE_LIMIT),
                include: None,
            })
            .await?;

        if !(200..300).contains(&response.status) {
            return Ok(response);
        }

        let Some(batch) = response.body.get("data").and_then(Value::as_array) else {
            return Ok(response);
        };

        if batch.is_empty() {
            break;
        }

        let next_cursor = batch
            .last()
            .and_then(|row| row.get("id"))
            .and_then(Value::as_str)
            .map(ToString::to_string);

        all_products.extend(batch.iter().cloned());

        if batch.len() < PRODUCTS_PAGE_LIMIT as usize {
            break;
        }

        let Some(next_cursor) = next_cursor else {
            break;
        };

        cursor = Some(next_cursor);
    }

    Ok(RawApiResponse {
        status: 200,
        path: "/products/".to_string(),
        body: json!({
          "ok": true,
          "data": all_products,
        }),
    })
}

fn normalize_locator_input(locator: &str) -> Result<String> {
    let path = Path::new(locator);

    if path.is_absolute() {
        return LocatorId::from_host_path(path, LocatorKind::Local)
            .map(|parsed| parsed.to_locator_id())
            .map_err(Into::into);
    }

    LocatorId::parse(locator)
        .map(|parsed| parsed.to_locator_id())
        .map_err(Into::into)
}

fn map_include(include: Option<IncludeLevel>) -> Option<ProductIncludeQuery> {
    match include {
        Some(IncludeLevel::Minimal) => Some(ProductIncludeQuery::Minimal),
        Some(IncludeLevel::Full) => Some(ProductIncludeQuery::Full),
        None => None,
    }
}

fn resolve_directory(path: Option<&str>) -> Result<std::path::PathBuf> {
    let base_path = match path {
        Some(value) => std::path::PathBuf::from(value),
        None => {
            env::current_dir().context("Dark CLI // Init // Failed to get current directory")?
        }
    };

    let absolute = if base_path.is_absolute() {
        base_path
    } else {
        env::current_dir()
            .context("Dark CLI // Init // Failed to get current directory")?
            .join(base_path)
    };

    absolute.canonicalize().with_context(|| {
        format!(
            "Dark CLI // Init // Expected existing path (path={})",
            absolute.display()
        )
    })
}

fn directory_name(path: &Path) -> Result<String> {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .context("Dark CLI // Init // Unable to derive directory name")
}

async fn info_for_directory(
    path: Option<&str>,
    api: &DarkCoreClient,
    base_url: &str,
) -> Result<RawApiResponse> {
    let directory = resolve_directory(path)?;
    let locator = LocatorId::from_host_path(directory.as_path(), LocatorKind::Local)
        .map(|parsed| parsed.to_locator_id())?;

    let variants_response = api
        .variants_list(&VariantListQuery {
            locator: Some(locator.clone()),
            limit: Some(500),
            poll: Some(true),
            ..VariantListQuery::default()
        })
        .await
        .with_context(|| {
            format!(
                "Dark CLI // Info // Failed to reach dark_core (base_url={base_url}). Start dark_core and retry"
            )
        })?;

    if !(200..300).contains(&variants_response.status) {
        return Ok(variants_response);
    }

    let variants = extract_data_rows(&variants_response.body);
    let product_ids: BTreeSet<String> = variants
        .iter()
        .filter_map(|variant| variant.get("productId").and_then(Value::as_str))
        .map(ToString::to_string)
        .collect();

    let mut products: Vec<Value> = Vec::new();
    let mut product_variants: Vec<Value> = Vec::new();

    for product_id in product_ids {
        let product_response = api
            .products_get(&product_id, Some(ProductIncludeQuery::Full))
            .await?;

        if !(200..300).contains(&product_response.status) {
            return Ok(product_response);
        }

        if let Some(product) = product_response.body.get("data") {
            if let Some(included_variants) = product.get("variants").and_then(Value::as_array) {
                product_variants.extend(included_variants.iter().cloned());
            }
            products.push(product.clone());
        }
    }

    Ok(RawApiResponse {
        status: 200,
        path: "/info".to_string(),
        body: json!({
            "ok": true,
            "data": {
                "directory": directory.to_string_lossy().to_string(),
                "locator": locator,
                "products": products,
                "variants": product_variants,
            }
        }),
    })
}

fn extract_data_rows(body: &Value) -> Vec<Value> {
    body.get("data")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{extract_attach_command, parse_tmux_attach_target};

    #[test]
    fn extracts_attach_command_from_actor_attach_response() {
        let body = json!({
            "ok": true,
            "data": {
                "attachCommand": "tmux attach-session -t dark-opencode-server"
            }
        });

        let command = extract_attach_command(&body).expect("attach command should be present");
        assert_eq!(command, "tmux attach-session -t dark-opencode-server");
    }

    #[test]
    fn parses_tmux_attach_target_from_command() {
        let session = parse_tmux_attach_target("tmux attach-session -t dark-opencode-server")
            .expect("session should parse");
        assert_eq!(session, "dark-opencode-server");
    }

    #[test]
    fn rejects_non_tmux_attach_command() {
        let error = parse_tmux_attach_target("opencode --session abc")
            .expect_err("non tmux command should fail");
        assert!(error.to_string().contains("not a tmux attach command"));
    }
}

async fn resolve_provider_for_spawn(
    api: &DarkCoreClient,
    provider: Option<&str>,
) -> Result<String> {
    if let Some(value) = provider {
        return Ok(value.to_string());
    }

    let response = api.system_providers().await?;
    if !(200..300).contains(&response.status) {
        return Err(DarkRustError::ApiStatus {
            status: response.status,
            path: response.path,
            body: response.body,
        }
        .into());
    }

    let default_provider = response
        .body
        .get("data")
        .and_then(|value| value.get("defaultProvider"))
        .and_then(Value::as_str)
        .map(ToString::to_string);

    if let Some(default_provider) = default_provider {
        return Ok(default_provider);
    }

    let first_enabled = response
        .body
        .get("data")
        .and_then(|value| value.get("enabledProviders"))
        .and_then(Value::as_array)
        .and_then(|providers| providers.first())
        .and_then(Value::as_str)
        .map(ToString::to_string);

    first_enabled.context("Dark CLI // Actors // No configured providers available for spawn")
}
