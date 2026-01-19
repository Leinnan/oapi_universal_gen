use serde::{Serialize, Deserialize};
use oapi_universal_gen::*;
use std::future::Future;
///Generated from schema 'SessionStatus'.
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "type")]
pub enum SessionStatus {
    #[serde(rename = "idle")]
    /// 'SessionStatus' variant.
    Idle,
    #[serde(rename = "retry")]
    #[display("Retry")]
    /// 'SessionStatus' variant.
    Retry {
        /// Required 'attempt' field.
        attempt: f64,
        /// Required 'message' field.
        message: String,
        /// Required 'next' field.
        next: f64,
    },
    #[serde(rename = "busy")]
    /// 'SessionStatus' variant.
    Busy,
}
///Generated from schema 'Message'.
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "role")]
pub enum Message {
    #[serde(rename = "user")]
    #[display("User")]
    /// 'Message' variant.
    User {
        /// Required 'id' field.
        id: String,
        /// Required 'sessionID' field.
        #[serde(rename = "sessionID")]
        session_id: String,
        /// Required 'time' field.
        time: UserMessageTime,
        /// Optional 'summary' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        summary: Option<UserMessageSummary>,
        /// Required 'agent' field.
        agent: String,
        /// Required 'model' field.
        model: UserMessageModel,
        /// Optional 'system' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        system: Option<String>,
        /// Optional 'tools' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tools: Option<serde_json::Value>,
        /// Optional 'variant' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        variant: Option<String>,
    },
    #[serde(rename = "assistant")]
    #[display("Assistant")]
    /// 'Message' variant.
    Assistant {
        /// Required 'id' field.
        id: String,
        /// Required 'sessionID' field.
        #[serde(rename = "sessionID")]
        session_id: String,
        /// Required 'time' field.
        time: AssistantMessageTime,
        /// Optional 'error' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        error: Option<Name>,
        /// Required 'parentID' field.
        #[serde(rename = "parentID")]
        parent_id: String,
        /// Required 'modelID' field.
        #[serde(rename = "modelID")]
        model_id: String,
        /// Required 'providerID' field.
        #[serde(rename = "providerID")]
        provider_id: String,
        /// Required 'mode' field.
        mode: String,
        /// Required 'agent' field.
        agent: String,
        /// Required 'path' field.
        path: AssistantMessagePath,
        /// Optional 'summary' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        summary: Option<bool>,
        /// Required 'cost' field.
        cost: f64,
        /// Required 'tokens' field.
        tokens: AssistantMessageTokens,
        /// Optional 'finish' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        finish: Option<String>,
    },
}
///Generated from schema 'FilePartSource'.
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "type")]
pub enum FilePartSource {
    #[serde(rename = "file")]
    #[display("File")]
    /// 'FilePartSource' variant.
    File {
        /// Required 'text' field.
        text: FilePartSourceText,
        /// Required 'path' field.
        path: String,
    },
    #[serde(rename = "symbol")]
    #[display("Symbol")]
    /// 'FilePartSource' variant.
    Symbol {
        /// Required 'text' field.
        text: FilePartSourceText,
        /// Required 'path' field.
        path: String,
        /// Required 'range' field.
        range: Range,
        /// Required 'name' field.
        name: String,
        /// Required 'kind' field.
        kind: i64,
    },
    #[serde(rename = "resource")]
    #[display("Resource")]
    /// 'FilePartSource' variant.
    Resource {
        /// Required 'text' field.
        text: FilePartSourceText,
        /// Required 'clientName' field.
        #[serde(rename = "clientName")]
        client_name: String,
        /// Required 'uri' field.
        uri: String,
    },
}
///Generated from schema 'ToolState'.
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "status")]
pub enum ToolState {
    #[serde(rename = "pending")]
    #[display("Pending")]
    /// 'ToolState' variant.
    Pending {
        /// Required 'input' field.
        input: serde_json::Value,
        /// Required 'raw' field.
        raw: String,
    },
    #[serde(rename = "running")]
    #[display("Running")]
    /// 'ToolState' variant.
    Running {
        /// Required 'input' field.
        input: serde_json::Value,
        /// Optional 'title' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        /// Optional 'metadata' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<serde_json::Value>,
        /// Required 'time' field.
        time: ToolStateRunningTime,
    },
    #[serde(rename = "completed")]
    #[display("Completed")]
    /// 'ToolState' variant.
    Completed {
        /// Required 'input' field.
        input: serde_json::Value,
        /// Required 'output' field.
        output: String,
        /// Required 'title' field.
        title: String,
        /// Required 'metadata' field.
        metadata: serde_json::Value,
        /// Required 'time' field.
        time: ToolStateCompletedTime,
        /// Optional 'attachments' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        attachments: Option<Vec<FilePart>>,
    },
    #[serde(rename = "error")]
    #[display("Error")]
    /// 'ToolState' variant.
    Error {
        /// Required 'input' field.
        input: serde_json::Value,
        /// Required 'error' field.
        error: String,
        /// Optional 'metadata' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<serde_json::Value>,
        /// Required 'time' field.
        time: ToolStateErrorTime,
    },
}
///Generated from schema 'Part'.
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "type")]
pub enum Part {
    #[serde(rename = "text")]
    #[display("Text")]
    /// 'Part' variant.
    Text {
        /// Required 'id' field.
        id: String,
        /// Required 'sessionID' field.
        #[serde(rename = "sessionID")]
        session_id: String,
        /// Required 'messageID' field.
        #[serde(rename = "messageID")]
        message_id: String,
        /// Required 'text' field.
        text: String,
        /// Optional 'synthetic' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        synthetic: Option<bool>,
        /// Optional 'ignored' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ignored: Option<bool>,
        /// Optional 'time' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        time: Option<TextPartTime>,
        /// Optional 'metadata' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<serde_json::Value>,
    },
    #[serde(rename = "subtask")]
    #[display("Subtask")]
    /// 'Part' variant.
    Subtask {
        /// Required 'id' field.
        id: String,
        /// Required 'sessionID' field.
        #[serde(rename = "sessionID")]
        session_id: String,
        /// Required 'messageID' field.
        #[serde(rename = "messageID")]
        message_id: String,
        /// Required 'prompt' field.
        prompt: String,
        /// Required 'description' field.
        description: String,
        /// Required 'agent' field.
        agent: String,
        /// Optional 'command' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        command: Option<String>,
    },
    #[serde(rename = "reasoning")]
    #[display("Reasoning")]
    /// 'Part' variant.
    Reasoning {
        /// Required 'id' field.
        id: String,
        /// Required 'sessionID' field.
        #[serde(rename = "sessionID")]
        session_id: String,
        /// Required 'messageID' field.
        #[serde(rename = "messageID")]
        message_id: String,
        /// Required 'text' field.
        text: String,
        /// Optional 'metadata' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<serde_json::Value>,
        /// Required 'time' field.
        time: ReasoningPartTime,
    },
    #[serde(rename = "file")]
    #[display("File")]
    /// 'Part' variant.
    File {
        /// Required 'id' field.
        id: String,
        /// Required 'sessionID' field.
        #[serde(rename = "sessionID")]
        session_id: String,
        /// Required 'messageID' field.
        #[serde(rename = "messageID")]
        message_id: String,
        /// Required 'mime' field.
        mime: String,
        /// Optional 'filename' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
        /// Required 'url' field.
        url: String,
        /// Optional 'source' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source: Option<FilePartSource>,
    },
    #[serde(rename = "tool")]
    #[display("Tool")]
    /// 'Part' variant.
    Tool {
        /// Required 'id' field.
        id: String,
        /// Required 'sessionID' field.
        #[serde(rename = "sessionID")]
        session_id: String,
        /// Required 'messageID' field.
        #[serde(rename = "messageID")]
        message_id: String,
        /// Required 'callID' field.
        #[serde(rename = "callID")]
        call_id: String,
        /// Required 'tool' field.
        tool: String,
        /// Required 'state' field.
        state: ToolState,
        /// Optional 'metadata' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<serde_json::Value>,
    },
    #[serde(rename = "step-start")]
    #[display("StepStart")]
    /// 'Part' variant.
    StepStart {
        /// Required 'id' field.
        id: String,
        /// Required 'sessionID' field.
        #[serde(rename = "sessionID")]
        session_id: String,
        /// Required 'messageID' field.
        #[serde(rename = "messageID")]
        message_id: String,
        /// Optional 'snapshot' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        snapshot: Option<String>,
    },
    #[serde(rename = "step-finish")]
    #[display("StepFinish")]
    /// 'Part' variant.
    StepFinish {
        /// Required 'id' field.
        id: String,
        /// Required 'sessionID' field.
        #[serde(rename = "sessionID")]
        session_id: String,
        /// Required 'messageID' field.
        #[serde(rename = "messageID")]
        message_id: String,
        /// Required 'reason' field.
        reason: String,
        /// Optional 'snapshot' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        snapshot: Option<String>,
        /// Required 'cost' field.
        cost: f64,
        /// Required 'tokens' field.
        tokens: StepFinishPartTokens,
    },
    #[serde(rename = "snapshot")]
    #[display("Snapshot")]
    /// 'Part' variant.
    Snapshot {
        /// Required 'id' field.
        id: String,
        /// Required 'sessionID' field.
        #[serde(rename = "sessionID")]
        session_id: String,
        /// Required 'messageID' field.
        #[serde(rename = "messageID")]
        message_id: String,
        /// Required 'snapshot' field.
        snapshot: String,
    },
    #[serde(rename = "patch")]
    #[display("Patch")]
    /// 'Part' variant.
    Patch {
        /// Required 'id' field.
        id: String,
        /// Required 'sessionID' field.
        #[serde(rename = "sessionID")]
        session_id: String,
        /// Required 'messageID' field.
        #[serde(rename = "messageID")]
        message_id: String,
        /// Required 'hash' field.
        hash: String,
        /// Required 'files' field.
        files: Vec<String>,
    },
    #[serde(rename = "agent")]
    #[display("Agent")]
    /// 'Part' variant.
    Agent {
        /// Required 'id' field.
        id: String,
        /// Required 'sessionID' field.
        #[serde(rename = "sessionID")]
        session_id: String,
        /// Required 'messageID' field.
        #[serde(rename = "messageID")]
        message_id: String,
        /// Required 'name' field.
        name: String,
        /// Optional 'source' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source: Option<AgentPartSource>,
    },
    #[serde(rename = "retry")]
    #[display("Retry")]
    /// 'Part' variant.
    Retry {
        /// Required 'id' field.
        id: String,
        /// Required 'sessionID' field.
        #[serde(rename = "sessionID")]
        session_id: String,
        /// Required 'messageID' field.
        #[serde(rename = "messageID")]
        message_id: String,
        /// Required 'attempt' field.
        attempt: f64,
        /// Required 'error' field.
        error: APIError,
        /// Required 'time' field.
        time: RetryPartTime,
    },
    #[serde(rename = "compaction")]
    #[display("Compaction")]
    /// 'Part' variant.
    Compaction {
        /// Required 'id' field.
        id: String,
        /// Required 'sessionID' field.
        #[serde(rename = "sessionID")]
        session_id: String,
        /// Required 'messageID' field.
        #[serde(rename = "messageID")]
        message_id: String,
        /// Required 'auto' field.
        auto: bool,
    },
}
///Generated from schema 'PermissionAction'.
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum PermissionAction {
    Allow,
    Deny,
    Ask,
}
///Generated from schema 'Event'.
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
///Generated from schema 'PermissionActionConfig'.
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum PermissionActionConfig {
    Ask,
    Allow,
    Deny,
}
///Generated from schema 'PermissionRuleConfig'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionRuleConfig {
    PermissionActionConfig(PermissionActionConfig),
    PermissionObjectConfig(PermissionObjectConfig),
}
///Generated from schema 'PermissionConfig'.
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
///Generated from schema 'MCPStatus'.
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "status")]
pub enum MCPStatus {
    #[serde(rename = "connected")]
    /// 'MCPStatus' variant.
    Connected,
    #[serde(rename = "disabled")]
    /// 'MCPStatus' variant.
    Disabled,
    #[serde(rename = "failed")]
    #[display("Failed")]
    /// 'MCPStatus' variant.
    Failed {
        /// Required 'error' field.
        error: String,
    },
    #[serde(rename = "needs_auth")]
    /// 'MCPStatus' variant.
    NeedsAuth,
    #[serde(rename = "needs_client_registration")]
    #[display("NeedsClientRegistration")]
    /// 'MCPStatus' variant.
    NeedsClientRegistration {
        /// Required 'error' field.
        error: String,
    },
}
///Generated from schema 'Auth'.
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "type")]
pub enum Auth {
    #[serde(rename = "oauth")]
    #[display("Oauth")]
    /// 'Auth' variant.
    Oauth {
        /// Required 'refresh' field.
        refresh: String,
        /// Required 'access' field.
        access: String,
        /// Required 'expires' field.
        expires: f64,
        /// Optional 'accountId' field.
        #[serde(default, skip_serializing_if = "Option::is_none", rename = "accountId")]
        account_id: Option<String>,
        /// Optional 'enterpriseUrl' field.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "enterpriseUrl"
        )]
        enterprise_url: Option<String>,
    },
    #[serde(rename = "api")]
    #[display("Api")]
    /// 'Auth' variant.
    Api {
        /// Required 'key' field.
        key: String,
    },
    #[serde(rename = "wellknown")]
    #[display("Wellknown")]
    /// 'Auth' variant.
    Wellknown {
        /// Required 'key' field.
        key: String,
        /// Required 'token' field.
        token: String,
    },
}
///Generated from schema 'Event.tui.prompt.append'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiPromptAppend {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventTuiPromptAppendProperties,
}
///Generated from schema 'Event.tui.command.execute'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiCommandExecute {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventTuiCommandExecuteProperties,
}
///Generated from schema 'Event.tui.toast.show'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiToastShow {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventTuiToastShowProperties,
}
///Generated from schema 'Event.tui.session.select'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTuiSessionSelect {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventTuiSessionSelectProperties,
}
///Generated from schema 'Event.installation.updated'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInstallationUpdated {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventInstallationUpdatedProperties,
}
///Generated from schema 'Event.installation.update-available'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInstallationUpdateAvailable {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventInstallationUpdateAvailableProperties,
}
///Generated from schema 'Project'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Required 'id' field.
    pub id: String,
    /// Required 'worktree' field.
    pub worktree: String,
    /// Optional 'vcs' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcs: Option<String>,
    /// Optional 'name' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional 'icon' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<ProjectIcon>,
    /// Required 'time' field.
    pub time: ProjectTime,
    /// Required 'sandboxes' field.
    pub sandboxes: Vec<String>,
}
///Generated from schema 'Event.project.updated'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventProjectUpdated {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: Project,
}
///Generated from schema 'Event.server.instance.disposed'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventServerInstanceDisposed {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventServerInstanceDisposedProperties,
}
///Generated from schema 'Event.file.edited'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFileEdited {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventFileEditedProperties,
}
///Generated from schema 'Event.lsp.client.diagnostics'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLspClientDiagnostics {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventLspClientDiagnosticsProperties,
}
///Generated from schema 'PermissionRequest'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRequest {
    /// Required 'id' field.
    pub id: String,
    /// Required 'sessionID' field.
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Required 'permission' field.
    pub permission: String,
    /// Required 'patterns' field.
    pub patterns: Vec<String>,
    /// Required 'metadata' field.
    pub metadata: serde_json::Value,
    /// Required 'always' field.
    pub always: Vec<String>,
    /// Optional 'tool' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<PermissionRequestTool>,
}
///Generated from schema 'Event.permission.asked'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPermissionAsked {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: PermissionRequest,
}
///Generated from schema 'Event.permission.replied'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPermissionReplied {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventPermissionRepliedProperties,
}
///Generated from schema 'Event.session.status'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionStatus {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventSessionStatusProperties,
}
///Generated from schema 'Event.session.idle'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionIdle {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventSessionIdleProperties,
}
///Generated from schema 'QuestionOption'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    /// Display text (1-5 words, concise)
    pub label: String,
    /// Explanation of choice
    pub description: String,
}
///Generated from schema 'QuestionInfo'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionInfo {
    /// Complete question
    pub question: String,
    /// Very short label (max 12 chars)
    pub header: String,
    /// Available choices
    pub options: Vec<QuestionOption>,
    /// Allow selecting multiple choices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiple: Option<bool>,
}
///Generated from schema 'QuestionRequest'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionRequest {
    /// Required 'id' field.
    pub id: String,
    /// Required 'sessionID' field.
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Questions to ask
    pub questions: Vec<QuestionInfo>,
    /// Optional 'tool' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<QuestionRequestTool>,
}
///Generated from schema 'Event.question.asked'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventQuestionAsked {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: QuestionRequest,
}
///Generated from schema 'QuestionAnswer'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionAnswer(pub Vec<String>);
///Generated from schema 'Event.question.replied'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventQuestionReplied {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventQuestionRepliedProperties,
}
///Generated from schema 'Event.question.rejected'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventQuestionRejected {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventQuestionRejectedProperties,
}
///Generated from schema 'Todo'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    /// Brief description of the task
    pub content: String,
    /// Current status of the task: pending, in_progress, completed, cancelled
    pub status: String,
    /// Priority level of the task: high, medium, low
    pub priority: String,
    /// Unique identifier for the todo item
    pub id: String,
}
///Generated from schema 'Event.todo.updated'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTodoUpdated {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventTodoUpdatedProperties,
}
///Generated from schema 'Pty'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pty {
    /// Required 'id' field.
    pub id: String,
    /// Required 'title' field.
    pub title: String,
    /// Required 'command' field.
    pub command: String,
    /// Required 'args' field.
    pub args: Vec<String>,
    /// Required 'cwd' field.
    pub cwd: String,
    /// Required 'status' field.
    pub status: StringEnum,
    /// Required 'pid' field.
    pub pid: f64,
}
///Generated from schema 'Event.pty.created'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyCreated {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventPtyCreatedProperties,
}
///Generated from schema 'Event.pty.updated'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyUpdated {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventPtyUpdatedProperties,
}
///Generated from schema 'Event.pty.exited'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyExited {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventPtyExitedProperties,
}
///Generated from schema 'Event.pty.deleted'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPtyDeleted {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventPtyDeletedProperties,
}
///Generated from schema 'Event.mcp.tools.changed'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMcpToolsChanged {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventMcpToolsChangedProperties,
}
///Generated from schema 'Event.file.watcher.updated'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFileWatcherUpdated {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventFileWatcherUpdatedProperties,
}
///Generated from schema 'Event.lsp.updated'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLspUpdated {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: serde_json::Value,
}
///Generated from schema 'Event.command.executed'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCommandExecuted {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventCommandExecutedProperties,
}
///Generated from schema 'Event.vcs.branch.updated'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventVcsBranchUpdated {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventVcsBranchUpdatedProperties,
}
///Generated from schema 'FileDiff'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiff {
    /// Required 'file' field.
    pub file: String,
    /// Required 'before' field.
    pub before: String,
    /// Required 'after' field.
    pub after: String,
    /// Required 'additions' field.
    pub additions: f64,
    /// Required 'deletions' field.
    pub deletions: f64,
}
///Generated from schema 'UserMessage'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessage {
    /// Required 'id' field.
    pub id: String,
    /// Required 'sessionID' field.
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Required 'role' field.
    pub role: String,
    /// Required 'time' field.
    pub time: UserMessageTime,
    /// Optional 'summary' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<UserMessageSummary>,
    /// Required 'agent' field.
    pub agent: String,
    /// Required 'model' field.
    pub model: UserMessageModel,
    /// Optional 'system' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Optional 'tools' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
    /// Optional 'variant' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}
