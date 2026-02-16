use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ProductIncludeQuery {
    #[default]
    Minimal,
    Full,
}

impl ProductIncludeQuery {
    pub fn as_query_value(self) -> &'static str {
        match self {
            Self::Minimal => "minimal",
            Self::Full => "full",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCreateInput {
    pub locator: String,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ProductListQuery {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub include: Option<ProductIncludeQuery>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductGitInfo {
    #[serde(rename = "repoName")]
    pub repo_name: String,
    #[serde(rename = "remoteName")]
    pub remote_name: Option<String>,
    #[serde(rename = "remoteUrl")]
    pub remote_url: Option<String>,
    #[serde(rename = "authorName")]
    pub author_name: Option<String>,
    #[serde(rename = "authorEmail")]
    pub author_email: Option<String>,
    pub branch: Option<String>,
    pub commit: Option<String>,
    #[serde(rename = "repoRoot")]
    pub repo_root: String,
    #[serde(rename = "gitDir")]
    pub git_dir: String,
    #[serde(rename = "gitCommonDir")]
    pub git_common_dir: String,
    #[serde(rename = "isLinkedWorktree")]
    pub is_linked_worktree: bool,
    #[serde(rename = "worktreePath")]
    pub worktree_path: String,
    #[serde(rename = "worktreeCount")]
    pub worktree_count: u64,
    #[serde(rename = "scannedAt")]
    pub scanned_at: String,
}
