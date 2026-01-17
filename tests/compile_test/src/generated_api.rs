use serde::{Serialize, Deserialize};
use oapi_universal_gen::*;
use std::future::Future;
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum PermissionAction {
    Allow,
    Deny,
    Ask,
}
///Log level
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum PermissionActionConfig {
    Ask,
    Allow,
    Deny,
}
///@deprecated Always uses stretch layout.
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum LayoutConfig {
    Auto,
    Stretch,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiPromptAppend {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiCommandExecute {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiToastShow {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiSessionSelect {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInstallationUpdated {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInstallationUpdateAvailable {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worktree: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sandboxes: Option<Vec<Option<String>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventProjectUpdated {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Project>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventServerInstanceDisposed {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFileEdited {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLspClientDiagnostics {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patterns: Option<Vec<Option<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub always: Option<Vec<Option<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPermissionAsked {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PermissionRequest>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPermissionReplied {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatus {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionIdle {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    ///Display text (1-5 words, concise)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    ///Explanation of choice
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionInfo {
    ///Complete question
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    ///Very short label (max 12 chars)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    ///Available choices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Option<QuestionOption>>>,
    ///Allow selecting multiple choices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiple: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    ///Questions to ask
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub questions: Option<Vec<Option<QuestionInfo>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventQuestionAsked {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuestionRequest>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionAnswer {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventQuestionReplied {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventQuestionRejected {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    ///Brief description of the task
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    ///Current status of the task: pending, in_progress, completed, cancelled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    ///Priority level of the task: high, medium, low
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    ///Unique identifier for the todo item
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTodoUpdated {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Option<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pid: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyCreated {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyUpdated {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyExited {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyDeleted {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMcpToolsChanged {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFileWatcherUpdated {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLspUpdated {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCommandExecuted {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventVcsBranchUpdated {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletions: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAuthError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageOutputLengthError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAbortedError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parentID")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelID")]
    pub model_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerID")]
    pub provider_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokens: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finish: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessageUpdated {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessageRemoved {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synthetic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningPart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePartSourceText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<FilePartSourceText>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<FilePartSourceText>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<FilePartSourceText>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientName")]
    pub client_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePartSource {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<FilePartSource>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStatePending {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStateRunning {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStateCompleted {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Option<FilePart>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStateError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolState {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "callID")]
    pub call_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ToolState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepStartPart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepFinishPart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokens: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotPart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchPart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<Option<String>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempt: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<APIError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactionPart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessagePartUpdated {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessagePartRemoved {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionCompacted {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<PermissionAction>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRuleset {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "projectID")]
    pub project_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parentID")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<PermissionRuleset>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revert: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionCreated {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionUpdated {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionDeleted {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionDiff {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionError {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventServerConnected {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGlobalDisposed {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<Event>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadRequestError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Option<serde_json::Value>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotFoundError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
///Custom keybind configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindsConfig {
    ///Leader key for keybind combinations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader: Option<String>,
    ///Exit the application
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_exit: Option<String>,
    ///Open external editor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editor_open: Option<String>,
    ///List available themes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme_list: Option<String>,
    ///Toggle sidebar
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sidebar_toggle: Option<String>,
    ///Toggle session scrollbar
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scrollbar_toggle: Option<String>,
    ///Toggle username visibility
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username_toggle: Option<String>,
    ///View status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_view: Option<String>,
    ///Export session to editor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_export: Option<String>,
    ///Create a new session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_new: Option<String>,
    ///List all sessions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_list: Option<String>,
    ///Show session timeline
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_timeline: Option<String>,
    ///Fork session from message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_fork: Option<String>,
    ///Rename session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_rename: Option<String>,
    ///Share current session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_share: Option<String>,
    ///Unshare current session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_unshare: Option<String>,
    ///Interrupt current session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_interrupt: Option<String>,
    ///Compact the session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_compact: Option<String>,
    ///Scroll messages up by one page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_page_up: Option<String>,
    ///Scroll messages down by one page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_page_down: Option<String>,
    ///Scroll messages up by half page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_half_page_up: Option<String>,
    ///Scroll messages down by half page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_half_page_down: Option<String>,
    ///Navigate to first message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_first: Option<String>,
    ///Navigate to last message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_last: Option<String>,
    ///Navigate to next message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_next: Option<String>,
    ///Navigate to previous message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_previous: Option<String>,
    ///Navigate to last user message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_last_user: Option<String>,
    ///Copy message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_copy: Option<String>,
    ///Undo message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_undo: Option<String>,
    ///Redo message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_redo: Option<String>,
    ///Toggle code block concealment in messages
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_toggle_conceal: Option<String>,
    ///Toggle tool details visibility
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_details: Option<String>,
    ///List available models
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_list: Option<String>,
    ///Next recently used model
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_cycle_recent: Option<String>,
    ///Previous recently used model
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_cycle_recent_reverse: Option<String>,
    ///Next favorite model
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_cycle_favorite: Option<String>,
    ///Previous favorite model
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_cycle_favorite_reverse: Option<String>,
    ///List available commands
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_list: Option<String>,
    ///List agents
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_list: Option<String>,
    ///Next agent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_cycle: Option<String>,
    ///Previous agent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_cycle_reverse: Option<String>,
    ///Cycle model variants
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant_cycle: Option<String>,
    ///Clear input field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_clear: Option<String>,
    ///Paste from clipboard
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_paste: Option<String>,
    ///Submit input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_submit: Option<String>,
    ///Insert newline in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_newline: Option<String>,
    ///Move cursor left in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_move_left: Option<String>,
    ///Move cursor right in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_move_right: Option<String>,
    ///Move cursor up in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_move_up: Option<String>,
    ///Move cursor down in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_move_down: Option<String>,
    ///Select left in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_left: Option<String>,
    ///Select right in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_right: Option<String>,
    ///Select up in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_up: Option<String>,
    ///Select down in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_down: Option<String>,
    ///Move to start of line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_line_home: Option<String>,
    ///Move to end of line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_line_end: Option<String>,
    ///Select to start of line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_line_home: Option<String>,
    ///Select to end of line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_line_end: Option<String>,
    ///Move to start of visual line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_visual_line_home: Option<String>,
    ///Move to end of visual line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_visual_line_end: Option<String>,
    ///Select to start of visual line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_visual_line_home: Option<String>,
    ///Select to end of visual line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_visual_line_end: Option<String>,
    ///Move to start of buffer in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_buffer_home: Option<String>,
    ///Move to end of buffer in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_buffer_end: Option<String>,
    ///Select to start of buffer in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_buffer_home: Option<String>,
    ///Select to end of buffer in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_buffer_end: Option<String>,
    ///Delete line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_delete_line: Option<String>,
    ///Delete to end of line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_delete_to_line_end: Option<String>,
    ///Delete to start of line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_delete_to_line_start: Option<String>,
    ///Backspace in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_backspace: Option<String>,
    ///Delete character in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_delete: Option<String>,
    ///Undo in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_undo: Option<String>,
    ///Redo in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_redo: Option<String>,
    ///Move word forward in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_word_forward: Option<String>,
    ///Move word backward in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_word_backward: Option<String>,
    ///Select word forward in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_word_forward: Option<String>,
    ///Select word backward in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_word_backward: Option<String>,
    ///Delete word forward in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_delete_word_forward: Option<String>,
    ///Delete word backward in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_delete_word_backward: Option<String>,
    ///Previous history item
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_previous: Option<String>,
    ///Next history item
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_next: Option<String>,
    ///Next child session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_child_cycle: Option<String>,
    ///Previous child session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_child_cycle_reverse: Option<String>,
    ///Go to parent session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_parent: Option<String>,
    ///Suspend terminal
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal_suspend: Option<String>,
    ///Toggle terminal title
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal_title_toggle: Option<String>,
    ///Toggle tips on home screen
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tips_toggle: Option<String>,
}
///Server configuration for opencode serve and web commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    ///Port to listen on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    ///Hostname to listen on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    ///Enable mDNS service discovery
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdns: Option<bool>,
    ///Additional domains to allow for CORS
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cors: Option<Vec<Option<String>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionObjectConfig {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRuleConfig {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionConfig {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    ///@deprecated Use 'permission' field instead
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
    ///Description of when to use the agent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<StringEnum>,
    ///Hide this subagent from the @ autocomplete menu (default: false, only applies to mode: subagent)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    ///Hex color code for the agent (e.g., #FF5733)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    ///Maximum number of agentic iterations before forcing text-only response
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
    ///@deprecated Use 'steps' field instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSteps")]
    pub max_steps: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<PermissionConfig>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<Option<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub models: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<Option<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blacklist: Option<Vec<Option<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpLocalConfig {
    ///Type of MCP server connection
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    ///Command and arguments to run the MCP server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<Option<String>>>,
    ///Environment variables to set when running the MCP server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<serde_json::Value>,
    ///Enable or disable the MCP server on startup
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    ///Timeout in ms for fetching tools from the MCP server. Defaults to 5000 (5 seconds) if not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpOAuthConfig {
    ///OAuth client ID. If not provided, dynamic client registration (RFC 7591) will be attempted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientId")]
    pub client_id: Option<String>,
    ///OAuth client secret (if required by the authorization server)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientSecret")]
    pub client_secret: Option<String>,
    ///OAuth scopes to request during authorization
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRemoteConfig {
    ///Type of MCP server connection
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    ///URL of the remote MCP server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    ///Enable or disable the MCP server on startup
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    ///Headers to send with the request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    ///OAuth authentication configuration for the MCP server. Set to false to disable OAuth auto-detection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth: Option<serde_json::Value>,
    ///Timeout in ms for fetching tools from the MCP server. Defaults to 5000 (5 seconds) if not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    ///JSON schema reference for configuration validation
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "$schema")]
    pub dollar_schema: Option<String>,
    ///Theme name to use for the interface
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keybinds: Option<KeybindsConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<LogLevel>,
    ///TUI specific settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tui: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<ServerConfig>,
    ///Command configuration, see https://opencode.ai/docs/commands
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watcher: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin: Option<Vec<Option<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
    ///Control sharing behavior:'manual' allows manual sharing via commands, 'auto' enables automatic sharing, 'disabled' disables all sharing
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share: Option<StringEnum>,
    ///@deprecated Use 'share' field instead. Share newly created sessions automatically
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autoshare: Option<bool>,
    ///Automatically update to the latest version. Set to true to auto-update, false to disable, or 'notify' to show update notifications
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autoupdate: Option<serde_json::Value>,
    ///Disable providers that are loaded automatically
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_providers: Option<Vec<Option<String>>>,
    ///When set, ONLY these providers will be enabled. All other providers will be ignored
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled_providers: Option<Vec<Option<String>>>,
    ///Model to use in the format of provider/model, eg anthropic/claude-2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    ///Small model to use for tasks like title generation in the format of provider/model
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub small_model: Option<String>,
    ///Default agent to use when none is specified. Must be a primary agent. Falls back to 'build' if not set or if the specified agent is invalid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_agent: Option<String>,
    ///Custom username to display in conversations instead of system username
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    ///@deprecated Use `agent` field instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<serde_json::Value>,
    ///Agent configuration, see https://opencode.ai/docs/agent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<serde_json::Value>,
    ///Custom provider configurations and model overrides
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<serde_json::Value>,
    ///MCP (Model Context Protocol) server configurations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcp: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatter: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lsp: Option<serde_json::Value>,
    ///Additional instruction files or patterns to include
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<Vec<Option<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<LayoutConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<PermissionConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compaction: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experimental: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolIDs {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolListItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolList {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub home: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worktree: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worktree {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorktreeCreateInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startCommand")]
    pub start_command: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VcsInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPartInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synthetic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePartInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<FilePartSource>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPartInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtaskPartInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcp: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtask: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hints: Option<Vec<Option<String>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerID")]
    pub provider_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variants: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<StringEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<Option<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub models: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAuthMethod {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAuthAuthorization {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub absolute: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<StringEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContent {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mimeType")]
    pub mime_type: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StringEnum>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<StringEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topP")]
    pub top_p: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<PermissionRuleset>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatusConnected {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatusDisabled {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatusFailed {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatusNeedsAuth {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatusNeedsClientRegistration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatus {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mimeType")]
    pub mime_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LSPStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatterStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Option<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enterpriseUrl")]
    pub enterprise_url: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuth {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellKnownAuth {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalHealthGetResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Option<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline3 {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parentID")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<PermissionRuleset>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline4 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline5 {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelID")]
    pub model_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerID")]
    pub provider_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline6 {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline7 {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerID")]
    pub provider_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "modelID")]
    pub model_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidMessageGetResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<Message>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Option<Part>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline8 {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "noReply")]
    pub no_reply: Option<bool>,
    ///@deprecated tools and permissions have been merged, you can set permissions on the session itself now
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Option<serde_json::Value>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidMessagePostResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<AssistantMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Option<Part>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidMessageMessageidGetResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<Message>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Option<Part>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline9 {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "noReply")]
    pub no_reply: Option<bool>,
    ///@deprecated tools and permissions have been merged, you can set permissions on the session itself now
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Option<serde_json::Value>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline10 {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Option<serde_json::Value>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidCommandPostResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<AssistantMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Option<Part>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline11 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline12 {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "partID")]
    pub part_id: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline13 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<StringEnum>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline14 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply: Option<StringEnum>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline15 {
    ///User answers in order of questions (each answer is an array of selected labels)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answers: Option<Vec<Option<QuestionAnswer>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProvidersGetResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<Option<Provider>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderGetResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<Vec<Option<serde_json::Value>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connected: Option<Vec<Option<String>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline16 {
    ///Auth method index
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline17 {
    ///Auth method index
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<f64>,
    ///OAuth authorization code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindGetResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lines: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_number: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub absolute_offset: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submatches: Option<Vec<Option<serde_json::Value>>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline18 {
    ///Service name for the log entry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    ///Log level
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<StringEnum>,
    ///Log message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    ///Additional metadata for the log entry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline19 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpNameAuthPostResponse {
    ///URL to open in browser for authorization
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authorizationUrl"
    )]
    pub authorization_url: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpNameAuthDeleteResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline20 {
    ///Authorization code from OAuth callback
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline21 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline22 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline23 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<StringEnum>,
    ///Duration in milliseconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestInline24 {
    ///Session ID to navigate to
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiControlNextGetResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum StringEnum {
    Running,
    Exited,
}
pub trait ApiService: ::oapi_universal_gen::OapiRequester {
    /**ENDPOINT Get /global/health
Get health information about the OpenCode server.
*/
    fn global_health_get(
        &self,
    ) -> impl Future<Output = Option<GlobalHealthGetResponse>> {
        async move {
            let query_params: Vec<(String, String)> = Vec::new();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/global/health",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<GlobalHealthGetResponse>(&content).ok()
        }
    }
    /**ENDPOINT Get /global/event
Subscribe to global events from the OpenCode system using server-sent events.
*/
    fn global_event_get(&self) -> impl Future<Output = Option<()>> {
        async move {
            let query_params: Vec<(String, String)> = Vec::new();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/global/event",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            Some(())
        }
    }
    /**ENDPOINT Post /global/dispose
Clean up and dispose all OpenCode instances, releasing all resources.
*/
    fn global_dispose_post(&self) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = Vec::new();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/global/dispose",
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Get /project
Get a list of projects that have been opened with OpenCode.

# Arguments

- `query` directory
*/
    fn project_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<Vec<Project>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/project", &query_params);
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<Project>>(&content).ok()
        }
    }
    /**ENDPOINT Get /project/current
Retrieve the currently active project that OpenCode is working with.

# Arguments

- `query` directory
*/
    fn project_current_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<Project>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/project/current",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Project>(&content).ok()
        }
    }
    /**ENDPOINT Patch /project/{projectID}
Update project properties such as name, icon and color.

# Arguments

- `query` directory
- `path` project_id
*/
    fn project_projectid_patch(
        &self,
        directory: Option<String>,
        project_id: String,
    ) -> impl Future<Output = Option<Project>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/project/{}", ::oapi_universal_gen::urlencode(project_id)),
                &query_params,
            );
            let body: RequestInline = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Patch, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Project>(&content).ok()
        }
    }
    /**ENDPOINT Get /pty
Get a list of all active pseudo-terminal (PTY) sessions managed by OpenCode.

# Arguments

- `query` directory
*/
    fn pty_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<Vec<Pty>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/pty", &query_params);
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<Pty>>(&content).ok()
        }
    }
    /**ENDPOINT Post /pty
Create a new pseudo-terminal (PTY) session for running shell commands and processes.

# Arguments

- `query` directory
*/
    fn pty_post(&self, directory: Option<String>) -> impl Future<Output = Option<Pty>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/pty", &query_params);
            let body: RequestInline1 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Pty>(&content).ok()
        }
    }
    /**ENDPOINT Get /pty/{ptyID}
Retrieve detailed information about a specific pseudo-terminal (PTY) session.

# Arguments

- `query` directory
- `path` pty_id
*/
    fn pty_ptyid_get(
        &self,
        directory: Option<String>,
        pty_id: String,
    ) -> impl Future<Output = Option<Pty>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/pty/{}", ::oapi_universal_gen::urlencode(pty_id)),
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Pty>(&content).ok()
        }
    }
    /**ENDPOINT Put /pty/{ptyID}
Update properties of an existing pseudo-terminal (PTY) session.

# Arguments

- `query` directory
- `path` pty_id
*/
    fn pty_ptyid_put(
        &self,
        directory: Option<String>,
        pty_id: String,
    ) -> impl Future<Output = Option<Pty>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/pty/{}", ::oapi_universal_gen::urlencode(pty_id)),
                &query_params,
            );
            let body: RequestInline2 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Put, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Pty>(&content).ok()
        }
    }
    /**ENDPOINT Delete /pty/{ptyID}
Remove and terminate a specific pseudo-terminal (PTY) session.

# Arguments

- `query` directory
- `path` pty_id
*/
    fn pty_ptyid_delete(
        &self,
        directory: Option<String>,
        pty_id: String,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/pty/{}", ::oapi_universal_gen::urlencode(pty_id)),
                &query_params,
            );
            let request = self.create_request(RequestType::Delete, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Get /pty/{ptyID}/connect
Establish a WebSocket connection to interact with a pseudo-terminal (PTY) session in real-time.

# Arguments

- `query` directory
- `path` pty_id
*/
    fn pty_ptyid_connect_get(
        &self,
        directory: Option<String>,
        pty_id: String,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/pty/{}/connect", ::oapi_universal_gen::urlencode(pty_id)),
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Get /config
Retrieve the current OpenCode configuration settings and preferences.

# Arguments

- `query` directory
*/
    fn config_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<Config>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/config", &query_params);
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Config>(&content).ok()
        }
    }
    /**ENDPOINT Patch /config
Update OpenCode configuration settings and preferences.

# Arguments

- `query` directory
*/
    fn config_patch(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<Config>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/config", &query_params);
            let body: Config = serde_json::from_value(serde_json::json!({})).ok()?;
            let request = self
                .create_request_with_body(RequestType::Patch, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Config>(&content).ok()
        }
    }
    /**ENDPOINT Get /experimental/tool/ids
Get a list of all available tool IDs, including both built-in tools and dynamically registered tools.

# Arguments

- `query` directory
*/
    fn experimental_tool_ids_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<ToolIDs>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/experimental/tool/ids",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<ToolIDs>(&content).ok()
        }
    }
    /**ENDPOINT Get /experimental/tool
Get a list of available tools with their JSON schema parameters for a specific provider and model combination.

# Arguments

- `query` directory
- `query` provider
- `query` model
*/
    fn experimental_tool_get(
        &self,
        directory: Option<String>,
        provider: String,
        model: String,
    ) -> impl Future<Output = Option<ToolList>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }, Some((String::from("provider"), provider
                .to_string())), Some((String::from("model"), model.to_string()))
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/experimental/tool",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<ToolList>(&content).ok()
        }
    }
    /**ENDPOINT Post /instance/dispose
Clean up and dispose the current OpenCode instance, releasing all resources.

# Arguments

- `query` directory
*/
    fn instance_dispose_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/instance/dispose",
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Get /path
Retrieve the current working directory and related path information for the OpenCode instance.

# Arguments

- `query` directory
*/
    fn path_get(&self, directory: Option<String>) -> impl Future<Output = Option<Path>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/path", &query_params);
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Path>(&content).ok()
        }
    }
    /**ENDPOINT Get /experimental/worktree
List all sandbox worktrees for the current project.

# Arguments

- `query` directory
*/
    fn experimental_worktree_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<Vec<String>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/experimental/worktree",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<String>>(&content).ok()
        }
    }
    /**ENDPOINT Post /experimental/worktree
Create a new git worktree for the current project.

# Arguments

- `query` directory
*/
    fn experimental_worktree_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<Worktree>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/experimental/worktree",
                &query_params,
            );
            let body: WorktreeCreateInput = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Worktree>(&content).ok()
        }
    }
    /**ENDPOINT Get /vcs
Retrieve version control system (VCS) information for the current project, such as git branch.

# Arguments

- `query` directory
*/
    fn vcs_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<VcsInfo>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/vcs", &query_params);
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<VcsInfo>(&content).ok()
        }
    }
    /**ENDPOINT Get /session
Get a list of all OpenCode sessions, sorted by most recently updated.

# Arguments

- `query` directory
- `query` start - Filter sessions updated on or after this timestamp (milliseconds since epoch)
- `query` search - Filter sessions by title (case-insensitive)
- `query` limit - Maximum number of sessions to return
*/
    fn session_get(
        &self,
        directory: Option<String>,
        start: Option<f64>,
        search: Option<String>,
        limit: Option<f64>,
    ) -> impl Future<Output = Option<Vec<Session>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }, match start.as_ref() { Some(v) =>
                Some((String::from("start"), v.to_string())), None => None, }, match
                search.as_ref() { Some(v) => Some((String::from("search"), v
                .to_string())), None => None, }, match limit.as_ref() { Some(v) =>
                Some((String::from("limit"), v.to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/session", &query_params);
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<Session>>(&content).ok()
        }
    }
    /**ENDPOINT Post /session
Create a new OpenCode session for interacting with AI assistants and managing conversations.

# Arguments

- `query` directory
*/
    fn session_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<Session>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/session", &query_params);
            let body: RequestInline3 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Session>(&content).ok()
        }
    }
    /**ENDPOINT Get /session/status
Retrieve the current status of all sessions, including active, idle, and completed states.

# Arguments

- `query` directory
*/
    fn session_status_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<serde_json::Value>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/session/status",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<serde_json::Value>(&content).ok()
        }
    }
    /**ENDPOINT Get /session/{sessionID}
Retrieve detailed information about a specific OpenCode session.

# Arguments

- `query` directory
- `path` session_id
*/
    fn session_sessionid_get(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<Session>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/session/{}", ::oapi_universal_gen::urlencode(session_id)),
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Session>(&content).ok()
        }
    }
    /**ENDPOINT Patch /session/{sessionID}
Update properties of an existing session, such as title or other metadata.

# Arguments

- `query` directory
- `path` session_id
*/
    fn session_sessionid_patch(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<Session>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/session/{}", ::oapi_universal_gen::urlencode(session_id)),
                &query_params,
            );
            let body: RequestInline4 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Patch, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Session>(&content).ok()
        }
    }
    /**ENDPOINT Delete /session/{sessionID}
Delete a session and permanently remove all associated data, including messages and history.

# Arguments

- `query` directory
- `path` session_id
*/
    fn session_sessionid_delete(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/session/{}", ::oapi_universal_gen::urlencode(session_id)),
                &query_params,
            );
            let request = self.create_request(RequestType::Delete, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Get /session/{sessionID}/children
Retrieve all child sessions that were forked from the specified parent session.

# Arguments

- `query` directory
- `path` session_id
*/
    fn session_sessionid_children_get(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<Vec<Session>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/children", ::oapi_universal_gen::urlencode(session_id)
                ),
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<Session>>(&content).ok()
        }
    }
    /**ENDPOINT Get /session/{sessionID}/todo
Retrieve the todo list associated with a specific session, showing tasks and action items.

# Arguments

- `query` directory
- `path` session_id - Session ID
*/
    fn session_sessionid_todo_get(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<Vec<Todo>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/session/{}/todo", ::oapi_universal_gen::urlencode(session_id)),
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<Todo>>(&content).ok()
        }
    }
    /**ENDPOINT Post /session/{sessionID}/init
Analyze the current application and create an AGENTS.md file with project-specific agent configurations.

# Arguments

- `query` directory
- `path` session_id - Session ID
*/
    fn session_sessionid_init_post(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/session/{}/init", ::oapi_universal_gen::urlencode(session_id)),
                &query_params,
            );
            let body: RequestInline5 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /session/{sessionID}/fork
Create a new session by forking an existing session at a specific message point.

# Arguments

- `query` directory
- `path` session_id
*/
    fn session_sessionid_fork_post(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<Session>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/session/{}/fork", ::oapi_universal_gen::urlencode(session_id)),
                &query_params,
            );
            let body: RequestInline6 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Session>(&content).ok()
        }
    }
    /**ENDPOINT Post /session/{sessionID}/abort
Abort an active session and stop any ongoing AI processing or command execution.

# Arguments

- `query` directory
- `path` session_id
*/
    fn session_sessionid_abort_post(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/abort", ::oapi_universal_gen::urlencode(session_id)
                ),
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /session/{sessionID}/share
Create a shareable link for a session, allowing others to view the conversation.

# Arguments

- `query` directory
- `path` session_id
*/
    fn session_sessionid_share_post(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<Session>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/share", ::oapi_universal_gen::urlencode(session_id)
                ),
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Session>(&content).ok()
        }
    }
    /**ENDPOINT Delete /session/{sessionID}/share
Remove the shareable link for a session, making it private again.

# Arguments

- `query` directory
- `path` session_id
*/
    fn session_sessionid_share_delete(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<Session>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/share", ::oapi_universal_gen::urlencode(session_id)
                ),
                &query_params,
            );
            let request = self.create_request(RequestType::Delete, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Session>(&content).ok()
        }
    }
    /**ENDPOINT Get /session/{sessionID}/diff
Get all file changes (diffs) made during this session.

# Arguments

- `query` directory
- `path` session_id - Session ID
- `query` message_id
*/
    fn session_sessionid_diff_get(
        &self,
        directory: Option<String>,
        session_id: String,
        message_id: Option<String>,
    ) -> impl Future<Output = Option<Vec<FileDiff>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }, match message_id.as_ref() { Some(v) =>
                Some((String::from("messageID"), v.to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/session/{}/diff", ::oapi_universal_gen::urlencode(session_id)),
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<FileDiff>>(&content).ok()
        }
    }
    /**ENDPOINT Post /session/{sessionID}/summarize
Generate a concise summary of the session using AI compaction to preserve key information.

# Arguments

- `query` directory
- `path` session_id - Session ID
*/
    fn session_sessionid_summarize_post(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/summarize", ::oapi_universal_gen::urlencode(session_id)
                ),
                &query_params,
            );
            let body: RequestInline7 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Get /session/{sessionID}/message
Retrieve all messages in a session, including user prompts and AI responses.

# Arguments

- `query` directory
- `path` session_id - Session ID
- `query` limit
*/
    fn session_sessionid_message_get(
        &self,
        directory: Option<String>,
        session_id: String,
        limit: Option<f64>,
    ) -> impl Future<Output = Option<Vec<SessionSessionidMessageGetResponse>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }, match limit.as_ref() { Some(v) =>
                Some((String::from("limit"), v.to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/message", ::oapi_universal_gen::urlencode(session_id)
                ),
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<SessionSessionidMessageGetResponse>>(&content)
                .ok()
        }
    }
    /**ENDPOINT Post /session/{sessionID}/message
Create and send a new message to a session, streaming the AI response.

# Arguments

- `query` directory
- `path` session_id - Session ID
*/
    fn session_sessionid_message_post(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<SessionSessionidMessagePostResponse>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/message", ::oapi_universal_gen::urlencode(session_id)
                ),
                &query_params,
            );
            let body: RequestInline8 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<SessionSessionidMessagePostResponse>(&content).ok()
        }
    }
    /**ENDPOINT Get /session/{sessionID}/message/{messageID}
Retrieve a specific message from a session by its message ID.

# Arguments

- `query` directory
- `path` session_id - Session ID
- `path` message_id - Message ID
*/
    fn session_sessionid_message_messageid_get(
        &self,
        directory: Option<String>,
        session_id: String,
        message_id: String,
    ) -> impl Future<Output = Option<SessionSessionidMessageMessageidGetResponse>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/message/{}",
                    ::oapi_universal_gen::urlencode(session_id),
                    ::oapi_universal_gen::urlencode(message_id)
                ),
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<SessionSessionidMessageMessageidGetResponse>(&content)
                .ok()
        }
    }
    /**ENDPOINT Patch /session/{sessionID}/message/{messageID}/part/{partID}
Update a part in a message

# Arguments

- `query` directory
- `path` session_id - Session ID
- `path` message_id - Message ID
- `path` part_id - Part ID
*/
    fn session_sessionid_message_messageid_part_partid_patch(
        &self,
        directory: Option<String>,
        session_id: String,
        message_id: String,
        part_id: String,
    ) -> impl Future<Output = Option<Part>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/message/{}/part/{}",
                    ::oapi_universal_gen::urlencode(session_id),
                    ::oapi_universal_gen::urlencode(message_id),
                    ::oapi_universal_gen::urlencode(part_id)
                ),
                &query_params,
            );
            let body: Part = serde_json::from_value(serde_json::json!({})).ok()?;
            let request = self
                .create_request_with_body(RequestType::Patch, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Part>(&content).ok()
        }
    }
    /**ENDPOINT Delete /session/{sessionID}/message/{messageID}/part/{partID}
Delete a part from a message

# Arguments

- `query` directory
- `path` session_id - Session ID
- `path` message_id - Message ID
- `path` part_id - Part ID
*/
    fn session_sessionid_message_messageid_part_partid_delete(
        &self,
        directory: Option<String>,
        session_id: String,
        message_id: String,
        part_id: String,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/message/{}/part/{}",
                    ::oapi_universal_gen::urlencode(session_id),
                    ::oapi_universal_gen::urlencode(message_id),
                    ::oapi_universal_gen::urlencode(part_id)
                ),
                &query_params,
            );
            let request = self.create_request(RequestType::Delete, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /session/{sessionID}/prompt_async
Create and send a new message to a session asynchronously, starting the session if needed and returning immediately.

# Arguments

- `query` directory
- `path` session_id - Session ID
*/
    fn session_sessionid_prompt_async_post(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<()>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/prompt_async",
                    ::oapi_universal_gen::urlencode(session_id)
                ),
                &query_params,
            );
            let body: RequestInline9 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            Some(())
        }
    }
    /**ENDPOINT Post /session/{sessionID}/command
Send a new command to a session for execution by the AI assistant.

# Arguments

- `query` directory
- `path` session_id - Session ID
*/
    fn session_sessionid_command_post(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<SessionSessionidCommandPostResponse>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/command", ::oapi_universal_gen::urlencode(session_id)
                ),
                &query_params,
            );
            let body: RequestInline10 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<SessionSessionidCommandPostResponse>(&content).ok()
        }
    }
    /**ENDPOINT Post /session/{sessionID}/shell
Execute a shell command within the session context and return the AI's response.

# Arguments

- `query` directory
- `path` session_id - Session ID
*/
    fn session_sessionid_shell_post(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<AssistantMessage>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/shell", ::oapi_universal_gen::urlencode(session_id)
                ),
                &query_params,
            );
            let body: RequestInline11 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<AssistantMessage>(&content).ok()
        }
    }
    /**ENDPOINT Post /session/{sessionID}/revert
Revert a specific message in a session, undoing its effects and restoring the previous state.

# Arguments

- `query` directory
- `path` session_id
*/
    fn session_sessionid_revert_post(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<Session>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/revert", ::oapi_universal_gen::urlencode(session_id)
                ),
                &query_params,
            );
            let body: RequestInline12 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Session>(&content).ok()
        }
    }
    /**ENDPOINT Post /session/{sessionID}/unrevert
Restore all previously reverted messages in a session.

# Arguments

- `query` directory
- `path` session_id
*/
    fn session_sessionid_unrevert_post(
        &self,
        directory: Option<String>,
        session_id: String,
    ) -> impl Future<Output = Option<Session>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/unrevert", ::oapi_universal_gen::urlencode(session_id)
                ),
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Session>(&content).ok()
        }
    }
    /**ENDPOINT Post /session/{sessionID}/permissions/{permissionID}
Approve or deny a permission request from the AI assistant.

# Arguments

- `query` directory
- `path` session_id
- `path` permission_id
*/
    fn session_sessionid_permissions_permissionid_post(
        &self,
        directory: Option<String>,
        session_id: String,
        permission_id: String,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/session/{}/permissions/{}",
                    ::oapi_universal_gen::urlencode(session_id),
                    ::oapi_universal_gen::urlencode(permission_id)
                ),
                &query_params,
            );
            let body: RequestInline13 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /permission/{requestID}/reply
