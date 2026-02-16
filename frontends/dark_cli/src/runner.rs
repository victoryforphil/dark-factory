use std::env;
use std::path::Path;

use anyhow::{Context, Result};
use serde_json::{json, Value};

use crate::api::{ApiClient, ApiResponse};
use crate::cli::{
  Cli, Command, OpencodeAction, OpencodeSessionsAction, ProductsAction, ServiceAction,
  SystemAction,
};
use crate::errors::DarkCliError;

pub async fn run(cli: Cli, api: &ApiClient) -> Result<()> {
  let response = dispatch(&cli, api).await?;
  let output = crate::output::render(cli.format, &response.body)?;

  if (200..300).contains(&response.status) {
    println!("{output}");
    return Ok(());
  }

  eprintln!("{output}");
  Err(DarkCliError::ApiStatus {
    status: response.status,
    path: response.path,
    body: response.body,
  }
  .into())
}

async fn dispatch(cli: &Cli, api: &ApiClient) -> Result<ApiResponse> {
  match &cli.command {
    Command::Init { path } => {
      let directory = resolve_directory(path.as_deref())?;
      let directory_name = directory_name(&directory)?;
      let locator = directory.to_string_lossy().to_string();

      api.post(
        "/products/",
        json!({
          "locator": locator,
          "displayName": directory_name,
        }),
      )
      .await
      .map_err(Into::into)
    }
    Command::Service(command) => match command.action {
      ServiceAction::Status => api.get("/", None).await.map_err(Into::into),
    },
    Command::System(command) => match command.action {
      SystemAction::Health => api.get("/system/health", None).await.map_err(Into::into),
      SystemAction::Info => api.get("/system/info", None).await.map_err(Into::into),
      SystemAction::Metrics => api.get("/system/metrics", None).await.map_err(Into::into),
    },
    Command::Products(command) => match &command.action {
      ProductsAction::List { cursor, limit } => {
        let mut query = Vec::new();

        if let Some(cursor_value) = cursor {
          query.push(("cursor".to_string(), cursor_value.clone()));
        }

        if let Some(limit_value) = limit {
          query.push(("limit".to_string(), limit_value.to_string()));
        }

        let query = if query.is_empty() { None } else { Some(query) };
        api.get("/products/", query).await.map_err(Into::into)
      }
      ProductsAction::Create {
        locator,
        display_name,
      } => {
        api.post(
          "/products/",
          json!({
            "locator": locator,
            "displayName": display_name,
          }),
        )
        .await
        .map_err(Into::into)
      }
    },
    Command::Opencode(command) => match &command.action {
      OpencodeAction::State { directory } => {
        let query = vec![("directory".to_string(), directory.clone())];
        api.get("/opencode/state", Some(query)).await.map_err(Into::into)
      }
      OpencodeAction::Sessions(sessions_command) => match &sessions_command.action {
        OpencodeSessionsAction::List { directory } => {
          let query = vec![("directory".to_string(), directory.clone())];
          api.get("/opencode/sessions", Some(query)).await.map_err(Into::into)
        }
        OpencodeSessionsAction::Create { directory, title } => {
          api.post(
            "/opencode/sessions",
            json!({
              "directory": directory,
              "title": title,
            }),
          )
          .await
          .map_err(Into::into)
        }
        OpencodeSessionsAction::Get {
          id,
          directory,
          include_messages,
        } => {
          let mut query = vec![("directory".to_string(), directory.clone())];
          if *include_messages {
            query.push(("includeMessages".to_string(), "true".to_string()));
          }

          api.get(&format!("/opencode/sessions/{id}"), Some(query))
            .await
            .map_err(Into::into)
        }
        OpencodeSessionsAction::Attach {
          id,
          directory,
          model,
          agent,
        } => {
          let mut query = vec![("directory".to_string(), directory.clone())];
          if let Some(model_value) = model {
            query.push(("model".to_string(), model_value.clone()));
          }
          if let Some(agent_value) = agent {
            query.push(("agent".to_string(), agent_value.clone()));
          }

          api.get(&format!("/opencode/sessions/{id}/attach"), Some(query))
            .await
            .map_err(Into::into)
        }
        OpencodeSessionsAction::Command {
          id,
          directory,
          command,
        } => {
          api.post(
            &format!("/opencode/sessions/{id}/command"),
            json!({
              "directory": directory,
              "command": command,
            }),
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
          let mut payload = json!({
            "directory": directory,
            "prompt": prompt,
          });

          if *no_reply {
            payload["noReply"] = Value::Bool(true);
          }

          api.post(&format!("/opencode/sessions/{id}/prompt"), payload)
            .await
            .map_err(Into::into)
        }
        OpencodeSessionsAction::Abort { id, directory } => {
          api.post(
            &format!("/opencode/sessions/{id}/abort"),
            json!({
              "directory": directory,
            }),
          )
          .await
          .map_err(Into::into)
        }
        OpencodeSessionsAction::Delete { id, directory } => {
          let query = vec![("directory".to_string(), directory.clone())];
          api.delete(&format!("/opencode/sessions/{id}"), Some(query))
            .await
            .map_err(Into::into)
        }
      },
    },
  }
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

  absolute
    .canonicalize()
    .with_context(|| format!("Dark CLI // Init // Expected existing path (path={})", absolute.display()))
}

fn directory_name(path: &Path) -> Result<String> {
  path.file_name()
    .map(|name| name.to_string_lossy().to_string())
    .context("Dark CLI // Init // Unable to derive directory name")
}
