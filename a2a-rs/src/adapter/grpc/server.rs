//! gRPC server implementation for the A2A protocol
//!
//! This module provides a gRPC server that implements the A2A protocol
//! using the official Protocol Buffer definitions.

use crate::services::AgentInfoProvider;
use crate::port::{AsyncTaskManager, AsyncMessageHandler, AsyncNotificationManager, AsyncStreamingHandler};
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
use super::convert::{to_proto_agent_card, to_proto_task, from_proto_message};

/// gRPC server for the A2A protocol
///
/// This server implements the A2AService gRPC service according to the
/// A2A Protocol v0.3.0 specification.
///
/// The server uses direct manager access for optimal performance and
/// clean separation of concerns.
///
/// # Example
///
/// ```rust,no_run
/// # #[cfg(feature = "grpc-server")]
/// # {
/// use a2a_rs::{GrpcServer, SimpleAgentInfo, InMemoryTaskStorage, DefaultMessageHandler};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let task_storage = InMemoryTaskStorage::new();
///     let message_handler = DefaultMessageHandler::new(task_storage.clone());
///     let agent_info = SimpleAgentInfo::new("my-agent".to_string(), "1.0.0".to_string());
///     
///     let server = GrpcServer::new(
///         task_storage,
///         message_handler,
///         agent_info,
///         "[::1]:50051".parse()?,
///     );
///     server.start().await?;
///     Ok(())
/// }
/// # }
/// ```
pub struct GrpcServer<T, M>
where
    T: AsyncTaskManager + Send + Sync + 'static,
    M: AsyncMessageHandler + Send + Sync + 'static,
{
    task_manager: Arc<T>,
    message_handler: Arc<M>,
    agent_info: Arc<dyn AgentInfoProvider + Send + Sync>,
    addr: SocketAddr,
}

impl<T, M> GrpcServer<T, M>
where
    T: AsyncTaskManager + Send + Sync + 'static,
    M: AsyncMessageHandler + Send + Sync + 'static,
{
    /// Create a new gRPC server
    ///
    /// # Arguments
    ///
    /// * `task_manager` - Task manager for task lifecycle operations
    /// * `message_handler` - Message handler for processing messages
    /// * `agent_info` - Agent metadata and capabilities
    /// * `addr` - The socket address to bind to
    pub fn new(
        task_manager: T,
        message_handler: M,
        agent_info: impl AgentInfoProvider + Send + Sync + 'static,
        addr: SocketAddr,
    ) -> Self {
        Self {
            task_manager: Arc::new(task_manager),
            message_handler: Arc::new(message_handler),
            agent_info: Arc::new(agent_info),
            addr,
        }
    }

    /// Start the gRPC server
    ///
    /// This will bind to the configured address and start serving requests.
    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        let service = GrpcServiceImpl {
            task_manager: self.task_manager,
            message_handler: self.message_handler,
            agent_info: self.agent_info,
        };

        #[cfg(feature = "tracing")]
        tracing::info!("Starting A2A gRPC server on {}", self.addr);

        Server::builder()
            .add_service(A2aServiceServer::new(service))
            .serve(self.addr)
            .await?;

        Ok(())
    }
}

/// Internal gRPC service implementation
struct GrpcServiceImpl<T, M>
where
    T: AsyncTaskManager + Send + Sync + 'static,
    M: AsyncMessageHandler + Send + Sync + 'static,
{
    task_manager: Arc<T>,
    message_handler: Arc<M>,
    agent_info: Arc<dyn AgentInfoProvider + Send + Sync>,
}

