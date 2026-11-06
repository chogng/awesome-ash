use crate::JsonSchema;
use crate::protocol::common::CommandId;
use crate::protocol::common::SessionId;
use crate::protocol::common::ThreadId;
use crate::protocol::common::TurnId;
use serde::Deserialize;
use serde::Serialize;

/// Stable identity of one Turn/repository change set.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct ChangeSetId(pub String);

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TurnChangeCaptureStateDto {
    Open,
    Sealed,
    Incomplete,
    Discarded,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TurnChangeMessageStateDto {
    Unconfigured,
    Queued,
    Generating,
    Ready,
    Failed,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TurnChangeCommitStateDto {
    PartiallyCommitted,
    Idle,
    Queued,
    Committing,
    Committed,
    Conflict,
    Failed,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TurnChangeTerminalStateDto {
    Completed,
    Failed,
    Interrupted,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TurnChangeFileKindDto {
    Added,
    Modified,
    Deleted,
    Renamed,
    TypeChanged,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangeFileDto {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_path: Option<String>,
    pub kind: TurnChangeFileKindDto,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_mode: Option<String>,
    pub binary: bool,
    pub additions: u64,
    pub deletions: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangeFileStatisticsDto {
    pub files: u64,
    pub additions: u64,
    pub deletions: u64,
}

/// Public binding for a Thread directory. Managed filesystem paths stay private to App Server.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadDirBinding {
    pub managed_worktree_id: String,
    pub source_dir_id: String,
    pub repositories: Vec<ThreadWorktreeRepositoryBindingDto>,
    pub baseline_summary: String,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadWorktreeRepositoryBindingDto {
    pub repository_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline_object_id: Option<String>,
}

/// Small record used by Session Inspector lists and notifications.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangeSetSummary {
    pub change_set_id: ChangeSetId,
    pub session_id: SessionId,
    pub thread_id: ThreadId,
    pub turn_id: TurnId,
    pub repository_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_branch: Option<String>,
    pub statistics: TurnChangeFileStatisticsDto,
    pub capture_state: TurnChangeCaptureStateDto,
    pub message_state: TurnChangeMessageStateDto,
    pub commit_state: TurnChangeCommitStateDto,
    pub committed_paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal_state: Option<TurnChangeTerminalStateDto>,
    pub dependencies: Vec<ChangeSetId>,
    pub external_dependency_paths: Vec<String>,
    pub warnings: Vec<String>,
    pub conflict_paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    pub revision: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangesListParams {
    pub session_id: SessionId,
    pub thread_id: ThreadId,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangesListResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dir: Option<ThreadDirBinding>,
    pub change_sets: Vec<TurnChangeSetSummary>,
}

/// Net changes at the current history cursor, relative to this Thread's directory baseline.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangesSummaryResult {
    pub sequence: u64,
    pub revision: String,
    pub statistics: TurnChangeFileStatisticsDto,
    pub repositories: Vec<TurnChangesRepositorySummary>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangesRepositorySummary {
    pub repository_id: String,
    pub files: Vec<TurnChangeFileDto>,
}

/// Resolves only a file in the requested summary revision; stale revisions fail explicitly.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangesSummaryFileParams {
    pub session_id: SessionId,
    pub thread_id: ThreadId,
    pub revision: String,
    pub repository_id: String,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangesReadParams {
    pub session_id: SessionId,
    pub thread_id: ThreadId,
    pub change_set_id: ChangeSetId,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangesReadResult {
    pub summary: TurnChangeSetSummary,
    pub files: Vec<TurnChangeFileDto>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_message: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangesReadFileParams {
    pub session_id: SessionId,
    pub thread_id: ThreadId,
    pub change_set_id: ChangeSetId,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangesReadFileResult {
    pub path: String,
    pub binary: bool,
    pub truncated: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangesMutationParams {
    pub command_id: CommandId,
    pub session_id: SessionId,
    pub thread_id: ThreadId,
    pub change_set_id: ChangeSetId,
    pub expected_revision: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangesUpdateDraftParams {
    pub command_id: CommandId,
    pub session_id: SessionId,
    pub thread_id: ThreadId,
    pub change_set_id: ChangeSetId,
    pub expected_revision: u64,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TurnChangesCommitParams {
    pub command_id: CommandId,
    pub session_id: SessionId,
    pub thread_id: ThreadId,
    pub commit_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangesDiscardThreadParams {
    pub command_id: CommandId,
    pub session_id: SessionId,
    pub thread_id: ThreadId,
    pub expected_revision: u64,
    pub confirmed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangesMutationResult {
    pub change_sets: Vec<TurnChangeSetSummary>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangesChanged {
    pub session_id: SessionId,
    pub thread_id: ThreadId,
    pub change_sets: Vec<TurnChangeSetSummary>,
}

/// Paths identify immutable file changes within one saved Turn, including both rename sides.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TurnChangeSelectionDto {
    pub change_set_id: ChangeSetId,
    pub expected_revision: u64,
    pub paths: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TurnChangesPrepareCommitParams {
    pub command_id: CommandId,
    pub session_id: SessionId,
    pub thread_id: ThreadId,
    pub selections: Vec<TurnChangeSelectionDto>,
    pub message: String,
}

/// Exact final diff against the frozen target; submitting this ID cannot rebuild the selection.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnChangesPrepareCommitResult {
    pub commit_id: String,
    pub target_branch: String,
    pub message: String,
    pub files: Vec<TurnChangeFileDto>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TurnChangesReadCommitFileParams {
    pub session_id: SessionId,
    pub thread_id: ThreadId,
    pub commit_id: String,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TurnChangesReadCommitParams {
    pub session_id: SessionId,
    pub thread_id: ThreadId,
    pub commit_id: String,
}