///Generated from schema 'ProviderAuthError'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAuthError {
    /// Required 'name' field.
    pub name: String,
    /// Required 'data' field.
    pub data: ProviderAuthErrorData,
}
///Generated from schema 'UnknownError'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnknownError {
    /// Required 'name' field.
    pub name: String,
    /// Required 'data' field.
    pub data: UnknownErrorData,
}
///Generated from schema 'MessageOutputLengthError'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageOutputLengthError {
    /// Required 'name' field.
    pub name: String,
    /// Required 'data' field.
    pub data: serde_json::Value,
}
///Generated from schema 'MessageAbortedError'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAbortedError {
    /// Required 'name' field.
    pub name: String,
    /// Required 'data' field.
    pub data: MessageAbortedErrorData,
}
///Generated from schema 'APIError'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIError {
    /// Required 'name' field.
    pub name: String,
    /// Required 'data' field.
    pub data: APIErrorData,
}
///Generated from schema 'AssistantMessage'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessage {
    /// Required 'id' field.
    pub id: String,
    /// Required 'sessionID' field.
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Required 'role' field.
    pub role: String,
    /// Required 'time' field.
    pub time: AssistantMessageTime,
    /// Optional 'error' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Name>,
    /// Required 'parentID' field.
    #[serde(rename = "parentID")]
    pub parent_id: String,
    /// Required 'modelID' field.
    #[serde(rename = "modelID")]
    pub model_id: String,
    /// Required 'providerID' field.
    #[serde(rename = "providerID")]
    pub provider_id: String,
    /// Required 'mode' field.
    pub mode: String,
    /// Required 'agent' field.
    pub agent: String,
    /// Required 'path' field.
    pub path: AssistantMessagePath,
    /// Optional 'summary' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<bool>,
    /// Required 'cost' field.
    pub cost: f64,
    /// Required 'tokens' field.
    pub tokens: AssistantMessageTokens,
    /// Optional 'finish' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finish: Option<String>,
}
///Generated from schema 'Event.message.updated'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessageUpdated {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventMessageUpdatedProperties,
}
///Generated from schema 'Event.message.removed'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessageRemoved {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventMessageRemovedProperties,
}
///Generated from schema 'TextPart'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPart {
    /// Required 'id' field.
    pub id: String,
    /// Required 'sessionID' field.
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Required 'messageID' field.
    #[serde(rename = "messageID")]
    pub message_id: String,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'text' field.
    pub text: String,
    /// Optional 'synthetic' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synthetic: Option<bool>,
    /// Optional 'ignored' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
    /// Optional 'time' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<TextPartTime>,
    /// Optional 'metadata' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
