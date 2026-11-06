//! Thread lifecycle and archive reasons; Turn progress is tracked separately.

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ThreadStatus {
    Active,
    Archived,
}

/// User-visible reason that a Thread left the active Session catalog.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ThreadArchiveReason {
    #[default]
    Completed,
    Stopped,
}
