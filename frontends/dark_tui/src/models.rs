#[derive(Debug, Clone, Default)]
pub struct DashboardSnapshot {
    pub products: Vec<ProductRow>,
    pub variants: Vec<VariantRow>,
    pub actors: Vec<ActorRow>,
    pub runtime_status: String,
    pub last_updated: String,
}

#[derive(Debug, Clone)]
pub struct ProductRow {
    pub id: String,
    pub display_name: String,
    pub locator: String,
    pub branch: String,
    pub repo_name: String,
    pub updated_at: String,
    pub status: String,
    pub variant_total: usize,
    pub variant_dirty: usize,
    pub variant_drift: usize,
}

#[derive(Debug, Clone)]
pub struct VariantRow {
    pub id: String,
    pub product_id: String,
    pub locator: String,
    pub name: String,
    pub branch: String,
    pub git_state: String,
    pub has_git: bool,
    pub is_dirty: bool,
    pub ahead: u64,
    pub behind: u64,
    pub worktree: String,
    pub last_polled_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct ActorRow {
    pub id: String,
    pub variant_id: String,
    pub title: String,
    pub description: String,
    pub provider: String,
    pub provider_session_id: Option<String>,
    pub status: String,
    pub directory: String,
    pub connection_info: serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct ActorChatMessageRow {
    pub role: String,
    pub text: String,
    pub created_at: String,
}

pub fn compact_id(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() <= 12 {
        return trimmed.to_string();
    }

    format!("{}...", &trimmed[..12])
}

pub fn compact_locator(value: &str, max_len: usize) -> String {
    let trimmed = value.trim();
    if trimmed.len() <= max_len {
        return trimmed.to_string();
    }

    if max_len <= 3 {
        return ".".repeat(max_len);
    }

    let suffix_len = max_len.saturating_sub(3);
    format!("...{}", &trimmed[trimmed.len() - suffix_len..])
}

pub fn compact_timestamp(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return "-".to_string();
    }

    if let Some((date, rest)) = trimmed.split_once('T') {
        let time = rest.trim_end_matches('Z').split('.').next().unwrap_or(rest);
        return format!("{date} {time}");
    }

    trimmed.to_string()
}
