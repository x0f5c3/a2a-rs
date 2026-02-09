//! Conversion utilities between Protocol Buffer types and domain types
//!
//! This module provides functions to convert between the gRPC Protocol Buffer
//! message types and the internal domain model types.

use crate::domain::core::{
    Message, Task, TaskStatus, TaskState, Part, Artifact, AgentCard,
};
use crate::domain::error::A2AError;
use super::proto;

/// Convert a domain Message to a proto Message
pub fn to_proto_message(message: &Message) -> Result<proto::Message, A2AError> {
    // Convert role to proto Role enum value
    let role_value = match message.role {
        crate::domain::core::Role::User => proto::Role::User as i32,
        crate::domain::core::Role::Agent => proto::Role::Agent as i32,
    };
    
    // Convert parts
    let parts = message.parts.iter()
        .map(to_proto_part)
        .collect::<Result<Vec<_>, _>>()?;
    
    Ok(proto::Message {
        message_id: message.message_id.clone(),
        context_id: message.context_id.clone().unwrap_or_default(),
        task_id: message.task_id.clone().unwrap_or_default(),
        role: role_value,
        parts,
        metadata: None, // TODO: Convert metadata
        extensions: message.extensions.clone().unwrap_or_default(),
        reference_task_ids: message.reference_task_ids.clone().unwrap_or_default(),
    })
}

/// Convert a proto Message to a domain Message
pub fn from_proto_message(message: proto::Message) -> Result<Message, A2AError> {
    use crate::domain::core::Role;
    
    // Convert proto role to domain role
    let role = match proto::Role::try_from(message.role) {
        Ok(proto::Role::User) => Role::User,
        Ok(proto::Role::Agent) => Role::Agent,
        _ => Role::User, // Default to User if unspecified
    };
    
    // Convert parts
    let parts = message.parts.into_iter()
        .map(from_proto_part)
        .collect::<Result<Vec<_>, _>>()?;
    
    Ok(Message {
        message_id: message.message_id,
        role,
        parts,
        metadata: None, // TODO: Convert metadata from proto Struct
        reference_task_ids: if message.reference_task_ids.is_empty() {
            None
        } else {
            Some(message.reference_task_ids)
        },
        task_id: if message.task_id.is_empty() {
            None
        } else {
            Some(message.task_id)
        },
        context_id: if message.context_id.is_empty() {
            None
        } else {
            Some(message.context_id)
        },
        extensions: if message.extensions.is_empty() {
            None
        } else {
            Some(message.extensions)
        },
        kind: "message".to_string(),
    })
}

/// Convert a domain Task to a proto Task
pub fn to_proto_task(task: &Task) -> Result<proto::Task, A2AError> {
    // Convert status
    let status = Some(to_proto_task_status(&task.status)?);
    
    // Convert artifacts
    let artifacts = task.artifacts.as_ref()
        .map(|arts| arts.iter().map(to_proto_artifact).collect::<Result<Vec<_>, _>>())
        .transpose()?
        .unwrap_or_default();
    
    // Convert history
    let history = task.history.as_ref()
        .map(|hist| hist.iter().map(to_proto_message).collect::<Result<Vec<_>, _>>())
        .transpose()?
        .unwrap_or_default();
    
    Ok(proto::Task {
        id: task.id.clone(),
        context_id: task.context_id.clone(),
        status,
        artifacts,
        history,
        metadata: None, // TODO: Convert metadata
    })
}

/// Convert a proto Task to a domain Task
pub fn from_proto_task(_task: proto::Task) -> Result<Task, A2AError> {
    // TODO: Implement full conversion
    // This is a skeleton implementation
    Err(A2AError::UnsupportedOperation("Conversion not yet implemented".to_string()))
}

/// Convert a domain TaskStatus to a proto TaskStatus
pub fn to_proto_task_status(status: &TaskStatus) -> Result<proto::TaskStatus, A2AError> {
    use prost_types::Timestamp;
    
    let message = status.message.as_ref()
        .map(|m| to_proto_message(m))
        .transpose()?;
    
    let timestamp = status.timestamp.as_ref()
        .map(|dt| {
            Timestamp {
                seconds: dt.timestamp(),
                nanos: dt.timestamp_subsec_nanos() as i32,
            }
        });
    
    Ok(proto::TaskStatus {
        state: to_proto_task_state(&status.state) as i32,
        message,
        timestamp,
    })
}

/// Convert a proto TaskStatus to a domain TaskStatus
pub fn from_proto_task_status(_status: proto::TaskStatus) -> Result<TaskStatus, A2AError> {
    // TODO: Implement full conversion
    Err(A2AError::UnsupportedOperation("Conversion not yet implemented".to_string()))
}

