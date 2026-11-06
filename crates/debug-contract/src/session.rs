use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

/// Admitted before resolving an extension descriptor, so a parent's end also
/// revokes children which do not yet own a transport.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DebugSessionAdmission {
    pub id: String,
    pub workspace_id: String,
    pub parent_id: Option<String>,
    pub lifecycle_managed_by_parent: bool,
    pub merge_console: bool,
    pub no_debug: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugSessionRelation {
    pub id: String,
    pub parent_id: Option<String>,
    pub console_id: String,
    pub console_ids: Vec<String>,
    pub console_catalog_revision: u64,
    pub lifecycle_target_id: String,
    pub no_debug: Option<bool>,
    pub ended: bool,
    /// Descendants in release order, including prepared and already ended nodes.
    pub descendants: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DebugSessionAction {
    Read,
    Route,
    End,
}

#[derive(Clone, Copy, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DebugConsoleEntryKind {
    Output,
    Input,
    Evaluation,
    Group,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugConsoleEntry {
    pub id: u64,
    pub session_id: String,
    pub kind: DebugConsoleEntryKind,
    pub output: String,
    pub category: String,
    pub parent_id: Option<u64>,
    pub collapsed: bool,
    pub variables_reference: Option<u64>,
    pub inspection_revision: Option<u64>,
    pub source: Option<Value>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub data: Option<Value>,
}

/// Bounded, ordered runtime output. It survives transport release within this
/// connection and is never written to the workspace's durable definitions.
#[derive(Clone, Debug, Default, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugConsoleSnapshot {
    pub revision: u64,
    pub entries: Vec<DebugConsoleEntry>,
}
