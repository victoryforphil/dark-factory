use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantProductConnectInput {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantProductRelationInput {
    pub connect: VariantProductConnectInput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantCreateInput {
    pub locator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub product: VariantProductRelationInput,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VariantUpdateInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct VariantListQuery {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
    pub product_id: Option<String>,
    pub locator: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantGitStatus {
    pub clean: bool,
    pub staged: u64,
    pub unstaged: u64,
    pub untracked: u64,
    pub conflicted: u64,
    pub ignored: u64,
    pub upstream: Option<String>,
    pub ahead: u64,
    pub behind: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantGitWorktree {
    pub path: String,
    pub branch: Option<String>,
    pub head: Option<String>,
    pub bare: bool,
    pub detached: bool,
    pub locked: bool,
    pub prunable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantGitInfo {
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
    pub status: VariantGitStatus,
    pub worktrees: Vec<VariantGitWorktree>,
    #[serde(rename = "scannedAt")]
    pub scanned_at: String,
}