/// Convert a domain TaskState to a proto TaskState
pub fn to_proto_task_state(state: &TaskState) -> proto::TaskState {
    match state {
        TaskState::Submitted => proto::TaskState::Submitted,
        TaskState::Working => proto::TaskState::Working,
        TaskState::Completed => proto::TaskState::Completed,
        TaskState::Failed => proto::TaskState::Failed,
        TaskState::Canceled => proto::TaskState::Cancelled,
        TaskState::InputRequired => proto::TaskState::InputRequired,
        TaskState::Rejected => proto::TaskState::Rejected,
        TaskState::AuthRequired => proto::TaskState::AuthRequired,
        TaskState::Unknown => proto::TaskState::Unspecified,
    }
}

/// Convert a proto TaskState to a domain TaskState
pub fn from_proto_task_state(state: i32) -> Result<TaskState, A2AError> {
    match proto::TaskState::try_from(state) {
        Ok(proto::TaskState::Submitted) => Ok(TaskState::Submitted),
        Ok(proto::TaskState::Working) => Ok(TaskState::Working),
        Ok(proto::TaskState::Completed) => Ok(TaskState::Completed),
        Ok(proto::TaskState::Failed) => Ok(TaskState::Failed),
        Ok(proto::TaskState::Cancelled) => Ok(TaskState::Canceled),
        Ok(proto::TaskState::InputRequired) => Ok(TaskState::InputRequired),
        Ok(proto::TaskState::Rejected) => Ok(TaskState::Rejected),
        Ok(proto::TaskState::AuthRequired) => Ok(TaskState::AuthRequired),
        Ok(proto::TaskState::Unspecified) | _ => Ok(TaskState::Unknown),
    }
}

/// Convert a domain Part to a proto Part
pub fn to_proto_part(part: &Part) -> Result<proto::Part, A2AError> {
    let part_content = match part {
        Part::Text { text, .. } => Some(proto::part::Part::Text(text.clone())),
        Part::File { file, .. } => {
            let file_part = if let Some(uri) = &file.uri {
                proto::FilePart {
                    file: Some(proto::file_part::File::FileWithUri(uri.clone())),
                    media_type: file.mime_type.clone().unwrap_or_default(),
                    name: file.name.clone().unwrap_or_default(),
                }
            } else if let Some(bytes_str) = &file.bytes {
                // Decode base64 string to Vec<u8>
                use base64::{Engine as _, engine::general_purpose::STANDARD};
                let bytes_vec = STANDARD.decode(bytes_str)
                    .map_err(|e| A2AError::Internal(format!("Failed to decode base64: {}", e)))?;
                proto::FilePart {
                    file: Some(proto::file_part::File::FileWithBytes(bytes_vec)),
                    media_type: file.mime_type.clone().unwrap_or_default(),
                    name: file.name.clone().unwrap_or_default(),
                }
            } else {
                return Err(A2AError::Internal("File must have either URI or bytes".to_string()));
            };
            Some(proto::part::Part::File(file_part))
        }
        Part::Data { data, .. } => {
            // TODO: Implement proper conversion from serde_json::Map to prost_types::Struct
            // This requires converting each Value type to prost_types::Value
            // For now, we skip data parts to avoid data loss/corruption
            // Users should use Text or File parts for structured data until this is implemented
            Some(proto::part::Part::Data(proto::DataPart {
                data: None,
            }))
        }
    };
    
    Ok(proto::Part {
        part: part_content,
        metadata: None, // TODO: Convert metadata
    })
}