///Generated from schema 'ReasoningPart'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningPart {
    /// Required 'id' field.
    pub id: String,
    /// Required 'sessionID' field.
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Required 'messageID' field.
    #[serde(rename = "messageID")]
    pub message_id: String,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'text' field.
    pub text: String,
    /// Optional 'metadata' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Required 'time' field.
    pub time: ReasoningPartTime,
}
///Generated from schema 'FilePartSourceText'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePartSourceText {
    /// Required 'value' field.
    pub value: String,
    /// Required 'start' field.
    pub start: i64,
    /// Required 'end' field.
    pub end: i64,
}
///Generated from schema 'FileSource'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSource {
    /// Required 'text' field.
    pub text: FilePartSourceText,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'path' field.
    pub path: String,
}
///Generated from schema 'Range'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    /// Required 'start' field.
    pub start: RangeStart,
    /// Required 'end' field.
    pub end: RangeEnd,
}
///Generated from schema 'SymbolSource'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolSource {
    /// Required 'text' field.
    pub text: FilePartSourceText,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'path' field.
    pub path: String,
    /// Required 'range' field.
    pub range: Range,
    /// Required 'name' field.
    pub name: String,
    /// Required 'kind' field.
    pub kind: i64,
}
///Generated from schema 'ResourceSource'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSource {
    /// Required 'text' field.
    pub text: FilePartSourceText,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'clientName' field.
    #[serde(rename = "clientName")]
    pub client_name: String,
    /// Required 'uri' field.
    pub uri: String,
}
///Generated from schema 'FilePart'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePart {
    /// Required 'id' field.
    pub id: String,
    /// Required 'sessionID' field.
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Required 'messageID' field.
    #[serde(rename = "messageID")]
    pub message_id: String,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'mime' field.
    pub mime: String,
    /// Optional 'filename' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Required 'url' field.
    pub url: String,
    /// Optional 'source' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<FilePartSource>,
}
///Generated from schema 'ToolStatePending'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStatePending {
    /// Required 'status' field.
    pub status: String,
    /// Required 'input' field.
    pub input: serde_json::Value,
    /// Required 'raw' field.
    pub raw: String,
}
///Generated from schema 'ToolStateRunning'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStateRunning {
    /// Required 'status' field.
    pub status: String,
    /// Required 'input' field.
    pub input: serde_json::Value,
    /// Optional 'title' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional 'metadata' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Required 'time' field.
    pub time: ToolStateRunningTime,
}
///Generated from schema 'ToolStateCompleted'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStateCompleted {
    /// Required 'status' field.
    pub status: String,
    /// Required 'input' field.
    pub input: serde_json::Value,
    /// Required 'output' field.
    pub output: String,
    /// Required 'title' field.
    pub title: String,
    /// Required 'metadata' field.
    pub metadata: serde_json::Value,
    /// Required 'time' field.
    pub time: ToolStateCompletedTime,
    /// Optional 'attachments' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<FilePart>>,
}
///Generated from schema 'ToolStateError'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStateError {
    /// Required 'status' field.
    pub status: String,
    /// Required 'input' field.
    pub input: serde_json::Value,
    /// Required 'error' field.
    pub error: String,
    /// Optional 'metadata' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Required 'time' field.
    pub time: ToolStateErrorTime,
}
///Generated from schema 'ToolPart'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPart {
    /// Required 'id' field.
    pub id: String,
    /// Required 'sessionID' field.
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Required 'messageID' field.
    #[serde(rename = "messageID")]
    pub message_id: String,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'callID' field.
    #[serde(rename = "callID")]
    pub call_id: String,
    /// Required 'tool' field.
    pub tool: String,
    /// Required 'state' field.
    pub state: ToolState,
    /// Optional 'metadata' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
