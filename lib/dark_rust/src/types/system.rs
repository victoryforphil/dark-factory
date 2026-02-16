use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemResetDatabaseDeletedRows {
    pub products: u64,
    pub variants: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemResetDatabaseData {
    #[serde(rename = "backupPath")]
    pub backup_path: String,
    #[serde(rename = "databasePath")]
    pub database_path: String,
    #[serde(rename = "deletedRows")]
    pub deleted_rows: SystemResetDatabaseDeletedRows,
    #[serde(rename = "resetAt")]
    pub reset_at: String,
}
