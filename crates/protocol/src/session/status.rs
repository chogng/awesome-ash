//! Session lifecycle summarized from its Threads, rather than stored independently.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

/// Lifecycle summarized from the Threads carrying one `session_id`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SessionStatus {
    Active,
    Archived,
}
