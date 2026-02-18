use std::collections::BTreeSet;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Default)]
pub(crate) struct ProductMetrics {
    pub(crate) variant_total: usize,
    pub(crate) variant_dirty: usize,
    pub(crate) variant_drift: usize,
    pub(crate) variants_with_git: usize,
    pub(crate) branches: BTreeSet<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiListEnvelope<T> {
    pub(crate) data: Option<Vec<T>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProductRecord {
    pub(crate) id: String,
    pub(crate) locator: String,
    #[serde(default)]
    pub(crate) workspace_locator: Option<String>,
    #[serde(default)]
    pub(crate) display_name: Option<String>,
    #[serde(default)]
    pub(crate) updated_at: String,
    #[serde(default)]
    pub(crate) git_info: Option<ProductGitInfoRecord>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProductGitInfoRecord {
    #[serde(default)]
    pub(crate) repo_name: Option<String>,
    #[serde(default)]
    pub(crate) branch: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VariantRecord {
    pub(crate) id: String,
    pub(crate) product_id: String,
    pub(crate) locator: String,
    #[serde(default)]
    pub(crate) name: Option<String>,
    #[serde(default)]
    pub(crate) updated_at: String,
    #[serde(default)]
    pub(crate) git_info_last_polled_at: Option<String>,
    #[serde(default)]
    pub(crate) git_info: Option<VariantGitInfoRecord>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VariantGitInfoRecord {
    #[serde(default)]
    pub(crate) branch: Option<String>,
    #[serde(default)]
    pub(crate) is_linked_worktree: Option<bool>,
    #[serde(default)]
    pub(crate) status: Option<VariantGitStatusRecord>,
    #[serde(default, rename = "_clone")]
    pub(crate) clone: Option<VariantCloneStatusRecord>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VariantCloneStatusRecord {
    #[serde(default)]
    pub(crate) status: Option<String>,
    #[serde(default)]
    pub(crate) phase: Option<String>,
    #[serde(default)]
    pub(crate) last_line: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VariantGitStatusRecord {
    #[serde(default)]
    pub(crate) clean: Option<bool>,
    #[serde(default)]
    pub(crate) ahead: Option<u64>,
    #[serde(default)]
    pub(crate) behind: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ActorRecord {
    pub(crate) id: String,
    pub(crate) variant_id: String,
    pub(crate) provider: String,
    #[serde(default)]
    pub(crate) provider_session_id: Option<String>,
    pub(crate) status: String,
    pub(crate) working_locator: String,
    #[serde(default)]
    pub(crate) connection_info: Option<Value>,
    #[serde(default)]
    pub(crate) title: Option<String>,
    #[serde(default)]
    pub(crate) description: Option<String>,
    #[serde(default)]
    pub(crate) sub_agents: Option<Vec<Value>>,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
}
