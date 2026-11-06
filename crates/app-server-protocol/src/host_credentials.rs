//! Private native-host credentials channel. These frames are never renderer JSON-RPC.

use crate::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Deserialize, JsonSchema, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum HostCredentialRequest {
    Authorize { request_id: String, context: String },
    Recover { request_id: String },
    Release { request_id: String },
}

#[derive(Deserialize, JsonSchema, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum HostCredentialResponse {
    Ready {
        version: u32,
        context: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_scope: Option<String>,
    },
    Changed {
        context: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        cache_scope: Option<String>,
    },
    Authorized {
        headers: BTreeMap<String, String>,
    },
    Recovered {
        headers: Option<BTreeMap<String, String>>,
    },
    Released,
    Failure {
        reason: HostCredentialFailure,
    },
}

#[derive(Clone, Copy, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum HostCredentialFailure {
    Unavailable,
    AccountChanged,
    InvalidRequest,
}
