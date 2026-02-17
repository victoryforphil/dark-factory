pub mod client;
pub mod error;
pub mod locator_id;
pub mod runtime;
pub mod types;
pub mod ws_client;

pub use client::{DarkCoreClient, RawApiResponse};
pub use error::DarkRustError;
pub use locator_id::{LocalLocator, LocatorId, LocatorKind};
pub use runtime::{
    DarkCoreLaunchConfig, EnsureDarkCoreState, ensure_dark_core_in_tmux_if_needed,
    is_local_dark_core_url,
};
pub use types::{
    ProductGitInfo, SystemResetDatabaseData, SystemResetDatabaseDeletedRows, VariantGitInfo,
    VariantGitStatus, VariantGitWorktree,
};
pub use ws_client::{DarkCoreWsClient, DarkCoreWsEvent};
