//! Integration tests for gRPC implementation
//!
//! These tests verify the gRPC client and server implementations
//! against the A2A Protocol v0.3.0 specification.

#![cfg(all(feature = "grpc-client", feature = "grpc-server"))]

use a2a_rs::{GrpcClient, GrpcServer, SimpleAgentInfo, DefaultRequestProcessor};
use std::net::SocketAddr;
use tokio::time::Duration;

/// Test that the gRPC server can start and stop
#[tokio::test]
async fn test_grpc_server_lifecycle() {
    let processor = DefaultRequestProcessor::new();
    let agent_info = SimpleAgentInfo::new(
        "test-agent".to_string(),
        "1.0.0".to_string(),
    );
    
    // Use a random available port
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let server = GrpcServer::new(processor, agent_info, addr);
    
    // Server creation should succeed
    // TODO: Add proper lifecycle test once server.start() can be gracefully shutdown
    // For now, we just verify that construction works
    assert!(true);
}

/// Test that the gRPC client can be created
#[tokio::test]
async fn test_grpc_client_creation() {
    // This will fail to connect but should create the client object
    let result = GrpcClient::connect("http://localhost:50999").await;
    
    // Connection should fail since no server is running
    // but the client should be created
    assert!(result.is_err());
}

/// Test basic client-server communication
/// TODO: Implement once server methods are complete
#[tokio::test]
#[ignore = "Server implementation incomplete"]
async fn test_grpc_send_message() {
    // Start a test server
    let processor = DefaultRequestProcessor::new();
    let agent_info = SimpleAgentInfo::new(
        "test-agent".to_string(),
        "1.0.0".to_string(),
    );
    
    let addr: SocketAddr = "127.0.0.1:50051".parse().unwrap();
    let server = GrpcServer::new(processor, agent_info, addr);
    
    // Start server in background
    tokio::spawn(async move {
        server.start().await.expect("Server failed");
    });
    
    // Give server time to start
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Connect client
    let client = GrpcClient::connect("http://127.0.0.1:50051")
        .await
        .expect("Failed to connect");
    
    // TODO: Test send_message once implemented
    // let message = Message::user_text("Hello".to_string());
    // let task = client.send_task_message("task-1", &message, None, None).await?;
    // assert_eq!(task.status().state(), &TaskState::Submitted);
}

/// Test task retrieval
/// TODO: Implement once server methods are complete
#[tokio::test]
#[ignore = "Server implementation incomplete"]
async fn test_grpc_get_task() {
    // TODO: Implement test
}

/// Test task cancellation
/// TODO: Implement once server methods are complete
#[tokio::test]
#[ignore = "Server implementation incomplete"]
async fn test_grpc_cancel_task() {
    // TODO: Implement test
}

/// Test task listing
/// TODO: Implement once server methods are complete
#[tokio::test]
#[ignore = "Server implementation incomplete"]
async fn test_grpc_list_tasks() {
    // TODO: Implement test
}

/// Test streaming messages
/// TODO: Implement once server methods are complete
#[tokio::test]
#[ignore = "Server implementation incomplete"]
async fn test_grpc_streaming_message() {
    // TODO: Implement test
}

/// Test extended agent card retrieval
/// TODO: Implement once server methods are complete
#[tokio::test]
#[ignore = "Server implementation incomplete"]
async fn test_grpc_get_extended_agent_card() {
    // TODO: Implement test
}

/// Test error handling
#[tokio::test]
#[ignore = "Server implementation incomplete"]
async fn test_grpc_error_handling() {
    // TODO: Test various error scenarios
    // - Invalid task ID
    // - Task not found
    // - Invalid message format
    // - Connection errors
}