#[tonic::async_trait]
impl<T, M> A2aService for GrpcServiceImpl<T, M>
where
    T: AsyncTaskManager + Send + Sync + 'static,
    M: AsyncMessageHandler + Send + Sync + 'static,
{
    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let req = request.into_inner();
        
        // Extract the message from the request
        let proto_message = req.request
            .ok_or_else(|| Status::invalid_argument("Message is required"))?;
        
        // Convert proto Message to domain Message
        let message = from_proto_message(proto_message)
            .map_err(|e| Status::internal(format!("Failed to convert message: {}", e)))?;
        
        // Extract task_id and context_id from the message
        let task_id = message.task_id.as_deref().unwrap_or("");
        
        // Process the message using the message handler
        let task = self.message_handler
            .process_message(task_id, &message, None)
            .await
            .map_err(|e| Status::internal(format!("Failed to process message: {}", e)))?;
        
        // Convert domain Task to proto Task
        let proto_task = to_proto_task(&task)
            .map_err(|e| Status::internal(format!("Failed to convert task: {}", e)))?;
        
        // Return response with task
        Ok(Response::new(SendMessageResponse {
            payload: Some(super::proto::send_message_response::Payload::Task(proto_task)),
        }))
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
        request: Request<GetTaskRequest>,
    ) -> Result<Response<ProtoTask>, Status> {
        let req = request.into_inner();
        
        // Extract task ID from the name field (format: "tasks/{task_id}")
        let task_id = req.name
            .strip_prefix("tasks/")
            .ok_or_else(|| Status::invalid_argument("Invalid task name format. Expected: tasks/{task_id}"))?
            .to_string();
        
        // Convert history_length from Option<i32> to Option<u32>
        let history_length = req.history_length.and_then(|l| {
            if l >= 0 {
                Some(l as u32)
            } else {
                None
            }
        });
        
        // Get the task using the task manager
        let task = self.task_manager
            .get_task(&task_id, history_length)
            .await
            .map_err(|e| match e {
                crate::domain::error::A2AError::TaskNotFound(_) => Status::not_found(format!("{}", e)),
                _ => Status::internal(format!("Failed to get task: {}", e)),
            })?;
        
        // Convert domain Task to proto Task
        let proto_task = to_proto_task(&task)
            .map_err(|e| Status::internal(format!("Failed to convert task: {}", e)))?;
        
        Ok(Response::new(proto_task))
    }

    async fn list_tasks(
        &self,
        request: Request<ListTasksRequest>,
    ) -> Result<Response<ListTasksResponse>, Status> {
        let req = request.into_inner();
        
        // Parse page_size, defaulting to 50
        let page_size = req.page_size.unwrap_or(50);
        let limit = if page_size > 0 {
            Some(page_size as u32)
        } else {
            Some(50)
        };
        
        // List tasks using the task manager
        let tasks = self.task_manager
            .list_tasks(None, limit)
            .await
            .map_err(|e| Status::internal(format!("Failed to list tasks: {}", e)))?;
        
        let total_count = tasks.len() as i32;
        
        // Convert domain Tasks to proto Tasks
        let proto_tasks = tasks
            .iter()
            .map(to_proto_task)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| Status::internal(format!("Failed to convert tasks: {}", e)))?;
        
        Ok(Response::new(ListTasksResponse {
            tasks: proto_tasks,
            next_page_token: String::new(), // TODO: Implement pagination token
            page_size,
            total_size: total_count,
        }))
    }

    async fn cancel_task(
        &self,
        request: Request<CancelTaskRequest>,
    ) -> Result<Response<ProtoTask>, Status> {
        let req = request.into_inner();
        
        // Extract task ID from the name field (format: "tasks/{task_id}")
        let task_id = req.name
            .strip_prefix("tasks/")
            .ok_or_else(|| Status::invalid_argument("Invalid task name format. Expected: tasks/{task_id}"))?
            .to_string();
        
        // Cancel the task using the task manager
        let task = self.task_manager
            .cancel_task(&task_id)
            .await
            .map_err(|e| match e {
                crate::domain::error::A2AError::TaskNotFound(_) => Status::not_found(format!("{}", e)),
                crate::domain::error::A2AError::TaskNotCancelable(_) => Status::failed_precondition(format!("{}", e)),
                _ => Status::internal(format!("Failed to cancel task: {}", e)),
            })?;
        
        // Convert domain Task to proto Task
        let proto_task = to_proto_task(&task)
            .map_err(|e| Status::internal(format!("Failed to convert task: {}", e)))?;
        
        Ok(Response::new(proto_task))
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
        let agent_card = self.agent_info.get_agent_card().await
            .map_err(|e| Status::internal(format!("Failed to get agent card: {}", e)))?;
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
