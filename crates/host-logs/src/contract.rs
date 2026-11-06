//! Private, ordered host-helper frames; never part of the renderer RPC connection.

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(
    tag = "op",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum HostLogRequest {
    Initialize {
        logs_home: String,
        max_file_bytes: u32,
        retained_sessions: u32,
        process_id: u32,
    },
    Append {
        source: String,
        entry: HostLogEntry,
    },
    List {
        window_source: String,
        app_server_log: Option<String>,
    },
    Read {
        id: String,
        cursor: Option<HostLogCursor>,
    },
    CleanUp,
    Flush,
    Close,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum HostLogResponse {
    Initialized {
        version: u32,
        session_path: String,
    },
    Ok,
    Listed {
        files: Vec<HostLogFile>,
    },
    Read {
        result: HostLogReadResult,
    },
    Failure {
        reason: HostLogFailure,
        message: String,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum HostLogFailure {
    InvalidRequest,
    Io,
    NotOpen,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostLogEntry {
    pub timestamp_millis: i64,
    pub level: HostLogLevel,
    pub category: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum HostLogLevel {
    Trace,
    Debug,
    Information,
    Warning,
    Error,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostLogCursor {
    pub generation: String,

    pub offset: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostLogFile {
    pub id: String,
    pub source: String,
    pub format: HostLogFormat,
    pub live: bool,
    pub current_window: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum HostLogFormat {
    Jsonl,
    Text,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostLogReadResult {
    pub text: String,
    pub cursor: HostLogCursor,
    pub gap: bool,
}