///Generated from schema 'StepStartPart'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepStartPart {
    /// Required 'id' field.
    pub id: String,
    /// Required 'sessionID' field.
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Required 'messageID' field.
    #[serde(rename = "messageID")]
    pub message_id: String,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Optional 'snapshot' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
}
///Generated from schema 'StepFinishPart'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepFinishPart {
    /// Required 'id' field.
    pub id: String,
    /// Required 'sessionID' field.
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Required 'messageID' field.
    #[serde(rename = "messageID")]
    pub message_id: String,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'reason' field.
    pub reason: String,
    /// Optional 'snapshot' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    /// Required 'cost' field.
    pub cost: f64,
    /// Required 'tokens' field.
    pub tokens: StepFinishPartTokens,
}
///Generated from schema 'SnapshotPart'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotPart {
    /// Required 'id' field.
    pub id: String,
    /// Required 'sessionID' field.
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Required 'messageID' field.
    #[serde(rename = "messageID")]
    pub message_id: String,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'snapshot' field.
    pub snapshot: String,
}
///Generated from schema 'PatchPart'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchPart {
    /// Required 'id' field.
    pub id: String,
    /// Required 'sessionID' field.
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Required 'messageID' field.
    #[serde(rename = "messageID")]
    pub message_id: String,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'hash' field.
    pub hash: String,
    /// Required 'files' field.
    pub files: Vec<String>,
}
///Generated from schema 'AgentPart'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPart {
    /// Required 'id' field.
    pub id: String,
    /// Required 'sessionID' field.
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Required 'messageID' field.
    #[serde(rename = "messageID")]
    pub message_id: String,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'name' field.
    pub name: String,
    /// Optional 'source' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<AgentPartSource>,
}
///Generated from schema 'RetryPart'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPart {
    /// Required 'id' field.
    pub id: String,
    /// Required 'sessionID' field.
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Required 'messageID' field.
    #[serde(rename = "messageID")]
    pub message_id: String,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'attempt' field.
    pub attempt: f64,
    /// Required 'error' field.
    pub error: APIError,
    /// Required 'time' field.
    pub time: RetryPartTime,
}
///Generated from schema 'CompactionPart'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactionPart {
    /// Required 'id' field.
    pub id: String,
    /// Required 'sessionID' field.
    #[serde(rename = "sessionID")]
    pub session_id: String,
    /// Required 'messageID' field.
    #[serde(rename = "messageID")]
    pub message_id: String,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'auto' field.
    pub auto: bool,
}
///Generated from schema 'Event.message.part.updated'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessagePartUpdated {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventMessagePartUpdatedProperties,
}
///Generated from schema 'Event.message.part.removed'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessagePartRemoved {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventMessagePartRemovedProperties,
}
///Generated from schema 'Event.session.compacted'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionCompacted {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventSessionCompactedProperties,
}
///Generated from schema 'PermissionRule'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRule {
    /// Required 'permission' field.
    pub permission: String,
    /// Required 'pattern' field.
    pub pattern: String,
    /// Required 'action' field.
    pub action: PermissionAction,
}
///Generated from schema 'PermissionRuleset'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRuleset(pub Vec<PermissionRule>);
///Generated from schema 'Session'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Required 'id' field.
    pub id: String,
    /// Required 'projectID' field.
    #[serde(rename = "projectID")]
    pub project_id: String,
    /// Required 'directory' field.
    pub directory: String,
    /// Optional 'parentID' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parentID")]
    pub parent_id: Option<String>,
    /// Optional 'summary' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<SessionSummary>,
    /// Optional 'share' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share: Option<SessionShare>,
    /// Required 'title' field.
    pub title: String,
    /// Required 'version' field.
    pub version: String,
    /// Required 'time' field.
    pub time: SessionTime,
    /// Optional 'permission' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<PermissionRuleset>,
    /// Optional 'revert' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revert: Option<SessionRevert>,
}
///Generated from schema 'Event.session.created'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionCreated {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventSessionCreatedProperties,
}
///Generated from schema 'Event.session.updated'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionUpdated {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventSessionUpdatedProperties,
}
///Generated from schema 'Event.session.deleted'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionDeleted {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventSessionDeletedProperties,
}
///Generated from schema 'Event.session.diff'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionDiff {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventSessionDiffProperties,
}
///Generated from schema 'Event.session.error'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSessionError {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: EventSessionErrorProperties,
}
///Generated from schema 'Event.server.connected'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventServerConnected {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: serde_json::Value,
}
///Generated from schema 'Event.global.disposed'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGlobalDisposed {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'properties' field.
    pub properties: serde_json::Value,
}
///Generated from schema 'GlobalEvent'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalEvent {
    /// Required 'directory' field.
    pub directory: String,
    /// Required 'payload' field.
    pub payload: Event,
}
///Generated from schema 'BadRequestError'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadRequestError {
    /// Required 'data' field.
    pub data: serde_json::Value,
    /// Required 'errors' field.
    pub errors: Vec<serde_json::Value>,
    /// Required 'success' field.
    pub success: bool,
}
///Generated from schema 'NotFoundError'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotFoundError {
    /// Required 'name' field.
    pub name: String,
    /// Required 'data' field.
    pub data: NotFoundErrorData,
}
///Custom keybind configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeybindsConfig {
    /// Leader key for keybind combinations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader: Option<String>,
    /// Exit the application
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_exit: Option<String>,
    /// Open external editor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editor_open: Option<String>,
    /// List available themes
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme_list: Option<String>,
    /// Toggle sidebar
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sidebar_toggle: Option<String>,
    /// Toggle session scrollbar
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scrollbar_toggle: Option<String>,
    /// Toggle username visibility
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username_toggle: Option<String>,
    /// View status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_view: Option<String>,
    /// Export session to editor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_export: Option<String>,
    /// Create a new session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_new: Option<String>,
    /// List all sessions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_list: Option<String>,
    /// Show session timeline
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_timeline: Option<String>,
    /// Fork session from message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_fork: Option<String>,
    /// Rename session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_rename: Option<String>,
    /// Share current session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_share: Option<String>,
    /// Unshare current session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_unshare: Option<String>,
    /// Interrupt current session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_interrupt: Option<String>,
    /// Compact the session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_compact: Option<String>,
    /// Scroll messages up by one page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_page_up: Option<String>,
    /// Scroll messages down by one page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_page_down: Option<String>,
    /// Scroll messages up by half page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_half_page_up: Option<String>,
    /// Scroll messages down by half page
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_half_page_down: Option<String>,
    /// Navigate to first message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_first: Option<String>,
    /// Navigate to last message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_last: Option<String>,
    /// Navigate to next message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_next: Option<String>,
    /// Navigate to previous message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_previous: Option<String>,
    /// Navigate to last user message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_last_user: Option<String>,
    /// Copy message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_copy: Option<String>,
    /// Undo message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_undo: Option<String>,
    /// Redo message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_redo: Option<String>,
    /// Toggle code block concealment in messages
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages_toggle_conceal: Option<String>,
    /// Toggle tool details visibility
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_details: Option<String>,
    /// List available models
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_list: Option<String>,
    /// Next recently used model
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_cycle_recent: Option<String>,
    /// Previous recently used model
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_cycle_recent_reverse: Option<String>,
    /// Next favorite model
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_cycle_favorite: Option<String>,
    /// Previous favorite model
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_cycle_favorite_reverse: Option<String>,
    /// List available commands
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_list: Option<String>,
    /// List agents
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_list: Option<String>,
    /// Next agent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_cycle: Option<String>,
    /// Previous agent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_cycle_reverse: Option<String>,
    /// Cycle model variants
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant_cycle: Option<String>,
    /// Clear input field
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_clear: Option<String>,
    /// Paste from clipboard
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_paste: Option<String>,
    /// Submit input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_submit: Option<String>,
    /// Insert newline in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_newline: Option<String>,
    /// Move cursor left in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_move_left: Option<String>,
    /// Move cursor right in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_move_right: Option<String>,
    /// Move cursor up in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_move_up: Option<String>,
    /// Move cursor down in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_move_down: Option<String>,
    /// Select left in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_left: Option<String>,
    /// Select right in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_right: Option<String>,
    /// Select up in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_up: Option<String>,
    /// Select down in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_down: Option<String>,
    /// Move to start of line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_line_home: Option<String>,
    /// Move to end of line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_line_end: Option<String>,
    /// Select to start of line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_line_home: Option<String>,
    /// Select to end of line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_line_end: Option<String>,
    /// Move to start of visual line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_visual_line_home: Option<String>,
    /// Move to end of visual line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_visual_line_end: Option<String>,
    /// Select to start of visual line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_visual_line_home: Option<String>,
    /// Select to end of visual line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_visual_line_end: Option<String>,
    /// Move to start of buffer in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_buffer_home: Option<String>,
    /// Move to end of buffer in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_buffer_end: Option<String>,
    /// Select to start of buffer in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_buffer_home: Option<String>,
    /// Select to end of buffer in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_buffer_end: Option<String>,
    /// Delete line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_delete_line: Option<String>,
    /// Delete to end of line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_delete_to_line_end: Option<String>,
    /// Delete to start of line in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_delete_to_line_start: Option<String>,
    /// Backspace in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_backspace: Option<String>,
    /// Delete character in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_delete: Option<String>,
    /// Undo in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_undo: Option<String>,
    /// Redo in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_redo: Option<String>,
    /// Move word forward in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_word_forward: Option<String>,
    /// Move word backward in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_word_backward: Option<String>,
    /// Select word forward in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_word_forward: Option<String>,
    /// Select word backward in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_select_word_backward: Option<String>,
    /// Delete word forward in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_delete_word_forward: Option<String>,
    /// Delete word backward in input
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_delete_word_backward: Option<String>,
    /// Previous history item
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_previous: Option<String>,
    /// Next history item
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_next: Option<String>,
    /// Next child session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_child_cycle: Option<String>,
    /// Previous child session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_child_cycle_reverse: Option<String>,
    /// Go to parent session
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_parent: Option<String>,
    /// Suspend terminal
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal_suspend: Option<String>,
    /// Toggle terminal title
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal_title_toggle: Option<String>,
    /// Toggle tips on home screen
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tips_toggle: Option<String>,
}
///Server configuration for opencode serve and web commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Port to listen on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// Hostname to listen on
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Enable mDNS service discovery
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdns: Option<bool>,
    /// Additional domains to allow for CORS
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cors: Option<Vec<String>>,
}
///Generated from schema 'PermissionObjectConfig'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionObjectConfig {}
///Generated from schema 'AgentConfig'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Optional 'model' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Optional 'temperature' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Optional 'top_p' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// Optional 'prompt' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// @deprecated Use 'permission' field instead
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
    /// Optional 'disable' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
    /// Description of when to use the agent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional 'mode' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<StringEnum>,
    /// Hide this subagent from the @ autocomplete menu (default: false, only applies to mode: subagent)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// Optional 'options' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    /// Hex color code for the agent (e.g., #FF5733)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Maximum number of agentic iterations before forcing text-only response
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
    /// @deprecated Use 'steps' field instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSteps")]
    pub max_steps: Option<i64>,
    /// Optional 'permission' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<PermissionConfig>,
}
///Generated from schema 'ProviderConfig'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Optional 'api' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    /// Optional 'name' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional 'env' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Optional 'id' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional 'npm' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npm: Option<String>,
    /// Optional 'models' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub models: Option<serde_json::Value>,
    /// Optional 'whitelist' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<Vec<String>>,
    /// Optional 'blacklist' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blacklist: Option<Vec<String>>,
    /// Optional 'options' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<ProviderConfigOptions>,
}
///Generated from schema 'McpLocalConfig'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpLocalConfig {
    /// Type of MCP server connection
    #[serde(rename = "type")]
    pub type_field: String,
    /// Command and arguments to run the MCP server
    pub command: Vec<String>,
    /// Environment variables to set when running the MCP server
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<serde_json::Value>,
    /// Enable or disable the MCP server on startup
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Timeout in ms for fetching tools from the MCP server. Defaults to 5000 (5 seconds) if not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}
///Generated from schema 'McpOAuthConfig'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpOAuthConfig {
    /// OAuth client ID. If not provided, dynamic client registration (RFC 7591) will be attempted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientId")]
    pub client_id: Option<String>,
    /// OAuth client secret (if required by the authorization server)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientSecret")]
    pub client_secret: Option<String>,
    /// OAuth scopes to request during authorization
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
///Generated from schema 'McpRemoteConfig'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRemoteConfig {
    /// Type of MCP server connection
    #[serde(rename = "type")]
    pub type_field: String,
    /// URL of the remote MCP server
    pub url: String,
    /// Enable or disable the MCP server on startup
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Headers to send with the request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    /// OAuth authentication configuration for the MCP server. Set to false to disable OAuth auto-detection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth: Option<serde_json::Value>,
    /// Timeout in ms for fetching tools from the MCP server. Defaults to 5000 (5 seconds) if not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}