/// Convert a proto Part to a domain Part
pub fn from_proto_part(part: proto::Part) -> Result<Part, A2AError> {
    use crate::domain::core::FileContent;
    
    match part.part {
        Some(proto::part::Part::Text(text)) => {
            Ok(Part::Text {
                text,
                metadata: None, // TODO: Convert metadata
            })
        }
        Some(proto::part::Part::File(file_part)) => {
            let (uri, bytes) = match file_part.file {
                Some(proto::file_part::File::FileWithUri(uri)) => (Some(uri), None),
                Some(proto::file_part::File::FileWithBytes(bytes_vec)) => {
                    // Convert Vec<u8> to base64 string
                    use base64::{Engine as _, engine::general_purpose::STANDARD};
                    let bytes_str = STANDARD.encode(&bytes_vec);
                    (None, Some(bytes_str))
                }
                None => {
                    return Err(A2AError::InvalidRequest(
                        "File part must have either URI or bytes".to_string(),
                    ));
                }
            };
            
            let file_content = FileContent {
                name: if file_part.name.is_empty() { None } else { Some(file_part.name) },
                mime_type: if file_part.media_type.is_empty() { None } else { Some(file_part.media_type) },
                bytes,
                uri,
            };
            
            Ok(Part::File {
                file: file_content,
                metadata: None, // TODO: Convert metadata
            })
        }
        Some(proto::part::Part::Data(_data_part)) => {
            // TODO: Implement proper conversion from prost_types::Struct to serde_json::Map
            // This requires converting prost_types::Value to serde_json::Value
            // For now, we return empty data to avoid corruption
            // Users should use Text or File parts for structured data until this is implemented
            Ok(Part::Data {
                data: serde_json::Map::new(),
                metadata: None,
            })
        }
        None => {
            Err(A2AError::InvalidRequest(
                "Part must have content".to_string(),
            ))
        }
    }
}

/// Convert a domain Artifact to a proto Artifact
pub fn to_proto_artifact(artifact: &Artifact) -> Result<proto::Artifact, A2AError> {
    let parts = artifact.parts.iter()
        .map(to_proto_part)
        .collect::<Result<Vec<_>, _>>()?;
    
    Ok(proto::Artifact {
        artifact_id: artifact.artifact_id.clone(),
        name: String::new(), // TODO: Domain Artifact doesn't have name field
        description: String::new(), // TODO: Domain Artifact doesn't have description field  
        parts,
        metadata: None, // TODO: Convert metadata
        extensions: vec![], // TODO: Domain Artifact doesn't have extensions field
    })
}

/// Convert a proto Artifact to a domain Artifact
pub fn from_proto_artifact(_artifact: proto::Artifact) -> Result<Artifact, A2AError> {
    // TODO: Implement full conversion
    Err(A2AError::UnsupportedOperation("Conversion not yet implemented".to_string()))
}

/// Convert AgentCard to a proto AgentCard
pub fn to_proto_agent_card(agent_card: &AgentCard) -> Result<proto::AgentCard, A2AError> {
    // TODO: Complete conversion of complex nested types
    // This is a partial implementation with correct field mappings
    
    use std::collections::HashMap;
    
    Ok(proto::AgentCard {
        name: agent_card.name.clone(),
        description: agent_card.description.clone(),
        version: agent_card.version.clone(),
        protocol_version: Some(agent_card.protocol_version.clone()),
        
        // Deprecated fields - set from current values for backward compatibility
        #[allow(deprecated)]
        url: Some(agent_card.url.clone()),
        #[allow(deprecated)]
        preferred_transport: None,
        #[allow(deprecated)]
        additional_interfaces: vec![],
        
        // New fields
        supported_interfaces: vec![], // TODO: Convert interfaces
        provider: None, // TODO: Convert provider
        documentation_url: None,
        icon_url: agent_card.icon_url.clone(),
        
        // Content type modes
        default_input_modes: agent_card.default_input_modes.iter().map(|m| m.to_string()).collect(),
        default_output_modes: agent_card.default_output_modes.iter().map(|m| m.to_string()).collect(),
        
        // Capabilities and skills
        capabilities: None, // TODO: Convert capabilities
        skills: vec![],     // TODO: Convert skills
        
        // Security
        security: vec![],   // TODO: Convert security requirements
        security_schemes: HashMap::new(), // TODO: Convert security schemes
        
        // Extended card support
        supports_extended_agent_card: agent_card.supports_authenticated_extended_card,
        
        // Signatures
        signatures: vec![],
    })
}

/// Convert a proto AgentCard to domain types
pub fn from_proto_agent_card(_card: proto::AgentCard) -> Result<(), A2AError> {
    // TODO: Implement full conversion
    Err(A2AError::UnsupportedOperation("Conversion not yet implemented".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::core::TaskState;

    #[test]
    fn test_task_state_conversions() {
        // Test all task state conversions
        let states = vec![
            (TaskState::Submitted, proto::TaskState::Submitted),
            (TaskState::Working, proto::TaskState::Working),
            (TaskState::Completed, proto::TaskState::Completed),
            (TaskState::Failed, proto::TaskState::Failed),
            (TaskState::Canceled, proto::TaskState::Cancelled),
            (TaskState::InputRequired, proto::TaskState::InputRequired),
            (TaskState::Rejected, proto::TaskState::Rejected),
            (TaskState::AuthRequired, proto::TaskState::AuthRequired),
            (TaskState::Unknown, proto::TaskState::Unspecified),
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
