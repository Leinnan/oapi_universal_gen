use serde::{Serialize, Deserialize};
use oapi_universal_gen::*;
use std::future::Future;
///
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "type")]
pub enum SessionStatus {
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "retry")]
    #[display("Retry")]
    Retry { attempt: f64, message: String, next: f64 },
    #[serde(rename = "busy")]
    Busy,
}
///
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "role")]
pub enum Message {
    #[serde(rename = "user")]
    #[display("User")]
    User {
        id: String,
        #[serde(rename = "sessionID")]
        session_id: String,
        time: UserMessageTime,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        summary: Option<UserMessageSummary>,
        agent: String,
        model: UserMessageModel,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        system: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tools: Option<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        variant: Option<String>,
    },
    #[serde(rename = "assistant")]
    #[display("Assistant")]
    Assistant {
        id: String,
        #[serde(rename = "sessionID")]
        session_id: String,
        time: AssistantMessageTime,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        error: Option<Name>,
        #[serde(rename = "parentID")]
        parent_id: String,
        #[serde(rename = "modelID")]
        model_id: String,
        #[serde(rename = "providerID")]
        provider_id: String,
        mode: String,
        agent: String,
        path: AssistantMessagePath,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        summary: Option<bool>,
        cost: f64,
        tokens: AssistantMessageTokens,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        finish: Option<String>,
    },
}
///
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "type")]
pub enum FilePartSource {
    #[serde(rename = "file")]
    #[display("File")]
    File { text: FilePartSourceText, path: String },
    #[serde(rename = "symbol")]
    #[display("Symbol")]
    Symbol {
        text: FilePartSourceText,
        path: String,
        range: Range,
        name: String,
        kind: i64,
    },
    #[serde(rename = "resource")]
    #[display("Resource")]
    Resource {
        text: FilePartSourceText,
        #[serde(rename = "clientName")]
        client_name: String,
        uri: String,
    },
}
///
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "status")]
pub enum ToolState {
    #[serde(rename = "pending")]
    #[display("Pending")]
    Pending { input: serde_json::Value, raw: String },
    #[serde(rename = "running")]
    #[display("Running")]
    Running {
        input: serde_json::Value,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<serde_json::Value>,
        time: ToolStateRunningTime,
    },
    #[serde(rename = "completed")]
    #[display("Completed")]
    Completed {
        input: serde_json::Value,
        output: String,
        title: String,
        metadata: serde_json::Value,
        time: ToolStateCompletedTime,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        attachments: Option<Vec<FilePart>>,
    },
    #[serde(rename = "error")]
    #[display("Error")]
    Error {
        input: serde_json::Value,
        error: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<serde_json::Value>,
        time: ToolStateErrorTime,
    },
}
///
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "type")]
pub enum Part {
    #[serde(rename = "text")]
    #[display("Text")]
    Text {
        id: String,
        #[serde(rename = "sessionID")]
        session_id: String,
        #[serde(rename = "messageID")]
        message_id: String,
        text: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        synthetic: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ignored: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        time: Option<TextPartTime>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<serde_json::Value>,
    },
    #[serde(rename = "subtask")]
    #[display("Subtask")]
    Subtask {
        id: String,
        #[serde(rename = "sessionID")]
        session_id: String,
        #[serde(rename = "messageID")]
        message_id: String,
        prompt: String,
        description: String,
        agent: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        command: Option<String>,
    },
    #[serde(rename = "reasoning")]
    #[display("Reasoning")]
    Reasoning {
        id: String,
        #[serde(rename = "sessionID")]
        session_id: String,
        #[serde(rename = "messageID")]
        message_id: String,
        text: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<serde_json::Value>,
        time: ReasoningPartTime,
    },
    #[serde(rename = "file")]
    #[display("File")]
    File {
        id: String,
        #[serde(rename = "sessionID")]
        session_id: String,
        #[serde(rename = "messageID")]
        message_id: String,
        mime: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
        url: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source: Option<FilePartSource>,
    },
    #[serde(rename = "tool")]
    #[display("Tool")]
    Tool {
        id: String,
        #[serde(rename = "sessionID")]
        session_id: String,
        #[serde(rename = "messageID")]
        message_id: String,
        #[serde(rename = "callID")]
        call_id: String,
        tool: String,
        state: ToolState,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<serde_json::Value>,
    },
    #[serde(rename = "step-start")]
    #[display("StepStart")]
    StepStart {
        id: String,
        #[serde(rename = "sessionID")]
        session_id: String,
        #[serde(rename = "messageID")]
        message_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        snapshot: Option<String>,
    },
    #[serde(rename = "step-finish")]
    #[display("StepFinish")]
    StepFinish {
        id: String,
        #[serde(rename = "sessionID")]
        session_id: String,
        #[serde(rename = "messageID")]
        message_id: String,
        reason: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        snapshot: Option<String>,
        cost: f64,
        tokens: StepFinishPartTokens,
    },
    #[serde(rename = "snapshot")]
    #[display("Snapshot")]
    Snapshot {
        id: String,
        #[serde(rename = "sessionID")]
        session_id: String,
        #[serde(rename = "messageID")]
        message_id: String,
        snapshot: String,
    },
    #[serde(rename = "patch")]
    #[display("Patch")]
    Patch {
        id: String,
        #[serde(rename = "sessionID")]
        session_id: String,
        #[serde(rename = "messageID")]
        message_id: String,
        hash: String,
        files: Vec<String>,
    },
    #[serde(rename = "agent")]
    #[display("Agent")]
    Agent {
        id: String,
        #[serde(rename = "sessionID")]
        session_id: String,
        #[serde(rename = "messageID")]
        message_id: String,
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source: Option<AgentPartSource>,
    },
    #[serde(rename = "retry")]
    #[display("Retry")]
    Retry {
        id: String,
        #[serde(rename = "sessionID")]
        session_id: String,
        #[serde(rename = "messageID")]
        message_id: String,
        attempt: f64,
        error: APIError,
        time: RetryPartTime,
    },
    #[serde(rename = "compaction")]
    #[display("Compaction")]
    Compaction {
        id: String,
        #[serde(rename = "sessionID")]
        session_id: String,
        #[serde(rename = "messageID")]
        message_id: String,
        auto: bool,
    },
}
///
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum PermissionAction {
    Allow,
    Deny,
    Ask,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    EventTuiPromptAppend(EventTuiPromptAppend),
    EventTuiCommandExecute(EventTuiCommandExecute),
    EventTuiToastShow(EventTuiToastShow),
    EventTuiSessionSelect(EventTuiSessionSelect),
    EventInstallationUpdated(EventInstallationUpdated),
    EventInstallationUpdateAvailable(EventInstallationUpdateAvailable),
    EventProjectUpdated(EventProjectUpdated),
    EventServerInstanceDisposed(EventServerInstanceDisposed),
    EventFileEdited(EventFileEdited),
    EventLspClientDiagnostics(EventLspClientDiagnostics),
    EventPermissionAsked(EventPermissionAsked),
    EventPermissionReplied(EventPermissionReplied),
    EventSessionStatus(EventSessionStatus),
    EventSessionIdle(EventSessionIdle),
    EventQuestionAsked(EventQuestionAsked),
    EventQuestionReplied(EventQuestionReplied),
    EventQuestionRejected(EventQuestionRejected),
    EventTodoUpdated(EventTodoUpdated),
    EventPtyCreated(EventPtyCreated),
    EventPtyUpdated(EventPtyUpdated),
    EventPtyExited(EventPtyExited),
    EventPtyDeleted(EventPtyDeleted),
    EventMcpToolsChanged(EventMcpToolsChanged),
    EventFileWatcherUpdated(EventFileWatcherUpdated),
    EventLspUpdated(EventLspUpdated),
    EventCommandExecuted(EventCommandExecuted),
    EventVcsBranchUpdated(EventVcsBranchUpdated),
    EventMessageUpdated(EventMessageUpdated),
    EventMessageRemoved(EventMessageRemoved),
    EventMessagePartUpdated(EventMessagePartUpdated),
    EventMessagePartRemoved(EventMessagePartRemoved),
    EventSessionCompacted(EventSessionCompacted),
    EventSessionCreated(EventSessionCreated),
    EventSessionUpdated(EventSessionUpdated),
    EventSessionDeleted(EventSessionDeleted),
    EventSessionDiff(EventSessionDiff),
    EventSessionError(EventSessionError),
    EventServerConnected(EventServerConnected),
    EventGlobalDisposed(EventGlobalDisposed),
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
///
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum PermissionActionConfig {
    Ask,
    Allow,
    Deny,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionRuleConfig {
    PermissionActionConfig(PermissionActionConfig),
    PermissionObjectConfig(PermissionObjectConfig),
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionConfig {
    PermissionActionConfig(PermissionActionConfig),
}
///@deprecated Always uses stretch layout.
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum LayoutConfig {
    Auto,
    Stretch,
}
///
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "status")]
pub enum MCPStatus {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "failed")]
    #[display("Failed")]
    Failed { error: String },
    #[serde(rename = "needs_auth")]
    NeedsAuth,
    #[serde(rename = "needs_client_registration")]
    #[display("NeedsClientRegistration")]
    NeedsClientRegistration { error: String },
}
///
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "type")]
pub enum Auth {
    #[serde(rename = "oauth")]
    #[display("Oauth")]
    Oauth {
        refresh: String,
        access: String,
        expires: f64,
        #[serde(default, skip_serializing_if = "Option::is_none", rename = "accountId")]
        account_id: Option<String>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "enterpriseUrl"
        )]
        enterprise_url: Option<String>,
    },
    #[serde(rename = "api")]
    #[display("Api")]
    Api { key: String },
    #[serde(rename = "wellknown")]
    #[display("Wellknown")]
    Wellknown { key: String, token: String },
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiPromptAppend {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventTuiPromptAppendProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiCommandExecute {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventTuiCommandExecuteProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiToastShow {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventTuiToastShowProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiSessionSelect {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventTuiSessionSelectProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInstallationUpdated {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventInstallationUpdatedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInstallationUpdateAvailable {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventInstallationUpdateAvailableProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub worktree: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcs: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<ProjectIcon>,
    pub time: ProjectTime,
    pub sandboxes: Vec<String>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventProjectUpdated {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Project,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventServerInstanceDisposed {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventServerInstanceDisposedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFileEdited {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventFileEditedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLspClientDiagnostics {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventLspClientDiagnosticsProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRequest {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    pub permission: String,
    pub patterns: Vec<String>,
    pub metadata: serde_json::Value,
    pub always: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<PermissionRequestTool>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPermissionAsked {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: PermissionRequest,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPermissionReplied {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventPermissionRepliedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionStatus {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventSessionStatusProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionIdle {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventSessionIdleProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    ///Display text (1-5 words, concise)
    pub label: String,
    ///Explanation of choice
    pub description: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionInfo {
    ///Complete question
    pub question: String,
    ///Very short label (max 12 chars)
    pub header: String,
    ///Available choices
    pub options: Vec<QuestionOption>,
    ///Allow selecting multiple choices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiple: Option<bool>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionRequest {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    ///Questions to ask
    pub questions: Vec<QuestionInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<QuestionRequestTool>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventQuestionAsked {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: QuestionRequest,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionAnswer(pub Vec<String>);
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventQuestionReplied {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventQuestionRepliedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventQuestionRejected {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventQuestionRejectedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    ///Brief description of the task
    pub content: String,
    ///Current status of the task: pending, in_progress, completed, cancelled
    pub status: String,
    ///Priority level of the task: high, medium, low
    pub priority: String,
    ///Unique identifier for the todo item
    pub id: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTodoUpdated {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventTodoUpdatedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pty {
    pub id: String,
    pub title: String,
    pub command: String,
    pub args: Vec<String>,
    pub cwd: String,
    pub status: StringEnum,
    pub pid: f64,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyCreated {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventPtyCreatedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyUpdated {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventPtyUpdatedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyExited {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventPtyExitedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyDeleted {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventPtyDeletedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMcpToolsChanged {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventMcpToolsChangedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFileWatcherUpdated {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventFileWatcherUpdatedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLspUpdated {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: serde_json::Value,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCommandExecuted {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventCommandExecutedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventVcsBranchUpdated {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventVcsBranchUpdatedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiff {
    pub file: String,
    pub before: String,
    pub after: String,
    pub additions: f64,
    pub deletions: f64,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    pub role: String,
    pub time: UserMessageTime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<UserMessageSummary>,
    pub agent: String,
    pub model: UserMessageModel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAuthError {
    pub name: String,
    pub data: ProviderAuthErrorData,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownError {
    pub name: String,
    pub data: UnknownErrorData,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageOutputLengthError {
    pub name: String,
    pub data: serde_json::Value,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAbortedError {
    pub name: String,
    pub data: MessageAbortedErrorData,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIError {
    pub name: String,
    pub data: APIErrorData,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessage {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    pub role: String,
    pub time: AssistantMessageTime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Name>,
    #[serde(rename = "parentID")]
    pub parent_id: String,
    #[serde(rename = "modelID")]
    pub model_id: String,
    #[serde(rename = "providerID")]
    pub provider_id: String,
    pub mode: String,
    pub agent: String,
    pub path: AssistantMessagePath,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<bool>,
    pub cost: f64,
    pub tokens: AssistantMessageTokens,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finish: Option<String>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessageUpdated {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventMessageUpdatedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessageRemoved {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventMessageRemovedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPart {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synthetic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<TextPartTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningPart {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub time: ReasoningPartTime,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePartSourceText {
    pub value: String,
    pub start: i64,
    pub end: i64,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSource {
    pub text: FilePartSourceText,
    #[serde(rename = "type")]
    pub type_field: String,
    pub path: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: RangeStart,
    pub end: RangeEnd,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolSource {
    pub text: FilePartSourceText,
    #[serde(rename = "type")]
    pub type_field: String,
    pub path: String,
    pub range: Range,
    pub name: String,
    pub kind: i64,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSource {
    pub text: FilePartSourceText,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "clientName")]
    pub client_name: String,
    pub uri: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePart {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub mime: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<FilePartSource>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStatePending {
    pub status: String,
    pub input: serde_json::Value,
    pub raw: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStateRunning {
    pub status: String,
    pub input: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub time: ToolStateRunningTime,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStateCompleted {
    pub status: String,
    pub input: serde_json::Value,
    pub output: String,
    pub title: String,
    pub metadata: serde_json::Value,
    pub time: ToolStateCompletedTime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<FilePart>>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStateError {
    pub status: String,
    pub input: serde_json::Value,
    pub error: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub time: ToolStateErrorTime,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPart {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "callID")]
    pub call_id: String,
    pub tool: String,
    pub state: ToolState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepStartPart {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepFinishPart {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    pub cost: f64,
    pub tokens: StepFinishPartTokens,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotPart {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub snapshot: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchPart {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub hash: String,
    pub files: Vec<String>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPart {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<AgentPartSource>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPart {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub attempt: f64,
    pub error: APIError,
    pub time: RetryPartTime,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactionPart {
    pub id: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub auto: bool,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessagePartUpdated {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventMessagePartUpdatedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessagePartRemoved {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventMessagePartRemovedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionCompacted {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventSessionCompactedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRule {
    pub permission: String,
    pub pattern: String,
    pub action: PermissionAction,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRuleset(pub Vec<PermissionRule>);
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    #[serde(rename = "projectID")]
    pub project_id: String,
    pub directory: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parentID")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<SessionSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share: Option<SessionShare>,
    pub title: String,
    pub version: String,
    pub time: SessionTime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<PermissionRuleset>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revert: Option<SessionRevert>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionCreated {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventSessionCreatedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionUpdated {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventSessionUpdatedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionDeleted {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventSessionDeletedProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionDiff {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventSessionDiffProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionError {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: EventSessionErrorProperties,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventServerConnected {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: serde_json::Value,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGlobalDisposed {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: serde_json::Value,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalEvent {
    pub directory: String,
    pub payload: Event,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadRequestError {
    pub data: serde_json::Value,
    pub errors: Vec<serde_json::Value>,
    pub success: bool,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotFoundError {
    pub name: String,
    pub data: NotFoundErrorData,
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
    pub cors: Option<Vec<String>>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionObjectConfig {}
///
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
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub models: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blacklist: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<ProviderConfigOptions>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpLocalConfig {
    ///Type of MCP server connection
    #[serde(rename = "type")]
    pub type_field: String,
    ///Command and arguments to run the MCP server
    pub command: Vec<String>,
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
///
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
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRemoteConfig {
    ///Type of MCP server connection
    #[serde(rename = "type")]
    pub type_field: String,
    ///URL of the remote MCP server
    pub url: String,
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
///
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
    pub tui: Option<ConfigTui>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<ServerConfig>,
    ///Command configuration, see https://opencode.ai/docs/commands
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watcher: Option<ConfigWatcher>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin: Option<Vec<String>>,
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
    pub disabled_providers: Option<Vec<String>>,
    ///When set, ONLY these providers will be enabled. All other providers will be ignored
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled_providers: Option<Vec<String>>,
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
    pub mode: Option<ConfigMode>,
    ///Agent configuration, see https://opencode.ai/docs/agent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<ConfigAgent>,
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
    pub instructions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<LayoutConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<PermissionConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<ConfigEnterprise>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compaction: Option<ConfigCompaction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experimental: Option<ConfigExperimental>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolIDs(pub Vec<String>);
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolListItem {
    pub id: String,
    pub description: String,
    pub parameters: serde_json::Value,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolList(pub Vec<ToolListItem>);
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    pub home: String,
    pub state: String,
    pub config: String,
    pub worktree: String,
    pub directory: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worktree {
    pub name: String,
    pub branch: String,
    pub directory: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorktreeCreateInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startCommand")]
    pub start_command: Option<String>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VcsInfo {
    pub branch: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPartInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synthetic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<TextPartInputTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePartInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub mime: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<FilePartSource>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPartInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<AgentPartInputSource>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtaskPartInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub prompt: String,
    pub description: String,
    pub agent: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcp: Option<bool>,
    pub template: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtask: Option<bool>,
    pub hints: Vec<String>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    #[serde(rename = "providerID")]
    pub provider_id: String,
    pub api: ModelApi,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    pub capabilities: ModelCapabilities,
    pub cost: ModelCost,
    pub limit: ModelLimit,
    pub status: StringEnum,
    pub options: serde_json::Value,
    pub headers: serde_json::Value,
    pub release_date: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variants: Option<serde_json::Value>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    pub name: String,
    pub source: StringEnum,
    pub env: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub options: serde_json::Value,
    pub models: serde_json::Value,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAuthMethod {
    #[serde(rename = "type")]
    pub type_field: serde_json::Value,
    pub label: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAuthAuthorization {
    pub url: String,
    pub method: serde_json::Value,
    pub instructions: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub kind: f64,
    pub location: SymbolLocation,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub absolute: String,
    #[serde(rename = "type")]
    pub type_field: StringEnum,
    pub ignored: bool,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContent {
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<FileContentPatch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mimeType")]
    pub mime_type: Option<String>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub path: String,
    pub added: i64,
    pub removed: i64,
    pub status: StringEnum,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub mode: StringEnum,
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
    pub permission: PermissionRuleset,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<AgentModel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    pub options: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatusConnected {
    pub status: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatusDisabled {
    pub status: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatusFailed {
    pub status: String,
    pub error: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatusNeedsAuth {
    pub status: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatusNeedsClientRegistration {
    pub status: String,
    pub error: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResource {
    pub name: String,
    pub uri: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mimeType")]
    pub mime_type: Option<String>,
    pub client: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LSPStatus {
    pub id: String,
    pub name: String,
    pub root: String,
    pub status: serde_json::Value,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatterStatus {
    pub name: String,
    pub extensions: Vec<String>,
    pub enabled: bool,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth {
    #[serde(rename = "type")]
    pub type_field: String,
    pub refresh: String,
    pub access: String,
    pub expires: f64,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accountId")]
    pub account_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enterpriseUrl")]
    pub enterprise_url: Option<String>,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuth {
    #[serde(rename = "type")]
    pub type_field: String,
    pub key: String,
}
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellKnownAuth {
    #[serde(rename = "type")]
    pub type_field: String,
    pub key: String,
    pub token: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalHealthGetResponse {
    pub healthy: bool,
    pub version: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectProjectidPatchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<InlineObject>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtyPostRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtyPtyidPutRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<InlineObject>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionPostRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parentID")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<PermissionRuleset>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidPatchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<InlineObject>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidInitPostRequest {
    #[serde(rename = "modelID")]
    pub model_id: String,
    #[serde(rename = "providerID")]
    pub provider_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidForkPostRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidSummarizePostRequest {
    #[serde(rename = "providerID")]
    pub provider_id: String,
    #[serde(rename = "modelID")]
    pub model_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidMessageGetResponse {
    pub info: Message,
    pub parts: Vec<Part>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidMessagePostRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<InlineObject>,
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
    pub parts: Vec<Type>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidMessagePostResponse {
    pub info: AssistantMessage,
    pub parts: Vec<Part>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidMessageMessageidGetResponse {
    pub info: Message,
    pub parts: Vec<Part>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidPromptAsyncPostRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<InlineObject>,
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
    pub parts: Vec<Type>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidCommandPostRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub arguments: String,
    pub command: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Type>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidCommandPostResponse {
    pub info: AssistantMessage,
    pub parts: Vec<Part>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidShellPostRequest {
    pub agent: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<InlineObject>,
    pub command: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidRevertPostRequest {
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "partID")]
    pub part_id: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidPermissionsPermissionidPostRequest {
    pub response: StringEnum,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRequestidReplyPostRequest {
    pub reply: StringEnum,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionRequestidReplyPostRequest {
    ///User answers in order of questions (each answer is an array of selected labels)
    pub answers: Vec<QuestionAnswer>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProvidersGetResponse {
    pub providers: Vec<Provider>,
    pub default: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderGetResponse {
    pub all: Vec<InlineObject>,
    pub default: serde_json::Value,
    pub connected: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderProvideridOauthAuthorizePostRequest {
    ///Auth method index
    pub method: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderProvideridOauthCallbackPostRequest {
    ///Auth method index
    pub method: f64,
    ///OAuth authorization code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindGetResponse {
    pub path: FindGetResponsePath,
    pub lines: FindGetResponseLines,
    pub line_number: f64,
    pub absolute_offset: f64,
    pub submatches: Vec<InlineObject>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogPostRequest {
    ///Service name for the log entry
    pub service: String,
    ///Log level
    pub level: StringEnum,
    ///Log message
    pub message: String,
    ///Additional metadata for the log entry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPostRequest {
    pub name: String,
    pub config: Type,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpNameAuthPostResponse {
    ///URL to open in browser for authorization
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpNameAuthDeleteResponse {
    pub success: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpNameAuthCallbackPostRequest {
    ///Authorization code from OAuth callback
    pub code: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiAppendPromptPostRequest {
    pub text: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiExecuteCommandPostRequest {
    pub command: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiShowToastPostRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub message: String,
    pub variant: StringEnum,
    ///Duration in milliseconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiSelectSessionPostRequest {
    ///Session ID to navigate to
    #[serde(rename = "sessionID")]
    pub session_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiControlNextGetResponse {
    pub path: String,
    pub body: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum StringEnum {
    Info,
    Success,
    Warning,
    Error,
}
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "name")]
pub enum Name {
    #[display("ProviderAuthError")]
    ProviderAuthError { data: ProviderAuthErrorData1 },
    #[display("UnknownError")]
    UnknownError { data: UnknownErrorData1 },
    #[display("MessageOutputLengthError")]
    MessageOutputLengthError { data: serde_json::Value },
    #[display("MessageAbortedError")]
    MessageAbortedError { data: MessageAbortedErrorData1 },
    #[display("APIError")]
    APIError { data: APIErrorData1 },
}
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "type")]
pub enum Type {
    #[serde(rename = "text")]
    #[display("Text")]
    Text {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        text: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        synthetic: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ignored: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        time: Option<TextPartInputTime1>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<serde_json::Value>,
    },
    #[serde(rename = "file")]
    #[display("File")]
    File {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        mime: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
        url: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source: Option<FilePartSource>,
    },
    #[serde(rename = "agent")]
    #[display("Agent")]
    Agent {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source: Option<AgentPartInputSource1>,
    },
    #[serde(rename = "subtask")]
    #[display("Subtask")]
    Subtask {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        prompt: String,
        description: String,
        agent: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        command: Option<String>,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiPromptAppendProperties {
    pub text: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiCommandExecuteProperties {
    pub command: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiToastShowProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub message: String,
    pub variant: StringEnum,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiSessionSelectProperties {
    #[serde(rename = "sessionID")]
    pub session_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInstallationUpdatedProperties {
    pub version: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInstallationUpdateAvailableProperties {
    pub version: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectIcon {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTime {
    pub created: f64,
    pub updated: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initialized: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventServerInstanceDisposedProperties {
    pub directory: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFileEditedProperties {
    pub file: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLspClientDiagnosticsProperties {
    #[serde(rename = "serverID")]
    pub server_id: String,
    pub path: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRequestTool {
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "callID")]
    pub call_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPermissionRepliedProperties {
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "requestID")]
    pub request_id: String,
    pub reply: StringEnum,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionStatusProperties {
    #[serde(rename = "sessionID")]
    pub session_id: String,
    pub status: SessionStatus,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionIdleProperties {
    #[serde(rename = "sessionID")]
    pub session_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionRequestTool {
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "callID")]
    pub call_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventQuestionRepliedProperties {
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "requestID")]
    pub request_id: String,
    pub answers: Vec<QuestionAnswer>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventQuestionRejectedProperties {
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "requestID")]
    pub request_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTodoUpdatedProperties {
    #[serde(rename = "sessionID")]
    pub session_id: String,
    pub todos: Vec<Todo>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyCreatedProperties {
    pub info: Pty,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyUpdatedProperties {
    pub info: Pty,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyExitedProperties {
    pub id: String,
    #[serde(rename = "exitCode")]
    pub exit_code: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyDeletedProperties {
    pub id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMcpToolsChangedProperties {
    pub server: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFileWatcherUpdatedProperties {
    pub file: String,
    pub event: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCommandExecutedProperties {
    pub name: String,
    #[serde(rename = "sessionID")]
    pub session_id: String,
    pub arguments: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventVcsBranchUpdatedProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessageTime {
    pub created: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessageSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    pub diffs: Vec<FileDiff>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessageModel {
    #[serde(rename = "providerID")]
    pub provider_id: String,
    #[serde(rename = "modelID")]
    pub model_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAuthErrorData {
    #[serde(rename = "providerID")]
    pub provider_id: String,
    pub message: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownErrorData {
    pub message: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAbortedErrorData {
    pub message: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIErrorData {
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusCode")]
    pub status_code: Option<f64>,
    #[serde(rename = "isRetryable")]
    pub is_retryable: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "responseHeaders"
    )]
    pub response_headers: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseBody")]
    pub response_body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessageTime {
    pub created: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessagePath {
    pub cwd: String,
    pub root: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessageTokensCache {
    pub read: f64,
    pub write: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessageTokens {
    pub input: f64,
    pub output: f64,
    pub reasoning: f64,
    pub cache: AssistantMessageTokensCache,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessageUpdatedProperties {
    pub info: Message,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessageRemovedProperties {
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPartTime {
    pub start: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningPartTime {
    pub start: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeStart {
    pub line: f64,
    pub character: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeEnd {
    pub line: f64,
    pub character: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStateRunningTime {
    pub start: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStateCompletedTime {
    pub start: f64,
    pub end: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compacted: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStateErrorTime {
    pub start: f64,
    pub end: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepFinishPartTokensCache {
    pub read: f64,
    pub write: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepFinishPartTokens {
    pub input: f64,
    pub output: f64,
    pub reasoning: f64,
    pub cache: StepFinishPartTokensCache,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPartSource {
    pub value: String,
    pub start: i64,
    pub end: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPartTime {
    pub created: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessagePartUpdatedProperties {
    pub part: Part,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessagePartRemovedProperties {
    #[serde(rename = "sessionID")]
    pub session_id: String,
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(rename = "partID")]
    pub part_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionCompactedProperties {
    #[serde(rename = "sessionID")]
    pub session_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub additions: f64,
    pub deletions: f64,
    pub files: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diffs: Option<Vec<FileDiff>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionShare {
    pub url: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTime {
    pub created: f64,
    pub updated: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compacting: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRevert {
    #[serde(rename = "messageID")]
    pub message_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "partID")]
    pub part_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionCreatedProperties {
    pub info: Session,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionUpdatedProperties {
    pub info: Session,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionDeletedProperties {
    pub info: Session,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionDiffProperties {
    #[serde(rename = "sessionID")]
    pub session_id: String,
    pub diff: Vec<FileDiff>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionErrorProperties {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionID")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Name>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotFoundErrorData {
    pub message: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfigOptions {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiKey")]
    pub api_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "baseURL")]
    pub base_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enterpriseUrl")]
    pub enterprise_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "setCacheKey")]
    pub set_cache_key: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigTuiScrollAcceleration {
    pub enabled: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigTui {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll_speed: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll_acceleration: Option<ConfigTuiScrollAcceleration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff_style: Option<StringEnum>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigWatcher {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<String>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<AgentConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<AgentConfig>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigAgent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<AgentConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<AgentConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub general: Option<AgentConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explore: Option<AgentConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<AgentConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<AgentConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compaction: Option<AgentConfig>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigEnterprise {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigCompaction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prune: Option<bool>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineObject {
    pub command: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigExperimentalHook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_edited: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_completed: Option<Vec<InlineObject>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigExperimental {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hook: Option<ConfigExperimentalHook>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "chatMaxRetries")]
    pub chat_max_retries: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_paste_summary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_tool: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openTelemetry")]
    pub open_telemetry: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_tools: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continue_loop_on_deny: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcp_timeout: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPartInputTime {
    pub start: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPartInputSource {
    pub value: String,
    pub start: i64,
    pub end: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelApi {
    pub id: String,
    pub url: String,
    pub npm: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCapabilitiesInput {
    pub text: bool,
    pub audio: bool,
    pub image: bool,
    pub video: bool,
    pub pdf: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCapabilitiesOutput {
    pub text: bool,
    pub audio: bool,
    pub image: bool,
    pub video: bool,
    pub pdf: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCapabilities {
    pub temperature: bool,
    pub reasoning: bool,
    pub attachment: bool,
    pub toolcall: bool,
    pub input: ModelCapabilitiesInput,
    pub output: ModelCapabilitiesOutput,
    pub interleaved: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCostCache {
    pub read: f64,
    pub write: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCostExperimentalOver200KCache {
    pub read: f64,
    pub write: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCostExperimentalOver200K {
    pub input: f64,
    pub output: f64,
    pub cache: ModelCostExperimentalOver200KCache,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCost {
    pub input: f64,
    pub output: f64,
    pub cache: ModelCostCache,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "experimentalOver200K"
    )]
    pub experimental_over200_k: Option<ModelCostExperimentalOver200K>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelLimit {
    pub context: f64,
    pub output: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolLocation {
    pub uri: String,
    pub range: Range,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineObject1 {
    #[serde(rename = "oldStart")]
    pub old_start: f64,
    #[serde(rename = "oldLines")]
    pub old_lines: f64,
    #[serde(rename = "newStart")]
    pub new_start: f64,
    #[serde(rename = "newLines")]
    pub new_lines: f64,
    pub lines: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContentPatch {
    #[serde(rename = "oldFileName")]
    pub old_file_name: String,
    #[serde(rename = "newFileName")]
    pub new_file_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "oldHeader")]
    pub old_header: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "newHeader")]
    pub new_header: Option<String>,
    pub hunks: Vec<InlineObject1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentModel {
    #[serde(rename = "modelID")]
    pub model_id: String,
    #[serde(rename = "providerID")]
    pub provider_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindGetResponsePath {
    pub text: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindGetResponseLines {
    pub text: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineObjectMatch {
    pub text: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAuthErrorData1 {
    #[serde(rename = "providerID")]
    pub provider_id: String,
    pub message: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownErrorData1 {
    pub message: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAbortedErrorData1 {
    pub message: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIErrorData1 {
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusCode")]
    pub status_code: Option<f64>,
    #[serde(rename = "isRetryable")]
    pub is_retryable: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "responseHeaders"
    )]
    pub response_headers: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseBody")]
    pub response_body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPartInputTime1 {
    pub start: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPartInputSource1 {
    pub value: String,
    pub start: i64,
    pub end: i64,
}
pub trait ApiService: ::oapi_universal_gen::OapiRequester {
    /**ENDPOINT Get /global/health
Get health information about the OpenCode server.
*/
    fn global_health_get(
        &self,
    ) -> impl Future<Output = Result<GlobalHealthGetResponse, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = Vec::new();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/global/health",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<GlobalHealthGetResponse>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
        }
    }
    /**ENDPOINT Get /global/event
Subscribe to global events from the OpenCode system using server-sent events.
*/
    fn global_event_get(
        &self,
    ) -> impl Future<Output = Result<(), Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = Vec::new();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/global/event",
                &query_params,
            );
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            Ok(())
        }
    }
    /**ENDPOINT Post /global/dispose
Clean up and dispose all OpenCode instances, releasing all resources.
*/
    fn global_dispose_post(
        &self,
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = Vec::new();
            let uri = ::oapi_universal_gen::UrlBuilder::build(
                "/global/dispose",
                &query_params,
            );
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<Project>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/project", &query_params);
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<Project>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Project, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Project>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<ProjectProjectidPatchRequest>,
    ) -> impl Future<Output = Result<Project, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Patch, &uri, b)?
            } else {
                self.create_request(RequestType::Patch, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Project>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<Pty>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/pty", &query_params);
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<Pty>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
        }
    }
    /**ENDPOINT Post /pty
Create a new pseudo-terminal (PTY) session for running shell commands and processes.

# Arguments

- `query` directory
*/
    fn pty_post(
        &self,
        directory: Option<String>,
        body: Option<PtyPostRequest>,
    ) -> impl Future<Output = Result<Pty, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/pty", &query_params);
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Pty>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Pty, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Pty>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<PtyPtyidPutRequest>,
    ) -> impl Future<Output = Result<Pty, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Put, &uri, b)?
            } else {
                self.create_request(RequestType::Put, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Pty>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Delete, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Config, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/config", &query_params);
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Config>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<serde_json::Value>,
    ) -> impl Future<Output = Result<Config, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/config", &query_params);
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Patch, &uri, b)?
            } else {
                self.create_request(RequestType::Patch, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Config>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<ToolIDs, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<ToolIDs>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<ToolList, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<ToolList>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
        }
    }
    /**ENDPOINT Get /path
Retrieve the current working directory and related path information for the OpenCode instance.

# Arguments

- `query` directory
*/
    fn path_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Result<Path, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/path", &query_params);
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Path>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<String>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<String>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<serde_json::Value>,
    ) -> impl Future<Output = Result<Worktree, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Worktree>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<VcsInfo, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/vcs", &query_params);
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<VcsInfo>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<Session>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<Session>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<SessionPostRequest>,
    ) -> impl Future<Output = Result<Session, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/session", &query_params);
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Session>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<serde_json::Value, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<serde_json::Value>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Session, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Session>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<SessionSessionidPatchRequest>,
    ) -> impl Future<Output = Result<Session, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Patch, &uri, b)?
            } else {
                self.create_request(RequestType::Patch, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Session>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Delete, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<Session>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<Session>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<Todo>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<Todo>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<SessionSessionidInitPostRequest>,
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<SessionSessionidForkPostRequest>,
    ) -> impl Future<Output = Result<Session, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Session>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Session, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Session>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Session, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Delete, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Session>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<FileDiff>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<FileDiff>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<SessionSessionidSummarizePostRequest>,
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<
        Output = Result<
            Vec<SessionSessionidMessageGetResponse>,
            Self::RequesterErrorType,
        >,
    >
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<SessionSessionidMessageGetResponse>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<SessionSessionidMessagePostRequest>,
    ) -> impl Future<
        Output = Result<SessionSessionidMessagePostResponse, Self::RequesterErrorType>,
    >
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<SessionSessionidMessagePostResponse>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<
        Output = Result<
            SessionSessionidMessageMessageidGetResponse,
            Self::RequesterErrorType,
        >,
    >
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<
                    SessionSessionidMessageMessageidGetResponse,
                >(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<serde_json::Value>,
    ) -> impl Future<Output = Result<Part, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Patch, &uri, b)?
            } else {
                self.create_request(RequestType::Patch, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Part>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Delete, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<SessionSessionidPromptAsyncPostRequest>,
    ) -> impl Future<Output = Result<(), Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            Ok(())
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
        body: Option<SessionSessionidCommandPostRequest>,
    ) -> impl Future<
        Output = Result<SessionSessionidCommandPostResponse, Self::RequesterErrorType>,
    >
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<SessionSessionidCommandPostResponse>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<SessionSessionidShellPostRequest>,
    ) -> impl Future<Output = Result<AssistantMessage, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<AssistantMessage>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<SessionSessionidRevertPostRequest>,
    ) -> impl Future<Output = Result<Session, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Session>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Session, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Session>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<SessionSessionidPermissionsPermissionidPostRequest>,
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<PermissionRequestidReplyPostRequest>,
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<PermissionRequest>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<PermissionRequest>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<QuestionRequest>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<QuestionRequest>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<QuestionRequestidReplyPostRequest>,
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<Command>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/command", &query_params);
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<Command>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<
        Output = Result<ConfigProvidersGetResponse, Self::RequesterErrorType>,
    >
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<ConfigProvidersGetResponse>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<ProviderGetResponse, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<ProviderGetResponse>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<serde_json::Value, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<serde_json::Value>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<ProviderProvideridOauthAuthorizePostRequest>,
    ) -> impl Future<
        Output = Result<ProviderAuthAuthorization, Self::RequesterErrorType>,
    >
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<ProviderAuthAuthorization>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<ProviderProvideridOauthCallbackPostRequest>,
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<FindGetResponse>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<FindGetResponse>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<String>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<String>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<Symbol>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<Symbol>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<FileNode>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<FileNode>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<FileContent, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<FileContent>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<File>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<File>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
        }
    }
    /**ENDPOINT Post /log
Write a log entry to the server logs with specified level and metadata.

# Arguments

- `query` directory
*/
    fn log_post(
        &self,
        directory: Option<String>,
        body: Option<LogPostRequest>,
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/log", &query_params);
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<Agent>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/agent", &query_params);
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<Agent>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<serde_json::Value, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/mcp", &query_params);
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<serde_json::Value>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<McpPostRequest>,
    ) -> impl Future<Output = Result<serde_json::Value, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/mcp", &query_params);
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<serde_json::Value>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<McpNameAuthPostResponse, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<McpNameAuthPostResponse>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<
        Output = Result<McpNameAuthDeleteResponse, Self::RequesterErrorType>,
    >
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Delete, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<McpNameAuthDeleteResponse>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<McpNameAuthCallbackPostRequest>,
    ) -> impl Future<Output = Result<MCPStatus, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<MCPStatus>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<MCPStatus, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<MCPStatus>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<serde_json::Value, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<serde_json::Value>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<LSPStatus>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/lsp", &query_params);
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<LSPStatus>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<Vec<FormatterStatus>, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<Vec<FormatterStatus>>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<TuiAppendPromptPostRequest>,
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Post, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<TuiExecuteCommandPostRequest>,
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<TuiShowToastPostRequest>,
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<serde_json::Value>,
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<TuiSelectSessionPostRequest>,
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
    ) -> impl Future<
        Output = Result<TuiControlNextGetResponse, Self::RequesterErrorType>,
    >
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<TuiControlNextGetResponse>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<serde_json::Value>,
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Post, &uri, b)?
            } else {
                self.create_request(RequestType::Post, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
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
        body: Option<serde_json::Value>,
    ) -> impl Future<Output = Result<bool, Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
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
            let request = if let Some(b) = &body {
                self.create_request_with_body(RequestType::Put, &uri, b)?
            } else {
                self.create_request(RequestType::Put, &uri)?
            };
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            let content = response
                .response_content()
                .await
                .ok_or_else(|| OapiRequesterError::ResponseContentError.into())?;
            Ok(
                serde_json::from_str::<bool>(&content)
                    .map_err(|_e| OapiRequesterError::SerializationError.into())?,
            )
        }
    }
    /**ENDPOINT Get /event
Get events

# Arguments

- `query` directory
*/
    fn event_get(
        &self,
        directory: Option<String>,
    ) -> impl Future<Output = Result<(), Self::RequesterErrorType>>
    where
        Self::RequesterErrorType: From<OapiRequesterError>,
    {
        async move {
            let query_params: Vec<(String, String)> = vec![
                match directory.as_ref() { Some(v) => Some((String::from("directory"), v
                .to_string())), None => None, }
            ]
                .into_iter()
                .flatten()
                .collect();
            let uri = ::oapi_universal_gen::UrlBuilder::build("/event", &query_params);
            let request = self.create_request(RequestType::Get, &uri)?;
            let response = request.send_request().await?;
            if response.is_client_error() || response.is_server_error() {
                return Err(
                    OapiRequesterError::ClientOrServerError(response.status()).into(),
                );
            }
            Ok(())
        }
    }
}