///Generated from schema 'Config'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// JSON schema reference for configuration validation
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "$schema")]
    pub dollar_schema: Option<String>,
    /// Theme name to use for the interface
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    /// Optional 'keybinds' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keybinds: Option<KeybindsConfig>,
    /// Optional 'logLevel' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logLevel")]
    pub log_level: Option<LogLevel>,
    /// TUI specific settings
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tui: Option<ConfigTui>,
    /// Optional 'server' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<ServerConfig>,
    /// Command configuration, see https://opencode.ai/docs/commands
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<serde_json::Value>,
    /// Optional 'watcher' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watcher: Option<ConfigWatcher>,
    /// Optional 'plugin' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin: Option<Vec<String>>,
    /// Optional 'snapshot' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
    /// Control sharing behavior:'manual' allows manual sharing via commands, 'auto' enables automatic sharing, 'disabled' disables all sharing
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share: Option<StringEnum>,
    /// @deprecated Use 'share' field instead. Share newly created sessions automatically
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autoshare: Option<bool>,
    /// Automatically update to the latest version. Set to true to auto-update, false to disable, or 'notify' to show update notifications
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autoupdate: Option<serde_json::Value>,
    /// Disable providers that are loaded automatically
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_providers: Option<Vec<String>>,
    /// When set, ONLY these providers will be enabled. All other providers will be ignored
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled_providers: Option<Vec<String>>,
    /// Model to use in the format of provider/model, eg anthropic/claude-2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Small model to use for tasks like title generation in the format of provider/model
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub small_model: Option<String>,
    /// Default agent to use when none is specified. Must be a primary agent. Falls back to 'build' if not set or if the specified agent is invalid.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_agent: Option<String>,
    /// Custom username to display in conversations instead of system username
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// @deprecated Use `agent` field instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ConfigMode>,
    /// Agent configuration, see https://opencode.ai/docs/agent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<ConfigAgent>,
    /// Custom provider configurations and model overrides
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<serde_json::Value>,
    /// MCP (Model Context Protocol) server configurations
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcp: Option<serde_json::Value>,
    /// Optional 'formatter' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatter: Option<serde_json::Value>,
    /// Optional 'lsp' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lsp: Option<serde_json::Value>,
    /// Additional instruction files or patterns to include
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<Vec<String>>,
    /// Optional 'layout' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<LayoutConfig>,
    /// Optional 'permission' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<PermissionConfig>,
    /// Optional 'tools' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
    /// Optional 'enterprise' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<ConfigEnterprise>,
    /// Optional 'compaction' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compaction: Option<ConfigCompaction>,
    /// Optional 'experimental' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experimental: Option<ConfigExperimental>,
}
///Generated from schema 'ToolIDs'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolIDs(pub Vec<String>);
///Generated from schema 'ToolListItem'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolListItem {
    /// Required 'id' field.
    pub id: String,
    /// Required 'description' field.
    pub description: String,
    /// Required 'parameters' field.
    pub parameters: serde_json::Value,
}
///Generated from schema 'ToolList'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolList(pub Vec<ToolListItem>);
///Generated from schema 'Path'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    /// Required 'home' field.
    pub home: String,
    /// Required 'state' field.
    pub state: String,
    /// Required 'config' field.
    pub config: String,
    /// Required 'worktree' field.
    pub worktree: String,
    /// Required 'directory' field.
    pub directory: String,
}
///Generated from schema 'Worktree'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worktree {
    /// Required 'name' field.
    pub name: String,
    /// Required 'branch' field.
    pub branch: String,
    /// Required 'directory' field.
    pub directory: String,
}
///Generated from schema 'WorktreeCreateInput'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorktreeCreateInput {
    /// Optional 'name' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional 'startCommand' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startCommand")]
    pub start_command: Option<String>,
}
///Generated from schema 'VcsInfo'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VcsInfo {
    /// Required 'branch' field.
    pub branch: String,
}
///Generated from schema 'TextPartInput'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPartInput {
    /// Optional 'id' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'text' field.
    pub text: String,
    /// Optional 'synthetic' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synthetic: Option<bool>,
    /// Optional 'ignored' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignored: Option<bool>,
    /// Optional 'time' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<TextPartInputTime>,
    /// Optional 'metadata' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