Approve or deny a permission request from the AI assistant.

# Arguments

- `query` directory
- `path` request_id
*/
    fn permission_requestid_reply_post(
        &self,
        directory: Option<String>,
        request_id: String,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/permission/{}/reply", ::oapi_universal_gen::urlencode(request_id)
                ),
                &query_params,
            );
            let body: RequestInline14 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Get /permission
Get all pending permission requests across all sessions.

# Arguments

- `query` directory
*/
    fn permission_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<Vec<PermissionRequest>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/permission",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<PermissionRequest>>(&content).ok()
        }
    }
    /**ENDPOINT Get /question
Get all pending question requests across all sessions.

# Arguments

- `query` directory
*/
    fn question_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<Vec<QuestionRequest>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/question",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<QuestionRequest>>(&content).ok()
        }
    }
    /**ENDPOINT Post /question/{requestID}/reply
Provide answers to a question request from the AI assistant.

# Arguments

- `query` directory
- `path` request_id
*/
    fn question_requestid_reply_post(
        &self,
        directory: Option<String>,
        request_id: String,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/question/{}/reply", ::oapi_universal_gen::urlencode(request_id)
                ),
                &query_params,
            );
            let body: RequestInline15 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /question/{requestID}/reject
Reject a question request from the AI assistant.

