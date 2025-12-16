//! Conversion utilities between Protocol Buffer types and domain types
//!
//! This module provides functions to convert between the gRPC Protocol Buffer
//! message types and the internal domain model types.

use crate::domain::core::{
    Message, Task, TaskStatus, TaskState, Part, Artifact, AgentInfo,
};
use crate::port::A2AError;
use super::proto;
use prost_types::Timestamp;

/// Convert a domain Message to a proto Message
pub fn to_proto_message(message: &Message) -> Result<proto::Message, A2AError> {
    // TODO: Implement full conversion
    // This is a skeleton implementation
    Ok(proto::Message {
        id: message.id().to_string(),
        role: message.role().to_string(),
        parts: vec![], // TODO: Convert parts
        timestamp: None, // TODO: Convert timestamp
        metadata: None,
    })
}

/// Convert a proto Message to a domain Message
pub fn from_proto_message(message: proto::Message) -> Result<Message, A2AError> {
    // TODO: Implement full conversion
    // This is a skeleton implementation
    Err(A2AError::InvalidResponse("Conversion not yet implemented".to_string()))
}

/// Convert a domain Task to a proto Task
pub fn to_proto_task(task: &Task) -> Result<proto::Task, A2AError> {
    // TODO: Implement full conversion
    // This is a skeleton implementation
    Ok(proto::Task {
        id: task.id().to_string(),
        context_id: task.context_id().to_string(),
        status: Some(to_proto_task_status(task.status())?),
        artifacts: vec![], // TODO: Convert artifacts
        history: vec![],   // TODO: Convert history
        metadata: None,
    })
}

/// Convert a proto Task to a domain Task
pub fn from_proto_task(task: proto::Task) -> Result<Task, A2AError> {
    // TODO: Implement full conversion
    // This is a skeleton implementation
    Err(A2AError::InvalidResponse("Conversion not yet implemented".to_string()))
}

/// Convert a domain TaskStatus to a proto TaskStatus
pub fn to_proto_task_status(status: &TaskStatus) -> Result<proto::TaskStatus, A2AError> {
    Ok(proto::TaskStatus {
        state: to_proto_task_state(status.state()) as i32,
        message: status.message().map(|m| to_proto_message(m)).transpose()?,
        timestamp: None, // TODO: Convert timestamp
    })
}

/// Convert a proto TaskStatus to a domain TaskStatus
pub fn from_proto_task_status(status: proto::TaskStatus) -> Result<TaskStatus, A2AError> {
    // TODO: Implement full conversion
    Err(A2AError::InvalidResponse("Conversion not yet implemented".to_string()))
}

/// Convert a domain TaskState to a proto TaskState
pub fn to_proto_task_state(state: &TaskState) -> proto::TaskState {
    match state {
        TaskState::Submitted => proto::TaskState::TaskStateSubmitted,
        TaskState::Working => proto::TaskState::TaskStateWorking,
        TaskState::Completed => proto::TaskState::TaskStateCompleted,
        TaskState::Failed => proto::TaskState::TaskStateFailed,
        TaskState::Cancelled => proto::TaskState::TaskStateCancelled,
        TaskState::InputRequired => proto::TaskState::TaskStateInputRequired,
        TaskState::Rejected => proto::TaskState::TaskStateRejected,
        TaskState::AuthRequired => proto::TaskState::TaskStateAuthRequired,
        TaskState::Unknown => proto::TaskState::TaskStateUnspecified,
    }
}

/// Convert a proto TaskState to a domain TaskState
pub fn from_proto_task_state(state: i32) -> Result<TaskState, A2AError> {
    match proto::TaskState::try_from(state) {
        Ok(proto::TaskState::TaskStateSubmitted) => Ok(TaskState::Submitted),
        Ok(proto::TaskState::TaskStateWorking) => Ok(TaskState::Working),
        Ok(proto::TaskState::TaskStateCompleted) => Ok(TaskState::Completed),
        Ok(proto::TaskState::TaskStateFailed) => Ok(TaskState::Failed),
        Ok(proto::TaskState::TaskStateCancelled) => Ok(TaskState::Cancelled),
        Ok(proto::TaskState::TaskStateInputRequired) => Ok(TaskState::InputRequired),
        Ok(proto::TaskState::TaskStateRejected) => Ok(TaskState::Rejected),
        Ok(proto::TaskState::TaskStateAuthRequired) => Ok(TaskState::AuthRequired),
        Ok(proto::TaskState::TaskStateUnspecified) | _ => Ok(TaskState::Unknown),
    }
}