///Generated from schema 'FilePartInput'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePartInput {
    /// Optional 'id' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'mime' field.
    pub mime: String,
    /// Optional 'filename' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Required 'url' field.
    pub url: String,
    /// Optional 'source' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<FilePartSource>,
}
///Generated from schema 'AgentPartInput'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPartInput {
    /// Optional 'id' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'name' field.
    pub name: String,
    /// Optional 'source' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<AgentPartInputSource>,
}
///Generated from schema 'SubtaskPartInput'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtaskPartInput {
    /// Optional 'id' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'prompt' field.
    pub prompt: String,
    /// Required 'description' field.
    pub description: String,
    /// Required 'agent' field.
    pub agent: String,
    /// Optional 'command' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
}
///Generated from schema 'Command'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    /// Required 'name' field.
    pub name: String,
    /// Optional 'description' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional 'agent' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Optional 'model' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Optional 'mcp' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcp: Option<bool>,
    /// Required 'template' field.
    pub template: serde_json::Value,
    /// Optional 'subtask' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtask: Option<bool>,
    /// Required 'hints' field.
    pub hints: Vec<String>,
}
///Generated from schema 'Model'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    /// Required 'id' field.
    pub id: String,
    /// Required 'providerID' field.
    #[serde(rename = "providerID")]
    pub provider_id: String,
    /// Required 'api' field.
    pub api: ModelApi,
    /// Required 'name' field.
    pub name: String,
    /// Optional 'family' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// Required 'capabilities' field.
    pub capabilities: ModelCapabilities,
    /// Required 'cost' field.
    pub cost: ModelCost,
    /// Required 'limit' field.
    pub limit: ModelLimit,
    /// Required 'status' field.
    pub status: StringEnum,
    /// Required 'options' field.
    pub options: serde_json::Value,
    /// Required 'headers' field.
    pub headers: serde_json::Value,
    /// Required 'release_date' field.
    pub release_date: String,
    /// Optional 'variants' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variants: Option<serde_json::Value>,
}
///Generated from schema 'Provider'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    /// Required 'id' field.
    pub id: String,
    /// Required 'name' field.
    pub name: String,
    /// Required 'source' field.
    pub source: StringEnum,
    /// Required 'env' field.
    pub env: Vec<String>,
    /// Optional 'key' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Required 'options' field.
    pub options: serde_json::Value,
    /// Required 'models' field.
    pub models: serde_json::Value,
}
///Generated from schema 'ProviderAuthMethod'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAuthMethod {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: serde_json::Value,
    /// Required 'label' field.
    pub label: String,
}
///Generated from schema 'ProviderAuthAuthorization'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAuthAuthorization {
    /// Required 'url' field.
    pub url: String,
    /// Required 'method' field.
    pub method: serde_json::Value,
    /// Required 'instructions' field.
    pub instructions: String,
}
///Generated from schema 'Symbol'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    /// Required 'name' field.
    pub name: String,
    /// Required 'kind' field.
    pub kind: f64,
    /// Required 'location' field.
    pub location: SymbolLocation,
}
///Generated from schema 'FileNode'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileNode {
    /// Required 'name' field.
    pub name: String,
    /// Required 'path' field.
    pub path: String,
    /// Required 'absolute' field.
    pub absolute: String,
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: StringEnum,
    /// Required 'ignored' field.
    pub ignored: bool,
}
///Generated from schema 'FileContent'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContent {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'content' field.
    pub content: String,
    /// Optional 'diff' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff: Option<String>,
    /// Optional 'patch' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<FileContentPatch>,
    /// Optional 'encoding' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// Optional 'mimeType' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mimeType")]
    pub mime_type: Option<String>,
}
///Generated from schema 'File'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    /// Required 'path' field.
    pub path: String,
    /// Required 'added' field.
    pub added: i64,
    /// Required 'removed' field.
    pub removed: i64,
    /// Required 'status' field.
    pub status: StringEnum,
}
///Generated from schema 'Agent'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Required 'name' field.
    pub name: String,
    /// Optional 'description' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Required 'mode' field.
    pub mode: StringEnum,
    /// Optional 'native' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native: Option<bool>,
    /// Optional 'hidden' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// Optional 'topP' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topP")]
    pub top_p: Option<f64>,
    /// Optional 'temperature' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Optional 'color' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Required 'permission' field.
    pub permission: PermissionRuleset,
    /// Optional 'model' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<AgentModel>,
    /// Optional 'prompt' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Required 'options' field.
    pub options: serde_json::Value,
    /// Optional 'steps' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<i64>,
}
///Generated from schema 'MCPStatusConnected'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatusConnected {
    /// Required 'status' field.
    pub status: String,
}
///Generated from schema 'MCPStatusDisabled'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatusDisabled {
    /// Required 'status' field.
    pub status: String,
}
///Generated from schema 'MCPStatusFailed'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatusFailed {
    /// Required 'status' field.
    pub status: String,
    /// Required 'error' field.
    pub error: String,
}
///Generated from schema 'MCPStatusNeedsAuth'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatusNeedsAuth {
    /// Required 'status' field.
    pub status: String,
}
///Generated from schema 'MCPStatusNeedsClientRegistration'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPStatusNeedsClientRegistration {
    /// Required 'status' field.
    pub status: String,
    /// Required 'error' field.
    pub error: String,
}
///Generated from schema 'McpResource'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResource {
    /// Required 'name' field.
    pub name: String,
    /// Required 'uri' field.
    pub uri: String,
    /// Optional 'description' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional 'mimeType' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mimeType")]
    pub mime_type: Option<String>,
    /// Required 'client' field.
    pub client: String,
}
///Generated from schema 'LSPStatus'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LSPStatus {
    /// Required 'id' field.
    pub id: String,
    /// Required 'name' field.
    pub name: String,
    /// Required 'root' field.
    pub root: String,
    /// Required 'status' field.
    pub status: serde_json::Value,
}
///Generated from schema 'FormatterStatus'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatterStatus {
    /// Required 'name' field.
    pub name: String,
    /// Required 'extensions' field.
    pub extensions: Vec<String>,
    /// Required 'enabled' field.
    pub enabled: bool,
}
///Generated from schema 'OAuth'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'refresh' field.
    pub refresh: String,
    /// Required 'access' field.
    pub access: String,
    /// Required 'expires' field.
    pub expires: f64,
    /// Optional 'accountId' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accountId")]
    pub account_id: Option<String>,
    /// Optional 'enterpriseUrl' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enterpriseUrl")]
    pub enterprise_url: Option<String>,
}
///Generated from schema 'ApiAuth'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuth {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'key' field.
    pub key: String,
}
///Generated from schema 'WellKnownAuth'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellKnownAuth {
    /// Required 'type' field.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Required 'key' field.
    pub key: String,
    /// Required 'token' field.
    pub token: String,
}
///Request response type for endpoint 'global_health_get'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalHealthGetResponse {
    /// Required 'healthy' field.
    pub healthy: bool,
    /// Required 'version' field.
    pub version: String,
}
///Request body type for endpoint 'project_projectid_patch'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectProjectidPatchRequest {
    /// Optional 'name' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional 'icon' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<InlineObject>,
}
///Request body type for endpoint 'pty_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtyPostRequest {
    /// Optional 'command' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// Optional 'args' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// Optional 'cwd' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    /// Optional 'title' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional 'env' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<serde_json::Value>,
}
///Request body type for endpoint 'pty_ptyid_put'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtyPtyidPutRequest {
    /// Optional 'title' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional 'size' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<InlineObject>,
}
///Request body type for endpoint 'session_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionPostRequest {
    /// Optional 'parentID' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parentID")]
    pub parent_id: Option<String>,
    /// Optional 'title' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional 'permission' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<PermissionRuleset>,
}
///Request body type for endpoint 'session_sessionid_patch'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidPatchRequest {
    /// Optional 'title' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional 'time' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<InlineObject>,
}
///Request body type for endpoint 'session_sessionid_init_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidInitPostRequest {
    /// Required 'modelID' field.
    #[serde(rename = "modelID")]
    pub model_id: String,
    /// Required 'providerID' field.
    #[serde(rename = "providerID")]
    pub provider_id: String,
    /// Required 'messageID' field.
    #[serde(rename = "messageID")]
    pub message_id: String,
}
///Request body type for endpoint 'session_sessionid_fork_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidForkPostRequest {
    /// Optional 'messageID' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
}
///Request body type for endpoint 'session_sessionid_summarize_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidSummarizePostRequest {
    /// Required 'providerID' field.
    #[serde(rename = "providerID")]
    pub provider_id: String,
    /// Required 'modelID' field.
    #[serde(rename = "modelID")]
    pub model_id: String,
    /// Optional 'auto' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto: Option<bool>,
}
///Request response type for endpoint 'session_sessionid_message_get'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidMessageGetResponse {
    /// Required 'info' field.
    pub info: Message,
    /// Required 'parts' field.
    pub parts: Vec<Part>,
}
///Request body type for endpoint 'session_sessionid_message_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidMessagePostRequest {
    /// Optional 'messageID' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    /// Optional 'model' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<InlineObject>,
    /// Optional 'agent' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Optional 'noReply' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "noReply")]
    pub no_reply: Option<bool>,
    /// @deprecated tools and permissions have been merged, you can set permissions on the session itself now
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
    /// Optional 'system' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Optional 'variant' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    /// Required 'parts' field.
    pub parts: Vec<Type>,
}
///Request response type for endpoint 'session_sessionid_message_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidMessagePostResponse {
    /// Required 'info' field.
    pub info: AssistantMessage,
    /// Required 'parts' field.
    pub parts: Vec<Part>,
}
///Request response type for endpoint 'session_sessionid_message_messageid_get'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidMessageMessageidGetResponse {
    /// Required 'info' field.
    pub info: Message,
    /// Required 'parts' field.
    pub parts: Vec<Part>,
}
///Request body type for endpoint 'session_sessionid_prompt_async_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidPromptAsyncPostRequest {
    /// Optional 'messageID' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    /// Optional 'model' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<InlineObject>,
    /// Optional 'agent' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Optional 'noReply' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "noReply")]
    pub no_reply: Option<bool>,
    /// @deprecated tools and permissions have been merged, you can set permissions on the session itself now
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<serde_json::Value>,
    /// Optional 'system' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// Optional 'variant' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    /// Required 'parts' field.
    pub parts: Vec<Type>,
}
///Request body type for endpoint 'session_sessionid_command_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidCommandPostRequest {
    /// Optional 'messageID' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "messageID")]
    pub message_id: Option<String>,
    /// Optional 'agent' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    /// Optional 'model' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Required 'arguments' field.
    pub arguments: String,
    /// Required 'command' field.
    pub command: String,
    /// Optional 'variant' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    /// Optional 'parts' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Type>>,
}
///Request response type for endpoint 'session_sessionid_command_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidCommandPostResponse {
    /// Required 'info' field.
    pub info: AssistantMessage,
    /// Required 'parts' field.
    pub parts: Vec<Part>,
}
///Request body type for endpoint 'session_sessionid_shell_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidShellPostRequest {
    /// Required 'agent' field.
    pub agent: String,
    /// Optional 'model' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<InlineObject>,
    /// Required 'command' field.
    pub command: String,
}
///Request body type for endpoint 'session_sessionid_revert_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidRevertPostRequest {
    /// Required 'messageID' field.
    #[serde(rename = "messageID")]
    pub message_id: String,
    /// Optional 'partID' field.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "partID")]
    pub part_id: Option<String>,
}
///Request body type for endpoint 'session_sessionid_permissions_permissionid_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSessionidPermissionsPermissionidPostRequest {
    /// Required 'response' field.
    pub response: StringEnum,
}
///Request body type for endpoint 'permission_requestid_reply_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRequestidReplyPostRequest {
    /// Required 'reply' field.
    pub reply: StringEnum,
    /// Optional 'message' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