# Arguments

- `query` directory
- `path` request_id
*/
    fn question_requestid_reject_post(
        &self,
        directory: Option<String>,
        request_id: String,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/question/{}/reject", ::oapi_universal_gen::urlencode(request_id)
                ),
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Get /command
Get a list of all available commands in the OpenCode system.

# Arguments

- `query` directory
*/
    fn command_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<Vec<Command>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/command", &query_params);
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<Command>>(&content).ok()
        }
    }
    /**ENDPOINT Get /config/providers
Get a list of all configured AI providers and their default models.

# Arguments

- `query` directory
*/
    fn config_providers_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<ConfigProvidersGetResponse>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/config/providers",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<ConfigProvidersGetResponse>(&content).ok()
        }
    }
    /**ENDPOINT Get /provider
Get a list of all available AI providers, including both available and connected ones.

# Arguments

- `query` directory
*/
    fn provider_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<ProviderGetResponse>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/provider",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<ProviderGetResponse>(&content).ok()
        }
    }
    /**ENDPOINT Get /provider/auth
Retrieve available authentication methods for all AI providers.

# Arguments

- `query` directory
*/
    fn provider_auth_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<serde_json::Value>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/provider/auth",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<serde_json::Value>(&content).ok()
        }
    }
    /**ENDPOINT Post /provider/{providerID}/oauth/authorize
Initiate OAuth authorization for a specific AI provider to get an authorization URL.

# Arguments

- `query` directory
- `path` provider_id - Provider ID
*/
    fn provider_providerid_oauth_authorize_post(
        &self,
        directory: Option<String>,
        provider_id: String,
    ) -> impl Future<Output = Option<ProviderAuthAuthorization>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/provider/{}/oauth/authorize",
                    ::oapi_universal_gen::urlencode(provider_id)
                ),
                &query_params,
            );
            let body: RequestInline16 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<ProviderAuthAuthorization>(&content).ok()
        }
    }
    /**ENDPOINT Post /provider/{providerID}/oauth/callback
Handle the OAuth callback from a provider after user authorization.

# Arguments

- `query` directory
- `path` provider_id - Provider ID
*/
    fn provider_providerid_oauth_callback_post(
        &self,
        directory: Option<String>,
        provider_id: String,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/provider/{}/oauth/callback",
                    ::oapi_universal_gen::urlencode(provider_id)
                ),
                &query_params,
            );
            let body: RequestInline17 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Get /find
