//! gRPC server implementation for the A2A protocol
//!
//! This module provides a gRPC server that implements the A2A protocol
//! using the official Protocol Buffer definitions.

use crate::services::{AsyncA2ARequestProcessor, AgentInfoProvider};
use tonic::{transport::Server, Request, Response, Status};
use std::sync::Arc;
use std::net::SocketAddr;
use futures::Stream;
use std::pin::Pin;

use super::proto::{
    a2a_service_server::{A2aService, A2aServiceServer},
    SendMessageRequest, SendMessageResponse, GetTaskRequest, Task as ProtoTask,
    CancelTaskRequest, ListTasksRequest, ListTasksResponse,
    SubscribeToTaskRequest, StreamResponse,
    SetTaskPushNotificationConfigRequest, TaskPushNotificationConfig,
    GetTaskPushNotificationConfigRequest,
    ListTaskPushNotificationConfigRequest, ListTaskPushNotificationConfigResponse,
    DeleteTaskPushNotificationConfigRequest,
    GetExtendedAgentCardRequest, AgentCard,
};
use super::convert::to_proto_agent_card;

/// gRPC server for the A2A protocol
///
/// This server implements the A2AService gRPC service according to the
/// A2A Protocol v0.3.0 specification.
///
/// # Example
///
/// ```rust,no_run
/// # #[cfg(feature = "grpc-server")]
/// # {
/// use a2a_rs::{GrpcServer, SimpleAgentInfo, DefaultRequestProcessor};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let server = GrpcServer::new(
///         DefaultRequestProcessor::new(),
///         SimpleAgentInfo::new("my-agent".to_string(), "1.0.0".to_string()),
///         "[::1]:50051".parse()?,
///     );
///     server.start().await?;
///     Ok(())
/// }
/// # }
/// ```
pub struct GrpcServer<P>
where
    P: AsyncA2ARequestProcessor + Send + Sync + 'static,
{
    processor: Arc<P>,
    agent_info: Arc<dyn AgentInfoProvider + Send + Sync>,
    addr: SocketAddr,
}

impl<P> GrpcServer<P>
where
    P: AsyncA2ARequestProcessor + Send + Sync + 'static,
{
    /// Create a new gRPC server
    ///
    /// # Arguments
    ///
    /// * `processor` - The request processor to handle A2A operations
    /// * `agent_info` - Agent metadata and capabilities
    /// * `addr` - The socket address to bind to
    pub fn new(
        processor: P,
        agent_info: impl AgentInfoProvider + Send + Sync + 'static,
        addr: SocketAddr,
    ) -> Self {
        Self {
            processor: Arc::new(processor),
            agent_info: Arc::new(agent_info),
            addr,
        }
    }

    /// Start the gRPC server
    ///
    /// This will bind to the configured address and start serving requests.
    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        let service = GrpcServiceImpl {
            processor: self.processor,
            agent_info: self.agent_info,
        };

        tracing::info!("Starting A2A gRPC server on {}", self.addr);

        Server::builder()
            .add_service(A2aServiceServer::new(service))
            .serve(self.addr)
            .await?;

        Ok(())
    }
}

/// Internal gRPC service implementation
struct GrpcServiceImpl<P>
where
    P: AsyncA2ARequestProcessor + Send + Sync + 'static,
{
    processor: Arc<P>,
    agent_info: Arc<dyn AgentInfoProvider + Send + Sync>,
}

#[tonic::async_trait]
impl<P> A2aService for GrpcServiceImpl<P>
where
    P: AsyncA2ARequestProcessor + Send + Sync + 'static,
{
    async fn send_message(
        &self,
        _request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        // TODO: Implement message processing
        // This should:
        // 1. Convert proto message to domain Message
        // 2. Call processor to handle the message
        // 3. Convert domain Task back to proto Task
        // 4. Return the response
        
        Err(Status::unimplemented("send_message not yet implemented"))
    }

    type SendStreamingMessageStream = Pin<Box<dyn Stream<Item = Result<StreamResponse, Status>> + Send>>;

    async fn send_streaming_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<Self::SendStreamingMessageStream>, Status> {
        // TODO: Implement streaming message processing
        Err(Status::unimplemented("send_streaming_message not yet implemented"))
    }

    async fn get_task(
        &self,
        _request: Request<GetTaskRequest>,
    ) -> Result<Response<ProtoTask>, Status> {
        // TODO: Implement task retrieval
        // Extract task ID from the name field (format: "tasks/{task_id}")
        
        Err(Status::unimplemented("get_task not yet implemented"))
    }

    async fn list_tasks(
        &self,
        _request: Request<ListTasksRequest>,
    ) -> Result<Response<ListTasksResponse>, Status> {
        // TODO: Implement task listing
        Err(Status::unimplemented("list_tasks not yet implemented"))
    }

    async fn cancel_task(
        &self,
        _request: Request<CancelTaskRequest>,
    ) -> Result<Response<ProtoTask>, Status> {
        // TODO: Implement task cancellation
        Err(Status::unimplemented("cancel_task not yet implemented"))
    }

    type SubscribeToTaskStream = Pin<Box<dyn Stream<Item = Result<StreamResponse, Status>> + Send>>;

    async fn subscribe_to_task(
        &self,
        _request: Request<SubscribeToTaskRequest>,
    ) -> Result<Response<Self::SubscribeToTaskStream>, Status> {
        // TODO: Implement task subscription/streaming
        Err(Status::unimplemented("subscribe_to_task not yet implemented"))
    }

    async fn set_task_push_notification_config(
        &self,
        _request: Request<SetTaskPushNotificationConfigRequest>,
    ) -> Result<Response<TaskPushNotificationConfig>, Status> {
        // TODO: Implement push notification config
        Err(Status::unimplemented("set_task_push_notification_config not yet implemented"))
    }

    async fn get_task_push_notification_config(
        &self,
        request: Request<GetTaskPushNotificationConfigRequest>,
    ) -> Result<Response<TaskPushNotificationConfig>, Status> {
        // TODO: Implement push notification config retrieval
        Err(Status::unimplemented("get_task_push_notification_config not yet implemented"))
    }

    async fn list_task_push_notification_config(
        &self,
        request: Request<ListTaskPushNotificationConfigRequest>,
    ) -> Result<Response<ListTaskPushNotificationConfigResponse>, Status> {
        // TODO: Implement push notification config listing
        Err(Status::unimplemented("list_task_push_notification_config not yet implemented"))
    }

    async fn get_extended_agent_card(
        &self,
        _request: Request<GetExtendedAgentCardRequest>,
    ) -> Result<Response<AgentCard>, Status> {
        // Convert our agent info to proto AgentCard
        let agent_card = self.agent_info.get_agent_card();
        let card = to_proto_agent_card(&agent_card)
            .map_err(|e| Status::internal(format!("Failed to convert agent card: {}", e)))?;
        
        Ok(Response::new(card))
    }

    async fn delete_task_push_notification_config(
        &self,
        request: Request<DeleteTaskPushNotificationConfigRequest>,
    ) -> Result<Response<()>, Status> {
        // TODO: Implement push notification config deletion
        Err(Status::unimplemented("delete_task_push_notification_config not yet implemented"))
    }
}