/// Convert a domain Part to a proto Part
pub fn to_proto_part(part: &Part) -> Result<proto::Part, A2AError> {
    // TODO: Implement full conversion
    Ok(proto::Part {
        part: None, // TODO: Handle different part types
        metadata: None,
    })
}

/// Convert a proto Part to a domain Part
pub fn from_proto_part(part: proto::Part) -> Result<Part, A2AError> {
    // TODO: Implement full conversion
    Err(A2AError::InvalidResponse("Conversion not yet implemented".to_string()))
}

/// Convert a domain Artifact to a proto Artifact
pub fn to_proto_artifact(artifact: &Artifact) -> Result<proto::Artifact, A2AError> {
    // TODO: Implement full conversion
    Ok(proto::Artifact {
        id: artifact.id().to_string(),
        parts: vec![], // TODO: Convert parts
        metadata: None,
    })
}

/// Convert a proto Artifact to a domain Artifact
pub fn from_proto_artifact(artifact: proto::Artifact) -> Result<Artifact, A2AError> {
    // TODO: Implement full conversion
    Err(A2AError::InvalidResponse("Conversion not yet implemented".to_string()))
}

/// Convert AgentInfo to a proto AgentCard
pub fn to_proto_agent_card(agent_info: &dyn AgentInfo) -> Result<proto::AgentCard, A2AError> {
    // TODO: Implement full conversion
    // This is a skeleton implementation
    Ok(proto::AgentCard {
        name: agent_info.name().to_string(),
        description: agent_info.description().unwrap_or("").to_string(),
        version: agent_info.version().to_string(),
        url: agent_info.url().unwrap_or("").to_string(),
        protocol_version: "0.3.0".to_string(),
        preferred_transport: String::new(),
        additional_interfaces: vec![],
        icon_url: String::new(),
        capabilities: None, // TODO: Convert capabilities
        skills: vec![],     // TODO: Convert skills
        security: None,
        security_schemes: vec![],
        signatures: vec![],
        supports_authenticated_extended_card: false,
        default_input_modes: vec![],
        default_output_modes: vec![],
    })
}

/// Convert a proto AgentCard to domain types
pub fn from_proto_agent_card(card: proto::AgentCard) -> Result<(), A2AError> {
    // TODO: Implement full conversion
    Err(A2AError::InvalidResponse("Conversion not yet implemented".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::core::TaskState;

    #[test]
    fn test_task_state_conversions() {
        // Test all task state conversions
        let states = vec![
            (TaskState::Submitted, proto::TaskState::TaskStateSubmitted),
            (TaskState::Working, proto::TaskState::TaskStateWorking),
            (TaskState::Completed, proto::TaskState::TaskStateCompleted),
            (TaskState::Failed, proto::TaskState::TaskStateFailed),
            (TaskState::Cancelled, proto::TaskState::TaskStateCancelled),
            (TaskState::InputRequired, proto::TaskState::TaskStateInputRequired),
            (TaskState::Rejected, proto::TaskState::TaskStateRejected),
            (TaskState::AuthRequired, proto::TaskState::TaskStateAuthRequired),
            (TaskState::Unknown, proto::TaskState::TaskStateUnspecified),
        ];

        for (domain_state, proto_state) in states {
            // Test domain -> proto
            let converted_proto = to_proto_task_state(&domain_state);
            assert_eq!(converted_proto as i32, proto_state as i32);

            // Test proto -> domain
            let converted_domain = from_proto_task_state(proto_state as i32).unwrap();
            assert_eq!(converted_domain, domain_state);
        }
    }

    #[test]
    fn test_invalid_task_state() {
        // Test that invalid proto state converts to Unknown
        let result = from_proto_task_state(9999);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), TaskState::Unknown);
    }
}