Search for text patterns across files in the project using ripgrep.

# Arguments

- `query` directory
- `query` pattern
*/
    fn find_get(
        &self,
        directory: Option<String>,
        pattern: String,
    ) -> impl Future<Output = Option<Vec<FindGetResponse>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }, Some((String::from("pattern"), pattern
                .to_string()))
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/find", &query_params);
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<FindGetResponse>>(&content).ok()
        }
    }
    /**ENDPOINT Get /find/file
Search for files or directories by name or pattern in the project directory.

# Arguments

- `query` directory
- `query` query
- `query` dirs
- `query` type_field
- `query` limit
*/
    fn find_file_get(
        &self,
        directory: Option<String>,
        query: String,
        dirs: Option<StringEnum>,
        type_field: Option<StringEnum>,
        limit: Option<i64>,
    ) -> impl Future<Output = Option<Vec<String>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }, Some((String::from("query"), query
                .to_string())), match dirs.as_ref() { Some(v) =>
                Some((String::from("dirs"), v.to_string())), None => None, }, match
                type_field.as_ref() { Some(v) => Some((String::from("type"), v
                .to_string())), None => None, }, match limit.as_ref() { Some(v) =>
                Some((String::from("limit"), v.to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/find/file",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<String>>(&content).ok()
        }
    }
    /**ENDPOINT Get /find/symbol
Search for workspace symbols like functions, classes, and variables using LSP.

# Arguments

- `query` directory
- `query` query
*/
    fn find_symbol_get(
        &self,
        directory: Option<String>,
        query: String,
    ) -> impl Future<Output = Option<Vec<Symbol>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }, Some((String::from("query"), query
                .to_string()))
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/find/symbol",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<Symbol>>(&content).ok()
        }
    }
    /**ENDPOINT Get /file
List files and directories in a specified path.

# Arguments

- `query` directory
- `query` path
*/
    fn file_get(
        &self,
        directory: Option<String>,
        path: String,
    ) -> impl Future<Output = Option<Vec<FileNode>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }, Some((String::from("path"), path
                .to_string()))
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/file", &query_params);
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<FileNode>>(&content).ok()
        }
    }
    /**ENDPOINT Get /file/content
Read the content of a specified file.

# Arguments

- `query` directory
- `query` path
*/
    fn file_content_get(
        &self,
        directory: Option<String>,
        path: String,
    ) -> impl Future<Output = Option<FileContent>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }, Some((String::from("path"), path
                .to_string()))
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/file/content",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<FileContent>(&content).ok()
        }
    }
    /**ENDPOINT Get /file/status
Get the git status of all files in the project.

# Arguments

- `query` directory
*/
    fn file_status_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<Vec<File>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/file/status",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<File>>(&content).ok()
        }
    }
    /**ENDPOINT Post /log
Write a log entry to the server logs with specified level and metadata.

# Arguments

- `query` directory
*/
    fn log_post(&self, directory: Option<String>) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/log", &query_params);
            let body: RequestInline18 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Get /agent
