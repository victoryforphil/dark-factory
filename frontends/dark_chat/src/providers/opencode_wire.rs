use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SessionWire {
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) title: Option<String>,
    #[serde(default, alias = "parentID", alias = "parent_id")]
    pub(crate) parent_id: Option<String>,
    #[serde(default)]
    pub(crate) updated_at: Option<String>,
    #[serde(default)]
    pub(crate) time: SessionTimeWire,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SessionTimeWire {
    #[serde(default)]
    pub(crate) updated: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct MessageWire {
    pub(crate) info: MessageInfoWire,
    #[serde(default)]
    pub(crate) parts: Vec<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MessageInfoWire {
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) role: String,
    #[serde(default)]
    pub(crate) created_at: Option<String>,
    #[serde(default)]
    pub(crate) time: MessageTimeWire,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MessageTimeWire {
    #[serde(default)]
    pub(crate) created: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::SessionWire;

    #[test]
    fn session_wire_reads_parent_id_from_parent_id_key() {
        let payload = serde_json::json!({
            "id": "ses_child",
            "parent_id": "ses_parent"
        });
        let parsed: SessionWire =
            serde_json::from_value(payload).expect("session wire should parse");

        assert_eq!(parsed.parent_id.as_deref(), Some("ses_parent"));
    }

    #[test]
    fn session_wire_reads_parent_id_from_parent_id_caps_key() {
        let payload = serde_json::json!({
            "id": "ses_child",
            "parentID": "ses_parent"
        });
        let parsed: SessionWire =
            serde_json::from_value(payload).expect("session wire should parse");

        assert_eq!(parsed.parent_id.as_deref(), Some("ses_parent"));
    }
}
