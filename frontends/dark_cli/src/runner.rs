use std::env;
use std::path::Path;

use anyhow::{Context, Result};
use dark_rust::types::{
  OpencodeAttachQuery, OpencodeSessionCommandInput, OpencodeSessionCreateInput,
  OpencodeSessionDirectoryInput, OpencodeSessionPromptInput, OpencodeSessionStateQuery,
  ProductCreateInput, ProductListQuery, ProductUpdateInput, VariantCreateInput, VariantListQuery,
  VariantProductConnectInput, VariantProductRelationInput, VariantUpdateInput,
};
use dark_rust::{DarkCoreClient, DarkRustError, RawApiResponse};
use serde_json::{json, Value};

use crate::cli::{
  Cli, Command, OpencodeAction, OpencodeSessionsAction, ProductsAction, ServiceAction, SystemAction,
  VariantsAction,
};

const PRODUCTS_PAGE_LIMIT: u32 = 100;

pub async fn run(cli: Cli, api: &DarkCoreClient) -> Result<()> {
  let response = dispatch(&cli, api).await?;
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

async fn dispatch(cli: &Cli, api: &DarkCoreClient) -> Result<RawApiResponse> {
  match &cli.command {
    Command::Init { path } => {
      let directory = resolve_directory(path.as_deref())?;
      let directory_name = directory_name(&directory)?;
      let locator = directory.to_string_lossy().to_string();

      api
        .products_create(&ProductCreateInput {
          locator,
          display_name: Some(directory_name),
        })
        .await
        .map_err(Into::into)
    }
    Command::Service(command) => match command.action {
      ServiceAction::Status => api.service_status().await.map_err(Into::into),
    },
    Command::System(command) => match command.action {
      SystemAction::Health => api.system_health().await.map_err(Into::into),
      SystemAction::Info => api.system_info().await.map_err(Into::into),
      SystemAction::Metrics => api.system_metrics().await.map_err(Into::into),
      SystemAction::ResetDb => api.system_reset_db().await.map_err(Into::into),
    },
    Command::Products(command) => match &command.action {
      ProductsAction::List { cursor, limit } => {
        if cursor.is_none() && limit.is_none() {
          list_all_products(api).await
        } else {
          api
            .products_list(&ProductListQuery {
              cursor: cursor.clone(),
              limit: *limit,
            })
            .await
            .map_err(Into::into)
        }
      }
      ProductsAction::Create {
        locator,
        display_name,
      } => {
        api
          .products_create(&ProductCreateInput {
            locator: locator.clone(),
            display_name: display_name.clone(),
          })
            .await
            .map_err(Into::into)
      }
      ProductsAction::Get { id } => api.products_get(id).await.map_err(Into::into),
      ProductsAction::Update {
        id,
        locator,
        display_name,
      } => {
        api
          .products_update(
            id,
            &ProductUpdateInput {
              locator: locator.clone(),
              display_name: display_name.clone(),
            },
          )
          .await
          .map_err(Into::into)
      }
      ProductsAction::Delete { id } => api.products_delete(id).await.map_err(Into::into),
    },
    Command::Variants(command) => match &command.action {
      VariantsAction::List {
        cursor,
        limit,
        product_id,
        locator,
        name,
      } => {
        api
          .variants_list(&VariantListQuery {
            cursor: cursor.clone(),
            limit: *limit,
            product_id: product_id.clone(),
            locator: locator.clone(),
            name: name.clone(),
          })
          .await
          .map_err(Into::into)
      }
      VariantsAction::Create {
        locator,
        product_id,
        name,
      } => {
        api
          .variants_create(&VariantCreateInput {
            locator: locator.clone(),
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
      VariantsAction::Get { id } => api.variants_get(id).await.map_err(Into::into),
      VariantsAction::Update { id, locator, name } => {
        api
          .variants_update(
            id,
            &VariantUpdateInput {
              locator: locator.clone(),
              name: name.clone(),
            },
          )
          .await
          .map_err(Into::into)
      }
      VariantsAction::Delete { id } => api.variants_delete(id).await.map_err(Into::into),
    },
    Command::Opencode(command) => match &command.action {
      OpencodeAction::State { directory } => {
        api.opencode_state(directory).await.map_err(Into::into)
      }
      OpencodeAction::Sessions(sessions_command) => match &sessions_command.action {
        OpencodeSessionsAction::List { directory } => {
          api.opencode_sessions_list(directory).await.map_err(Into::into)
        }
        OpencodeSessionsAction::Create { directory, title } => {
          api
            .opencode_sessions_create(&OpencodeSessionCreateInput {
              directory: directory.clone(),
              title: title.clone(),
            })
            .await
            .map_err(Into::into)
        }
        OpencodeSessionsAction::Get {
          id,
          directory,
          include_messages,
        } => {
          api
            .opencode_sessions_get(
              id,
              &OpencodeSessionStateQuery {
                directory: directory.clone(),
                include_messages: *include_messages,
              },
            )
            .await
            .map_err(Into::into)
        }
        OpencodeSessionsAction::Attach {
          id,
          directory,
          model,
          agent,
        } => {
          api
            .opencode_sessions_attach(
              id,
              &OpencodeAttachQuery {
                directory: directory.clone(),
                model: model.clone(),
                agent: agent.clone(),
              },
            )
            .await
            .map_err(Into::into)
        }
        OpencodeSessionsAction::Command {
          id,
          directory,
          command,
        } => {
          api
            .opencode_sessions_command(
              id,
              &OpencodeSessionCommandInput {
                directory: directory.clone(),
                command: command.clone(),
              },
            )
            .await
            .map_err(Into::into)
        }
        OpencodeSessionsAction::Prompt {
          id,
          directory,
          prompt,
          no_reply,
        } => {
          api
            .opencode_sessions_prompt(
              id,
              &OpencodeSessionPromptInput {
                directory: directory.clone(),
                prompt: prompt.clone(),
                no_reply: if *no_reply { Some(true) } else { None },
              },
            )
            .await
            .map_err(Into::into)
        }
        OpencodeSessionsAction::Abort { id, directory } => {
          api
            .opencode_sessions_abort(
              id,
              &OpencodeSessionDirectoryInput {
                directory: directory.clone(),
              },
            )
            .await
            .map_err(Into::into)
        }
        OpencodeSessionsAction::Delete { id, directory } => {
          api
            .opencode_sessions_delete(id, directory)
            .await
            .map_err(Into::into)
        }
      },
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

fn resolve_directory(path: Option<&str>) -> Result<std::path::PathBuf> {
  let base_path = match path {
    Some(value) => std::path::PathBuf::from(value),
    None => env::current_dir().context("Dark CLI // Init // Failed to get current directory")?,
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
  path
    .file_name()
    .map(|name| name.to_string_lossy().to_string())
    .context("Dark CLI // Init // Unable to derive directory name")
}
