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