Get a list of all available AI agents in the OpenCode system.

# Arguments

- `query` directory
*/
    fn agent_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<Vec<Agent>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/agent", &query_params);
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<Agent>>(&content).ok()
        }
    }
    /**ENDPOINT Get /mcp
Get the status of all Model Context Protocol (MCP) servers.

# Arguments

- `query` directory
*/
    fn mcp_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<serde_json::Value>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/mcp", &query_params);
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<serde_json::Value>(&content).ok()
        }
    }
    /**ENDPOINT Post /mcp
Dynamically add a new Model Context Protocol (MCP) server to the system.

# Arguments

- `query` directory
*/
    fn mcp_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<serde_json::Value>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/mcp", &query_params);
            let body: RequestInline19 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<serde_json::Value>(&content).ok()
        }
    }
    /**ENDPOINT Post /mcp/{name}/auth
Start OAuth authentication flow for a Model Context Protocol (MCP) server.

# Arguments

- `query` directory
- `path` name
*/
    fn mcp_name_auth_post(
        &self,
        directory: Option<String>,
        name: String,
    ) -> impl Future<Output = Option<McpNameAuthPostResponse>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/mcp/{}/auth", ::oapi_universal_gen::urlencode(name)),
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<McpNameAuthPostResponse>(&content).ok()
        }
    }
    /**ENDPOINT Delete /mcp/{name}/auth
Remove OAuth credentials for an MCP server

# Arguments

- `query` directory
- `path` name
*/
    fn mcp_name_auth_delete(
        &self,
        directory: Option<String>,
        name: String,
    ) -> impl Future<Output = Option<McpNameAuthDeleteResponse>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/mcp/{}/auth", ::oapi_universal_gen::urlencode(name)),
                &query_params,
            );
            let request = self.create_request(RequestType::Delete, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<McpNameAuthDeleteResponse>(&content).ok()
        }
    }
    /**ENDPOINT Post /mcp/{name}/auth/callback
Complete OAuth authentication for a Model Context Protocol (MCP) server using the authorization code.

# Arguments

- `query` directory
- `path` name
*/
    fn mcp_name_auth_callback_post(
        &self,
        directory: Option<String>,
        name: String,
    ) -> impl Future<Output = Option<MCPStatus>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/mcp/{}/auth/callback", ::oapi_universal_gen::urlencode(name)),
                &query_params,
            );
            let body: RequestInline20 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<MCPStatus>(&content).ok()
        }
    }
    /**ENDPOINT Post /mcp/{name}/auth/authenticate
Start OAuth flow and wait for callback (opens browser)

# Arguments

- `query` directory
- `path` name
*/
    fn mcp_name_auth_authenticate_post(
        &self,
        directory: Option<String>,
        name: String,
    ) -> impl Future<Output = Option<MCPStatus>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!(
                    "/mcp/{}/auth/authenticate", ::oapi_universal_gen::urlencode(name)
                ),
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<MCPStatus>(&content).ok()
        }
    }
    /**ENDPOINT Post /mcp/{name}/connect
Connect an MCP server

# Arguments

- `query` directory
- `path` name
*/
    fn mcp_name_connect_post(
        &self,
        directory: Option<String>,
        name: String,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/mcp/{}/connect", ::oapi_universal_gen::urlencode(name)),
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /mcp/{name}/disconnect
Disconnect an MCP server

# Arguments

- `query` directory
- `path` name
*/
    fn mcp_name_disconnect_post(
        &self,
        directory: Option<String>,
        name: String,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/mcp/{}/disconnect", ::oapi_universal_gen::urlencode(name)),
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Get /experimental/resource
Get all available MCP resources from connected servers. Optionally filter by name.

# Arguments

- `query` directory
*/
    fn experimental_resource_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<serde_json::Value>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/experimental/resource",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<serde_json::Value>(&content).ok()
        }
    }
    /**ENDPOINT Get /lsp
Get LSP server status

# Arguments

- `query` directory
*/
    fn lsp_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<Vec<LSPStatus>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/lsp", &query_params);
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<LSPStatus>>(&content).ok()
        }
    }
    /**ENDPOINT Get /formatter
Get formatter status

# Arguments

- `query` directory
*/
    fn formatter_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<Vec<FormatterStatus>>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/formatter",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<Vec<FormatterStatus>>(&content).ok()
        }
    }
    /**ENDPOINT Post /tui/append-prompt
Append prompt to the TUI

# Arguments

- `query` directory
*/
    fn tui_append_prompt_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/tui/append-prompt",
                &query_params,
            );
            let body: RequestInline21 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /tui/open-help
