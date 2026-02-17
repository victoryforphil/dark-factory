use std::collections::HashMap;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use dark_rust::{DarkRustError, RawApiResponse};
use serde_json::Value;

use crate::models::{compact_timestamp, ActorRow, ProductRow, VariantRow};
use crate::service_wire::{ActorRecord, ProductMetrics, ProductRecord, VariantRecord};

pub(crate) fn collect_product_metrics(variants: &[VariantRow]) -> HashMap<String, ProductMetrics> {
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

        let branch = variant.branch.trim();
        if branch != "-" && !branch.is_empty() {
            entry.branches.insert(branch.to_string());
        }
    }

    metrics
}

pub(crate) fn to_product_row(record: ProductRecord, metrics: ProductMetrics) -> ProductRow {
    let display_name = record
        .display_name
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| locator_tail(&record.locator));

    let branch = record
        .git_info
        .as_ref()
        .and_then(|git| git.branch.clone())
        .unwrap_or_else(|| "-".to_string());
    let mut branches = metrics.branches;
    if branch != "-" {
        branches.insert(branch.clone());
    }

    let branches_label = if branches.is_empty() {
        "-".to_string()
    } else {
        branches.into_iter().collect::<Vec<_>>().join(", ")
    };

    let is_git_repo = record.locator.starts_with("@git://");
    let product_type = if is_git_repo {
        "git"
    } else if record.locator.starts_with("@local://") {
        "local"
    } else {
        "unknown"
    };

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
        workspace_locator: record.workspace_locator.unwrap_or_else(|| "-".to_string()),
        product_type: product_type.to_string(),
        is_git_repo,
        branch,
        branches: branches_label,
        repo_name,
        updated_at: compact_timestamp(&record.updated_at),
        status: status.to_string(),
        variant_total: metrics.variant_total,
        variant_dirty: metrics.variant_dirty,
        variant_drift: metrics.variant_drift,
    }
}

pub(crate) fn to_variant_row(record: VariantRecord) -> VariantRow {
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

pub(crate) fn to_actor_row(record: ActorRecord) -> ActorRow {
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
pub(crate) struct ActorOpenCodeContext {
    pub(crate) base_url: String,
    pub(crate) directory: String,
    pub(crate) session_id: String,
}

pub(crate) fn actor_opencode_context(actor: &ActorRow) -> Option<ActorOpenCodeContext> {
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

pub(crate) fn required_actor_opencode_context(
    actor: &ActorRow,
    action: &str,
) -> Result<ActorOpenCodeContext> {
    actor_opencode_context(actor).ok_or_else(|| {
        anyhow!(
            "Dark TUI // Chat // Cannot {action} for actor {}: direct OpenCode session connection is required",
            actor.id
        )
    })
}

pub(crate) fn query_slice_or_none(query: &[(String, String)]) -> Option<&[(String, String)]> {
    if query.is_empty() {
        None
    } else {
        Some(query)
    }
}

pub(crate) fn ensure_success(response: RawApiResponse) -> Result<Value> {
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

pub(crate) fn directory_name(directory: &str) -> String {
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

pub(crate) fn summarize_error(error: &anyhow::Error) -> String {
    let message = error.to_string();
    let max_len = 72;

    if message.len() <= max_len {
        return message;
    }

    format!("{}...", &message[..max_len])
}

pub(crate) fn now_label() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    format!("unix:{seconds}")
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::service_wire::{ProductGitInfoRecord, VariantGitInfoRecord, VariantGitStatusRecord};

    #[test]
    fn collect_product_metrics_aggregates_variant_counts() {
        let variants = vec![
            VariantRow {
                id: "var_1".to_string(),
                product_id: "prd_1".to_string(),
                locator: "@local:///repo".to_string(),
                name: "default".to_string(),
                branch: "main".to_string(),
                git_state: "dirty".to_string(),
                has_git: true,
                is_dirty: true,
                ahead: 0,
                behind: 0,
                worktree: "main".to_string(),
                last_polled_at: "-".to_string(),
                updated_at: "unix:1".to_string(),
            },
            VariantRow {
                id: "var_2".to_string(),
                product_id: "prd_1".to_string(),
                locator: "@local:///repo".to_string(),
                name: "exp".to_string(),
                branch: "feature".to_string(),
                git_state: "drift".to_string(),
                has_git: true,
                is_dirty: false,
                ahead: 2,
                behind: 1,
                worktree: "linked".to_string(),
                last_polled_at: "-".to_string(),
                updated_at: "unix:2".to_string(),
            },
        ];

        let metrics = collect_product_metrics(&variants);
        let prd = metrics.get("prd_1").expect("metrics should exist");

        assert_eq!(prd.variant_total, 2);
        assert_eq!(prd.variant_dirty, 1);
        assert_eq!(prd.variant_drift, 1);
        assert_eq!(prd.variants_with_git, 2);
    }

    #[test]
    fn to_product_row_prefers_locator_tail_when_display_missing() {
        let record = ProductRecord {
            id: "prd_1".to_string(),
            locator: "@local:///workspace/dark-factory".to_string(),
            workspace_locator: Some("@local:///workspace".to_string()),
            display_name: None,
            updated_at: "unix:123".to_string(),
            git_info: Some(ProductGitInfoRecord {
                repo_name: Some("dark-factory".to_string()),
                branch: Some("main".to_string()),
            }),
        };
        let metrics = ProductMetrics {
            variant_total: 1,
            variant_dirty: 0,
            variant_drift: 0,
            variants_with_git: 1,
            branches: Default::default(),
        };

        let row = to_product_row(record, metrics);
        assert_eq!(row.display_name, "dark-factory");
        assert_eq!(row.status, "clean");
    }

    #[test]
    fn to_variant_row_maps_clean_and_drift_states() {
        let record = VariantRecord {
            id: "var_1".to_string(),
            product_id: "prd_1".to_string(),
            locator: "@local:///workspace".to_string(),
            name: Some("default".to_string()),
            updated_at: "unix:123".to_string(),
            git_info_last_polled_at: Some("unix:120".to_string()),
            git_info: Some(VariantGitInfoRecord {
                branch: Some("main".to_string()),
                is_linked_worktree: Some(false),
                status: Some(VariantGitStatusRecord {
                    clean: Some(true),
                    ahead: Some(3),
                    behind: Some(1),
                }),
            }),
        };

        let row = to_variant_row(record);
        assert_eq!(row.git_state, "drift");
        assert_eq!(row.ahead, 3);
        assert_eq!(row.behind, 1);
    }

    #[test]
    fn to_actor_row_normalizes_directory_from_local_locator() {
        let row = to_actor_row(ActorRecord {
            id: "act_1".to_string(),
            variant_id: "var_1".to_string(),
            provider: "opencode/server".to_string(),
            provider_session_id: Some("ses_1".to_string()),
            status: "idle".to_string(),
            working_locator: "@local:///tmp/work".to_string(),
            connection_info: Some(json!({ "serverUrl": "http://localhost:4096" })),
            title: Some("Agent".to_string()),
            description: Some("hello".to_string()),
            created_at: "unix:10".to_string(),
            updated_at: "unix:11".to_string(),
        });

        assert_eq!(row.directory, "/tmp/work");
    }
}
