//! Thin MCP wallet session -> ACTA connector (implementation candidate, Proposed ADR-021).
//!
//! Tether's WDK ships an MCP server, `wdk-mcp`, that lets an AI agent operate a self-custodial
//! wallet. Its own documentation is plain about what it does not do: the preview-then-confirm
//! sequence for transfers is guidance for the client and is not enforced, there are no
//! spending limits, and no record of tool calls is kept.
//!
//! This connector supplies the record and nothing else. A proxy keeps each tool call exactly as
//! it crossed the wire; this crate turns that session into `ai_agent` Profile v1.1 events.
//!
//! What it never does:
//!
//! - **It does not approve, block or limit.** A recorder that also decides is a policy engine,
//!   and its record would be evidence about itself.
//! - **It does not interpret results.** Requests and answers are committed as transmitted. An
//!   answer is read only for the two facts the mapping needs: did the server answer with an
//!   error, and does that error carry a code the deployer declared as a refusal.
//! - **It does not claim a human approved anything.** Approval dialogs live in the AI client
//!   and never cross the MCP wire, so no `human_authorization_recorded` is ever emitted.
//!
//! Core never learns that MCP or WDK exist.

use acta_ai_agent_profile::types::{
    AiAgentDomainEventV0, ContextCommittedPayloadV0, InstructionReceivedPayloadV0,
    RunStartedPayloadV0, SignificantActionExecutedPayloadV0, ToolCallExecutedPayloadV0,
};
use acta_ai_agent_profile_v1_1::lifecycle::validate_lifecycle_v1_1;
use acta_ai_agent_profile_v1_1::{
    map_v1_1_event_to_core_v0, ActionDeniedPayloadV1, AiAgentDomainEventV1, ControlNatureV1,
    CoverageSnapshotV1, SessionClosedPayloadV1, SessionOpenedPayloadV1,
};
use acta_core::types::{ActaEventV0, PolicySnapshotV0};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

pub const CONNECTOR_VERSION: &str = "acta.wdk-mcp.v0";
pub const SESSION_SCHEMA_V0: &str = "acta.mcp.session.v0";
/// The kinds this connector commits to emitting while a session is open (ADR-015 §3). It is the
/// connector's own declaration, not data read from the server.
pub const DECLARED_KINDS: [&str; 6] = [
    "run_started",
    "instruction_received",
    "context_committed",
    "tool_call_executed",
    "significant_action_executed",
    "action_denied",
];
pub const DECISION_REF_NONE: &str = "none-recorded";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreviewArgumentV0 {
    /// The boolean argument that makes the tool preview instead of execute.
    pub argument: String,
    /// What the server assumes when the argument is absent.
    pub default: bool,
}