///Request body type for endpoint 'question_requestid_reply_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionRequestidReplyPostRequest {
    /// User answers in order of questions (each answer is an array of selected labels)
    pub answers: Vec<QuestionAnswer>,
}
///Request response type for endpoint 'config_providers_get'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigProvidersGetResponse {
    /// Required 'providers' field.
    pub providers: Vec<Provider>,
    /// Required 'default' field.
    pub default: serde_json::Value,
}
///Request response type for endpoint 'provider_get'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderGetResponse {
    /// Required 'all' field.
    pub all: Vec<InlineObject>,
    /// Required 'default' field.
    pub default: serde_json::Value,
    /// Required 'connected' field.
    pub connected: Vec<String>,
}
///Request body type for endpoint 'provider_providerid_oauth_authorize_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderProvideridOauthAuthorizePostRequest {
    /// Auth method index
    pub method: f64,
}
///Request body type for endpoint 'provider_providerid_oauth_callback_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderProvideridOauthCallbackPostRequest {
    /// Auth method index
    pub method: f64,
    /// OAuth authorization code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
///Request response type for endpoint 'find_get'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindGetResponse {
    /// Required 'path' field.
    pub path: FindGetResponsePath,
    /// Required 'lines' field.
    pub lines: FindGetResponseLines,
    /// Required 'line_number' field.
    pub line_number: f64,
    /// Required 'absolute_offset' field.
    pub absolute_offset: f64,
    /// Required 'submatches' field.
    pub submatches: Vec<InlineObject>,
}
///Request body type for endpoint 'log_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogPostRequest {
    /// Service name for the log entry
    pub service: String,
    /// Log level
    pub level: StringEnum,
    /// Log message
    pub message: String,
    /// Additional metadata for the log entry
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>,
}
///Request body type for endpoint 'mcp_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPostRequest {
    /// Required 'name' field.
    pub name: String,
    /// Required 'config' field.
    pub config: Type,
}
///Request response type for endpoint 'mcp_name_auth_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpNameAuthPostResponse {
    /// URL to open in browser for authorization
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: String,
}
///Request response type for endpoint 'mcp_name_auth_delete'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpNameAuthDeleteResponse {
    /// Required 'success' field.
    pub success: bool,
}
///Request body type for endpoint 'mcp_name_auth_callback_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpNameAuthCallbackPostRequest {
    /// Authorization code from OAuth callback
    pub code: String,
}
///Request body type for endpoint 'tui_append_prompt_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiAppendPromptPostRequest {
    /// Required 'text' field.
    pub text: String,
}
///Request body type for endpoint 'tui_execute_command_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiExecuteCommandPostRequest {
    /// Required 'command' field.
    pub command: String,
}
///Request body type for endpoint 'tui_show_toast_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiShowToastPostRequest {
    /// Optional 'title' field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Required 'message' field.
    pub message: String,
    /// Required 'variant' field.
    pub variant: StringEnum,
    /// Duration in milliseconds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
}
///Request body type for endpoint 'tui_select_session_post'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiSelectSessionPostRequest {
    /// Session ID to navigate to
    #[serde(rename = "sessionID")]
    pub session_id: String,
}
///Request response type for endpoint 'tui_control_next_get'.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuiControlNextGetResponse {
    /// Required 'path' field.
    pub path: String,
    /// Required 'body' field.
    pub body: serde_json::Value,
}
///Generated from schema 'StringEnum'.
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(rename_all = "snake_case")]
pub enum StringEnum {
    Info,
    Success,
    Warning,
    Error,
}
///Generated from schema 'Name'.
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "name")]
pub enum Name {
    #[display("ProviderAuthError")]
    ProviderAuthError {
        /// Required 'data' field.
        data: ProviderAuthErrorData1,
    },
    #[display("UnknownError")]
    UnknownError {
        /// Required 'data' field.
        data: UnknownErrorData1,
    },
    #[display("MessageOutputLengthError")]
    MessageOutputLengthError {
        /// Required 'data' field.
        data: serde_json::Value,
    },
    #[display("MessageAbortedError")]
    MessageAbortedError {
        /// Required 'data' field.
        data: MessageAbortedErrorData1,
    },
    #[display("APIError")]
    APIError {
        /// Required 'data' field.
        data: APIErrorData1,
    },
}
///Generated from schema 'Type'.
#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(tag = "type")]
pub enum Type {
    #[serde(rename = "text")]
    #[display("Text")]
    Text {
        /// Optional 'id' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        /// Required 'text' field.
        text: String,
        /// Optional 'synthetic' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        synthetic: Option<bool>,
        /// Optional 'ignored' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ignored: Option<bool>,
        /// Optional 'time' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        time: Option<TextPartInputTime1>,
        /// Optional 'metadata' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<serde_json::Value>,
    },
    #[serde(rename = "file")]
    #[display("File")]
    File {
        /// Optional 'id' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        /// Required 'mime' field.
        mime: String,
        /// Optional 'filename' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
        /// Required 'url' field.
        url: String,
        /// Optional 'source' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source: Option<FilePartSource>,
    },
    #[serde(rename = "agent")]
    #[display("Agent")]
    Agent {
        /// Optional 'id' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        /// Required 'name' field.
        name: String,
        /// Optional 'source' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source: Option<AgentPartInputSource1>,
    },
    #[serde(rename = "subtask")]
    #[display("Subtask")]
    Subtask {
        /// Optional 'id' field.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        /// Required 'prompt' field.
        prompt: String,
        /// Required 'description' field.
        description: String,
        /// Required 'agent' field.
        agent: String,
        /// Optional 'command' field.
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