Open the help dialog in the TUI to display user assistance information.

# Arguments

- `query` directory
*/
    fn tui_open_help_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/tui/open-help",
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /tui/open-sessions
Open the session dialog

# Arguments

- `query` directory
*/
    fn tui_open_sessions_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/tui/open-sessions",
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /tui/open-themes
Open the theme dialog

# Arguments

- `query` directory
*/
    fn tui_open_themes_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/tui/open-themes",
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /tui/open-models
Open the model dialog

# Arguments

- `query` directory
*/
    fn tui_open_models_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/tui/open-models",
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /tui/submit-prompt
Submit the prompt

# Arguments

- `query` directory
*/
    fn tui_submit_prompt_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/tui/submit-prompt",
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /tui/clear-prompt
Clear the prompt

# Arguments

- `query` directory
*/
    fn tui_clear_prompt_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/tui/clear-prompt",
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /tui/execute-command
Execute a TUI command (e.g. agent_cycle)

# Arguments

- `query` directory
*/
    fn tui_execute_command_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/tui/execute-command",
                &query_params,
            );
            let body: RequestInline22 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /tui/show-toast
Show a toast notification in the TUI

# Arguments

- `query` directory
*/
    fn tui_show_toast_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/tui/show-toast",
                &query_params,
            );
            let body: RequestInline23 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /tui/publish