/// The deployer's own statement about the server's tools. Nothing here is checked against the
/// server: it is who ran the proxy saying what they consider significant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclaredV0 {
    pub significant_tools: Vec<String>,
    pub preview_arguments: BTreeMap<String, PreviewArgumentV0>,
    pub denial_codes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExchangeV0 {
    pub request: String,
    pub response: Option<String>,
    pub requested_at: String,
    pub responded_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolCallV0 {
    /// The JSON-RPC id as it appeared, JSON-encoded (`7` or `"a"`).
    pub jsonrpc_id: String,
    pub tool: String,
    /// The request line exactly as the client sent it.
    pub request: String,
    /// The response line exactly as the server sent it. `None` if it never answered.
    pub response: Option<String>,
    pub requested_at: String,
    pub responded_at: Option<String>,
}

/// One MCP session as the proxy recorded it. Produced by `js/src/recorder.mjs`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct McpSessionRecordV0 {
    pub schema: String,
    pub session_id: String,
    pub agent_ref: String,
    pub deployed_by_ref: String,
    /// What the deployer told the agent it may do with this wallet, as text. The proxy cannot
    /// see the prompts of the AI client; this is the only instruction it can honestly hold.
    pub standing_instruction: String,
    pub declared: DeclaredV0,
    pub opened_at: String,
    pub closed_at: Option<String>,
    pub initialize: Option<ExchangeV0>,
    pub tool_catalog: Option<ExchangeV0>,
    pub calls: Vec<ToolCallV0>,
    pub unparsed_lines: u64,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ConnectorError {
    #[error("unsupported record schema: {0}")]
    UnsupportedSchema(String),
    #[error("empty field: {0}")]
    Empty(&'static str),
    #[error("the session was never closed")]
    NotClosed,
    #[error("the server never answered `initialize`, so there is no context to commit")]
    NoContext,
    #[error("profile rejected the mapping: {0}")]
    Profile(String),
}

/// A recorded call that no Profile event describes. Committed, so the omission is on the record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnrepresentedV0 {
    pub jsonrpc_id: String,
    pub tool: String,
    pub code: String,
    pub reason: String,
    pub record_commitment: String,
}

/// Something the session shows that a reader should not have to discover alone. A finding
/// states a fact about the record. It is not a verdict about the agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "finding", rename_all = "snake_case")]
pub enum FindingV0 {
    /// A tool that can preview was executed with arguments no earlier call in this session had
    /// previewed.
    ExecutedWithoutPreview { jsonrpc_id: String, tool: String },
    /// The deployer declared no refusal codes, or the server produced none: this session
    /// contains no evidence that any control evaluated anything.
    NoControlObserved,
    /// The proxy forwarded lines it could not parse. They are not in the record.
    UnparsedLines { count: u64 },
}

#[derive(Debug, Clone)]
pub struct MappedSessionV0 {
    pub run_id: String,
    pub policy: PolicySnapshotV0,
    pub events: Vec<AiAgentDomainEventV1>,
    pub core_events: Vec<ActaEventV0>,
    pub unrepresented: Vec<UnrepresentedV0>,
    pub findings: Vec<FindingV0>,
}

pub fn commit(bytes: &[u8]) -> String {
    format!("sha256:{}", sha256_hex(bytes))
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// The two facts read out of an answer. Everything else in it is committed, not read.
enum Answer {
    /// A JSON-RPC error, or a line that is not a JSON-RPC result: no tool ran.
    NotExecuted,
    Ok,
    ToolError {
        code: Option<String>,
    },
}

fn read_answer(response: &str) -> Answer {
    let Ok(Value::Object(message)) = serde_json::from_str::<Value>(response) else {
        return Answer::NotExecuted;
    };
    let Some(Value::Object(result)) = message.get("result") else {
        return Answer::NotExecuted;
    };
    if result.get("isError") != Some(&Value::Bool(true)) {
        return Answer::Ok;
    }
    // WDK puts a JSON object with `code` in the first text block. Anything else has no code.
    let code = result
        .get("content")
        .and_then(|content| content.get(0))
        .and_then(|block| block.get("text"))
        .and_then(Value::as_str)
        .and_then(|text| serde_json::from_str::<Value>(text).ok())
        .and_then(|body| body.get("code").and_then(Value::as_str).map(str::to_string));
    Answer::ToolError { code }
}

/// The call's arguments with the preview switch removed, plus whether it was a preview.
/// `None` when the request cannot be read or the tool has no declared preview switch.
fn preview_shape(call: &ToolCallV0, declared: &DeclaredV0) -> Option<(bool, Value)> {
    let preview = declared.preview_arguments.get(&call.tool)?;
    let request: Value = serde_json::from_str(&call.request).ok()?;
    let mut arguments = request
        .get("params")?
        .get("arguments")?
        .as_object()?
        .clone();
    let is_preview = match arguments.remove(&preview.argument) {
        None => preview.default,
        Some(Value::Bool(value)) => value,
        // Not a boolean: the connector does not guess what the server made of it.
        Some(_) => return None,
    };
    Some((is_preview, Value::Object(arguments)))
}

fn non_empty(value: &str, field: &'static str) -> Result<(), ConnectorError> {
    if value.trim().is_empty() {
        Err(ConnectorError::Empty(field))
    } else {
        Ok(())
    }
}

/// Map one recorded session onto `ai_agent` Profile v1.1 and Core.
///
/// - The **instruction** is the deployer's standing instruction, because that is the only one
///   the proxy holds. Its hash is the policy hash of every event.
/// - The **context** is what the server offered the agent: its identity and its tool catalogue,
///   as transmitted.
/// - Every answered call becomes `tool_call_executed`, whatever the answer says: the call was
///   made and this is what came back.
/// - A declared-significant tool that executed (not a preview, not an error) additionally
///   becomes `significant_action_executed`. Its `decision_ref` points at the preview that
///   preceded it with the same arguments, or says none was recorded.
/// - An error carrying a declared refusal code becomes `action_denied`. Any other error is a
///   failure, which is plumbing, and stays a `tool_call_executed` whose output is the error.
pub fn map_session_v1_1(record: &McpSessionRecordV0) -> Result<MappedSessionV0, ConnectorError> {
    if record.schema != SESSION_SCHEMA_V0 {
        return Err(ConnectorError::UnsupportedSchema(record.schema.clone()));
    }
    non_empty(&record.session_id, "session_id")?;
    non_empty(&record.agent_ref, "agent_ref")?;
    non_empty(&record.deployed_by_ref, "deployed_by_ref")?;
    non_empty(&record.standing_instruction, "standing_instruction")?;
    non_empty(&record.opened_at, "opened_at")?;
    let closed_at = record.closed_at.clone().ok_or(ConnectorError::NotClosed)?;
    let initialize = record
        .initialize
        .as_ref()
        .ok_or(ConnectorError::NoContext)?;
    let (Some(initialize_response), Some(initialized_at)) =
        (&initialize.response, &initialize.responded_at)
    else {
        return Err(ConnectorError::NoContext);
    };

    let session_id = record.session_id.clone();
    let run_id = format!("{session_id}-run");
    let policy_hash = sha256_hex(record.standing_instruction.as_bytes());
    let policy = PolicySnapshotV0 {
        policy_id: format!("{CONNECTOR_VERSION}:standing-instruction:{session_id}"),
        policy_hash: policy_hash.clone(),
        policy_type: "internal".to_string(),
        jurisdiction: "n/a".to_string(),
        effective_from: record.opened_at.clone(),
        effective_to: None,
    };

    let mut events = vec![
        AiAgentDomainEventV1::SessionOpened(SessionOpenedPayloadV1 {
            session_id: session_id.clone(),
            agent_identity_ref: record.agent_ref.clone(),
            // The client's own `initialize` line is the only statement of what the agent is.
            agent_identity_commitment: commit(initialize.request.as_bytes()),
            coverage: CoverageSnapshotV1 {
                coverage_id: format!("{CONNECTOR_VERSION}:{session_id}"),
                coverage_hash: sha256_hex(DECLARED_KINDS.join(",").as_bytes()),
                declared_kinds: DECLARED_KINDS.iter().map(|k| k.to_string()).collect(),
                effective_from: record.opened_at.clone(),
                effective_to: Some(closed_at.clone()),
            },
            opened_at: record.opened_at.clone(),
        }),
        AiAgentDomainEventV1::Inherited(AiAgentDomainEventV0::RunStarted(RunStartedPayloadV0 {
            run_id: run_id.clone(),
            agent_ref: record.agent_ref.clone(),
            deployed_by_ref: record.deployed_by_ref.clone(),
            started_at: record.opened_at.clone(),
            environment_ref: "mcp:stdio".to_string(),
            agent_version_commitment: commit(initialize.request.as_bytes()),
        })),
        AiAgentDomainEventV1::Inherited(AiAgentDomainEventV0::InstructionReceived(
            InstructionReceivedPayloadV0 {
                run_id: run_id.clone(),
                instruction_ref: format!("{session_id}:standing-instruction"),
                instruction_commitment: commit(record.standing_instruction.as_bytes()),
                requester_ref: record.deployed_by_ref.clone(),
                received_at: record.opened_at.clone(),
                policy_hash: policy_hash.clone(),
            },
        )),
    ];

    // Context: what the server offered. The catalogue when the client asked for it, and the
    // server's identity either way.
    let catalogue = record
        .tool_catalog
        .as_ref()
        .and_then(|c| Some((c.response.as_ref()?, c.responded_at.as_ref()?)));
    let (context_bytes, evidence_refs, committed_at) = match catalogue {
        Some((response, at)) => (
            format!("{initialize_response}\n{response}"),
            vec!["mcp:initialize".to_string(), "mcp:tools/list".to_string()],
            at.clone(),
        ),
        None => (
            initialize_response.clone(),
            vec!["mcp:initialize".to_string()],
            initialized_at.clone(),
        ),
    };
    events.push(AiAgentDomainEventV1::Inherited(
        AiAgentDomainEventV0::ContextCommitted(ContextCommittedPayloadV0 {
            run_id: run_id.clone(),
            context_commitment: commit(context_bytes.as_bytes()),
            evidence_refs,
            retrieval_snapshot_commitment: None,
            committed_at,
        }),
    ));

    let mut unrepresented = Vec::new();
    let mut findings = Vec::new();
    let mut previews: Vec<(String, Value, String)> = Vec::new(); // (tool, arguments, call ref)
    let mut control_observed = false;

    for call in &record.calls {
        let call_ref = format!("mcp:{}:{}", call.tool, call.jsonrpc_id);
        let call_commitment = || commit(serde_json::to_string(call).unwrap_or_default().as_bytes());
        let mut skip = |code: &str, reason: &str| {
            unrepresented.push(UnrepresentedV0 {
                jsonrpc_id: call.jsonrpc_id.clone(),
                tool: call.tool.clone(),
                code: code.to_string(),
                reason: reason.to_string(),
                record_commitment: call_commitment(),
            })
        };
        if call.tool.trim().is_empty() {
            skip("no_tool_name", "the request named no tool");
            continue;
        }
        let (Some(response), Some(responded_at)) = (&call.response, &call.responded_at) else {
            skip(
                "unanswered",
                "the server never answered; nothing is known to have executed",
            );
            continue;
        };

        let answer = read_answer(response);
        match &answer {
            Answer::NotExecuted => {
                skip(
                    "not_executed",
                    "the answer is a protocol error, not a tool result; no tool ran",
                );
                continue;
            }
            Answer::ToolError { code: Some(code) }
                if record.declared.denial_codes.contains(code) =>
            {
                control_observed = true;
                events.push(AiAgentDomainEventV1::ActionDenied(ActionDeniedPayloadV1 {
                    run_id: run_id.clone(),
                    session_id: session_id.clone(),
                    attempted_ref: call_ref,
                    attempted_input_commitment: commit(call.request.as_bytes()),
                    control_ref: format!("ref:mcp-server:control:{code}"),
                    control_nature: ControlNatureV1::Policy,
                    // The server's answer, as sent, is the only statement of reasons there is.
                    reasoning_commitment: commit(response.as_bytes()),
                    denied_at: responded_at.clone(),
                }));
                continue;
            }
            Answer::Ok | Answer::ToolError { .. } => {}
        }

        events.push(AiAgentDomainEventV1::Inherited(
            AiAgentDomainEventV0::ToolCallExecuted(ToolCallExecutedPayloadV0 {
                run_id: run_id.clone(),
                tool_ref: call.tool.clone(),
                tool_input_commitment: commit(call.request.as_bytes()),
                tool_output_commitment: commit(response.as_bytes()),
                tool_call_id: call_ref.clone(),
                executed_at: responded_at.clone(),
            }),
        ));
        if !matches!(answer, Answer::Ok) {
            continue;
        }

        let shape = preview_shape(call, &record.declared);
        if let Some((true, arguments)) = &shape {
            previews.push((call.tool.clone(), arguments.clone(), call_ref));
            continue;
        }
        if !record.declared.significant_tools.contains(&call.tool) {
            continue;
        }
        let decision_ref = match &shape {
            // A tool with a preview switch, executed.
            Some((false, arguments)) => {
                let previewed = previews
                    .iter()
                    .rev()
                    .find(|(tool, previewed, _)| tool == &call.tool && previewed == arguments)
                    .map(|(_, _, preview_ref)| preview_ref.clone());
                if previewed.is_none() {
                    findings.push(FindingV0::ExecutedWithoutPreview {
                        jsonrpc_id: call.jsonrpc_id.clone(),
                        tool: call.tool.clone(),
                    });
                }
                previewed.unwrap_or_else(|| DECISION_REF_NONE.to_string())
            }
            // No preview switch declared for this tool, or its arguments could not be read.
            _ => DECISION_REF_NONE.to_string(),
        };
        events.push(AiAgentDomainEventV1::Inherited(
            AiAgentDomainEventV0::SignificantActionExecuted(SignificantActionExecutedPayloadV0 {
                run_id: run_id.clone(),
                action_ref: format!("{call_ref}:action"),
                action_type: call.tool.clone(),
                action_input_commitment: commit(call.request.as_bytes()),
                action_output_commitment: commit(response.as_bytes()),
                decision_ref,
                executed_at: responded_at.clone(),
                policy_hash: policy_hash.clone(),
            }),
        ));
    }

    events.push(AiAgentDomainEventV1::SessionClosed(
        SessionClosedPayloadV1 {
            session_id: session_id.clone(),
            closed_at,
        },
    ));

    if !control_observed {
        findings.push(FindingV0::NoControlObserved);
    }
    if record.unparsed_lines > 0 {
        findings.push(FindingV0::UnparsedLines {
            count: record.unparsed_lines,
        });
    }

    validate_lifecycle_v1_1(&events).map_err(|e| ConnectorError::Profile(e.to_string()))?;
    let core_events = events
        .iter()
        .enumerate()
        .map(|(index, event)| {
            map_v1_1_event_to_core_v0(event, format!("{session_id}:{index:04}"), &run_id, &policy)
                .map_err(ConnectorError::Profile)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(MappedSessionV0 {
        run_id,
        policy,
        events,
        core_events,
        unrepresented,
        findings,
    })
}
