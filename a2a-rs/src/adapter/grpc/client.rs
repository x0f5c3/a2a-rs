//! gRPC client implementation for the A2A protocol
//!
//! This module provides a gRPC client that implements the A2A protocol
//! using the official Protocol Buffer definitions.

use crate::domain::core::{Message, Task};
use crate::port::A2AError;
use crate::services::AsyncA2AClient;
use async_trait::async_trait;
use tonic::transport::Channel;

use super::proto::{
    a2a_service_client::A2aServiceClient, SendMessageRequest, GetTaskRequest,
    CancelTaskRequest, ListTasksRequest, SubscribeToTaskRequest,
};
use super::convert::{to_proto_message, from_proto_task};

/// gRPC client for the A2A protocol
///
/// This client implements the `AsyncA2AClient` trait using gRPC transport
/// according to the A2A Protocol v0.3.0 specification.
///
/// # Example
///
/// ```rust,no_run
/// # #[cfg(feature = "grpc-client")]
/// # {
/// use a2a_rs::GrpcClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = GrpcClient::connect("http://localhost:50051").await?;
///     // Use the client...
///     Ok(())
/// }
/// # }
/// ```
pub struct GrpcClient {
    client: A2aServiceClient<Channel>,
    base_url: String,
}

impl GrpcClient {
    /// Connect to a gRPC server
    ///
    /// # Arguments
    ///
    /// * `url` - The gRPC server URL (e.g., "http://localhost:50051")
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "grpc-client")]
    /// # {
    /// use a2a_rs::GrpcClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = GrpcClient::connect("http://localhost:50051").await?;
    ///     Ok(())
    /// }
    /// # }
    /// ```
    pub async fn connect(url: impl Into<String>) -> Result<Self, A2AError> {
        let url_string = url.into();
        let channel = Channel::from_shared(url_string.clone())
            .map_err(|e| A2AError::TransportError(format!("Invalid URL: {}", e)))?
            .connect()
            .await
            .map_err(|e| A2AError::TransportError(format!("Connection failed: {}", e)))?;

        let client = A2aServiceClient::new(channel);

        Ok(Self {
            client,
            base_url: url_string,
        })
    }

    /// Get the base URL of the gRPC server
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[async_trait]
impl AsyncA2AClient for GrpcClient {
    async fn send_task_message(
        &self,
        task_id: &str,
        message: &Message,
        context_id: Option<String>,
        accepted_output_modes: Option<Vec<String>>,
    ) -> Result<Task, A2AError> {
        let request = SendMessageRequest {
            message: Some(to_proto_message(message)?),
            task_id: task_id.to_string(),
            context_id: context_id.unwrap_or_default(),
            configuration: None, // TODO: Convert accepted_output_modes to configuration
            tenant: String::new(),
        };

        let response = self
            .client
            .clone()
            .send_message(request)
            .await
            .map_err(|e| A2AError::TransportError(format!("gRPC error: {}", e)))?;

        let task = response
            .into_inner()
            .task
            .ok_or_else(|| A2AError::InvalidResponse("No task in response".to_string()))?;

        from_proto_task(task)
    }

    async fn get_task(&self, task_id: &str, history_length: Option<i32>) -> Result<Task, A2AError> {
        let request = GetTaskRequest {
            name: format!("tasks/{}", task_id),
            history_length,
            tenant: String::new(),
        };

        let response = self
            .client
            .clone()
            .get_task(request)
            .await
            .map_err(|e| A2AError::TransportError(format!("gRPC error: {}", e)))?;

        from_proto_task(response.into_inner())
    }

    async fn cancel_task(&self, task_id: &str, reason: Option<String>) -> Result<Task, A2AError> {
        let request = CancelTaskRequest {
            name: format!("tasks/{}", task_id),
            reason: reason.unwrap_or_default(),
            tenant: String::new(),
        };

        let response = self
            .client
            .clone()
            .cancel_task(request)
            .await
            .map_err(|e| A2AError::TransportError(format!("gRPC error: {}", e)))?;

        from_proto_task(response.into_inner())
    }

    async fn list_tasks(
        &self,
        page_size: Option<i32>,
        page_token: Option<String>,
    ) -> Result<Vec<Task>, A2AError> {
        let request = ListTasksRequest {
            page_size: page_size.unwrap_or(50),
            page_token: page_token.unwrap_or_default(),
            filter: String::new(),
            order_by: String::new(),
            tenant: String::new(),
        };

        let response = self
            .client
            .clone()
            .list_tasks(request)
            .await
            .map_err(|e| A2AError::TransportError(format!("gRPC error: {}", e)))?;

        let tasks = response
            .into_inner()
            .tasks
            .into_iter()
            .map(from_proto_task)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tasks)
    }
}