Publish a TUI event

# Arguments

- `query` directory
*/
    fn tui_publish_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/tui/publish",
                &query_params,
            );
            let body: serde_json::Value = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Post /tui/select-session
Navigate the TUI to display the specified session.

# Arguments

- `query` directory
*/
    fn tui_select_session_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/tui/select-session",
                &query_params,
            );
            let body: RequestInline24 = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Get /tui/control/next
Retrieve the next TUI (Terminal User Interface) request from the queue for processing.

# Arguments

- `query` directory
*/
    fn tui_control_next_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<TuiControlNextGetResponse>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/tui/control/next",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<TuiControlNextGetResponse>(&content).ok()
        }
    }
    /**ENDPOINT Post /tui/control/response
Submit a response to the TUI request queue to complete a pending request.

# Arguments

- `query` directory
*/
    fn tui_control_response_post(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/tui/control/response",
                &query_params,
            );
            let body: serde_json::Value = serde_json::from_value(serde_json::json!({}))
                .ok()?;
            let request = self
                .create_request_with_body(RequestType::Post, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Put /auth/{providerID}
Set authentication credentials

# Arguments

- `query` directory
- `path` provider_id
*/
    fn auth_providerid_put(
        &self,
        directory: Option<String>,
        provider_id: String,
    ) -> impl Future<Output = Option<bool>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                format!("/auth/{}", ::oapi_universal_gen::urlencode(provider_id)),
                &query_params,
            );
            let body: Auth = serde_json::from_value(serde_json::json!({})).ok()?;
            let request = self
                .create_request_with_body(RequestType::Put, &uri, &body)
                .ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            let content = response.response_content().await?;
            serde_json::from_str::<bool>(&content).ok()
        }
    }
    /**ENDPOINT Get /event
Get events

# Arguments

- `query` directory
*/
    fn event_get(&self, directory: Option<String>) -> impl Future<Output = Option<()>> {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/event", &query_params);
            let request = self.create_request(RequestType::Get, &uri).ok()?;
            let response = request.send_request().await.ok()?;
            if response.is_client_error() || response.is_server_error() {
                return None;
            }
            Some(())
        }
    }
}
