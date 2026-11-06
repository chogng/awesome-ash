use crate::AppServerListenInfo;
use crate::protocol::error::AppServerError;
use crate::protocol::registry::ClientRequestSchema;
use crate::protocol::registry::ClientResultSchema;
use crate::protocol::registry::HostRequestSchema;
use crate::protocol::registry::HostResultSchema;
use crate::protocol::registry::ServerNotificationSchema;
use crate::rpc::JsonRpcError;
use crate::rpc::JsonRpcFailure;
use crate::rpc::JsonRpcNotification;
use crate::rpc::JsonRpcRequest;
use crate::rpc::JsonRpcResponse;
use schemars::JsonSchema;
use schemars::Schema;
use schemars::schema_for;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;
use std::collections::BTreeMap;

/// Generates version and schema identity from the current Rust contract.
pub fn protocol_metadata() -> String {
    // Fixture bytes must not depend on whether another crate enables preserve_order.
    let metadata = BTreeMap::from([
        (
            "major",
            Value::from(crate::protocol::initialize::APP_SERVER_PROTOCOL_MAJOR),
        ),
        ("schemaHash", Value::from(schema_hash())),
    ]);
    let mut output =
        serde_json::to_string_pretty(&metadata).expect("protocol metadata must serialize as JSON");
    output.push('\n');
    output
}

#[allow(dead_code)]
#[derive(JsonSchema)]
#[schemars(title = "Ash App Server Protocol")]
#[serde(rename_all = "camelCase")]
struct ProtocolSchema {
    host_log_request: ash_host_logs::contract::HostLogRequest,
    host_log_response: ash_host_logs::contract::HostLogResponse,
    native_host_credential_request: crate::host_credentials::HostCredentialRequest,
    native_host_credential_response: crate::host_credentials::HostCredentialResponse,
    listen_info: AppServerListenInfo,
    web_listen_info: crate::WebListenInfo,
    remote_tunnel_host_info: crate::RemoteTunnelHostInfo,
    web_session_info: crate::WebSessionInfo,
    web_workspace_list_request: crate::WebWorkspaceListRequest,
    web_workspace_list_result: crate::WebWorkspaceListResult,
    web_workspace_open_request: crate::WebWorkspaceOpenRequest,
    web_workspace_state: crate::WebWorkspaceState,
    web_workspace_folder: crate::WebWorkspaceFolder,
    web_workspace_update_request: crate::WebWorkspaceUpdateRequest,
    client_request: JsonRpcRequest<ClientRequestSchema>,
    client_response: JsonRpcResponse<ClientResultSchema, AppServerError>,
    host_request: JsonRpcRequest<HostRequestSchema>,
    host_response: JsonRpcResponse<HostResultSchema, JsonRpcError>,
    server_notification: JsonRpcNotification<ServerNotificationSchema>,
    error_response: JsonRpcFailure<AppServerError>,
}

pub fn json_schema() -> String {
    let schema = protocol_schema_value();
    let mut output =
        serde_json::to_string_pretty(&schema).expect("protocol schema must serialize as JSON");
    output.push('\n');
    output
}

pub(crate) fn protocol_schema_value() -> Value {
    let mut schema =
        serde_json::to_value(protocol_schema()).expect("protocol schema must serialize as JSON");
    canonicalize_json(&mut schema);
    schema
}

fn protocol_schema() -> Schema {
    schema_for!(ProtocolSchema)
}

pub(crate) fn schema_hash() -> String {
    let canonical = serde_json::to_vec(&protocol_schema_value())
        .expect("canonical protocol schema must serialize as JSON");
    let digest = Sha256::digest(canonical);
    format!("sha256:{digest:x}")
}

fn canonicalize_json(value: &mut Value) {
    match value {
        Value::Array(values) => {
            for value in values {
                canonicalize_json(value);
            }
        }
        Value::Object(object) => {
            let mut entries = std::mem::take(object).into_iter().collect::<Vec<_>>();
            for (_, value) in &mut entries {
                canonicalize_json(value);
            }
            entries.sort_by(|left, right| left.0.cmp(&right.0));
            object.extend(entries);
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}
