//! Runtime debug domain contracts shared by the manager and product protocol.
//! These values contain no Workbench objects, transport envelopes or IO owners.
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DebugSourceAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_reference: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "kind",
    deny_unknown_fields
)]
pub enum DebugBreakpointLocation {
    Source {
        source: DebugSourceAddress,
        line: u32,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        column: Option<u32>,
    },
    Function {
        name: String,
    },
    Data {
        data_id: String,
        access_type: DebugDataAccess,
        adapter_type: String,
        can_persist: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        session_id: Option<String>,
    },
    Instruction {
        instruction_reference: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        offset: Option<i64>,
        session_id: String,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DebugDataAccess {
    Read,
    Write,
    ReadWrite,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DebugBreakpointIntent {
    pub id: String,
    pub enabled: bool,
    pub location: DebugBreakpointLocation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hit_condition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_message: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DebugBreakpointUnsupported {
    Conditional,
    HitConditional,
    LogPoint,
    Function,
    Data,
    Instruction,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugBreakpointBinding {
    pub intent: DebugBreakpointIntent,
    pub verified: bool,
    pub applicable: bool,
    pub pending: bool,
    pub removed: bool,
    pub created: bool,
    pub unsupported: Option<DebugBreakpointUnsupported>,
    pub message: Option<String>,
    pub protocol: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugBreakpointsSnapshot {
    pub revision: u64,
    pub version: u64,
    pub applied_version: u64,
    pub bindings: Vec<DebugBreakpointBinding>,
    pub error: Option<String>,
}

/// Selection is explicit per operation; a backend session has no globally
/// selected thread/frame. Each query is fenced to its captured inspection epoch.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "kind",
    deny_unknown_fields
)]
pub enum DebugInspection {
    Stack {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        thread_id: Option<i64>,
    },
    Frame {
        frame_id: i64,
    },
    Variables {
        variables_reference: u64,
    },
    Watch {
        expressions: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        frame_id: Option<i64>,
    },
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugWatchValue {
    pub expression: String,
    pub result: Option<Value>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugScopeVariables {
    pub scope: Value,
    pub variables: Vec<Value>,
    /// Other scopes remain descriptors until explicitly expanded by a consumer.
    pub variables_loaded: bool,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "kind"
)]
pub enum DebugInspectionValue {
    Stack {
        threads: Vec<Value>,
        frames: Vec<Value>,
        thread_id: Option<i64>,
    },
    Frame {
        scopes: Vec<DebugScopeVariables>,
    },
    Variables {
        variables: Vec<Value>,
    },
    Watch {
        values: Vec<DebugWatchValue>,
    },
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "state"
)]
pub enum DebugInspectionResult {
    Pending,
    Complete { value: DebugInspectionValue },
    Failed { message: String },
    Invalidated,
}

mod workspace;
pub use workspace::*;

#[cfg(test)]
#[path = "contract_tests.rs"]
mod tests;

/// The desired live exception selection and the last completed DAP application.
#[derive(Clone, Debug, Default, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugExceptionsSnapshot {
    pub version: u64,
    pub applied_version: u64,
    /// Latest normalized preference. noDebug suppresses its wire application,
    /// without erasing the choice saved for a later debugging launch.
    pub filters: Vec<String>,
    /// Preference confirmed by the latest successful configuration response.
    pub applied_filters: Vec<String>,
    pub error: Option<String>,
}

mod session;
pub use session::*;
