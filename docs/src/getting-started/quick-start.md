# Quick Start

This guide will help you create your first A2A agent in minutes.

## Simple HTTP Server

```rust
use a2a_rs::{HttpServer, InMemoryTaskStorage, DefaultRequestProcessor, DefaultMessageHandler, SimpleAgentInfo};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create storage, handlers, and agent info
    let storage = InMemoryTaskStorage::new();
    let message_handler = DefaultMessageHandler::new(storage.clone());
    let agent = SimpleAgentInfo::new("my-agent".into(), "1.0.0".into());
    
    // Create processor with all required handlers
    let processor = DefaultRequestProcessor::new(
        message_handler,
        storage.clone(),
        storage.clone(), // storage also implements AsyncNotificationManager
        agent.clone(),
    );
    
    // Create and start HTTP server
    let server = HttpServer::new(processor, agent, "127.0.0.1:3000".parse()?);
    
    println!("Starting server on http://127.0.0.1:3000");
    server.start().await?;
    
    Ok(())
}
```

## gRPC Server (Production-Ready)

```rust
use a2a_rs::{GrpcServer, InMemoryTaskStorage, DefaultMessageHandler, SimpleAgentInfo};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up managers
    let task_storage = InMemoryTaskStorage::new();
    let message_handler = DefaultMessageHandler::new(task_storage.clone());
    let agent_info = SimpleAgentInfo::new("my-agent".into(), "1.0.0".into());
    
    // Create gRPC server
    let server = GrpcServer::new(
        task_storage,
        message_handler,
        agent_info,
        "[::1]:50051".parse()?,
    );
    
    println!("Starting gRPC server on [::1]:50051");
    server.start().await?;
    
    Ok(())
}
```

## WebSocket Server

```rust
use a2a_rs::{WebSocketServer, InMemoryTaskStorage, DefaultRequestProcessor, DefaultMessageHandler, SimpleAgentInfo};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create storage, handlers, and agent info
    let storage = InMemoryTaskStorage::new();
    let message_handler = DefaultMessageHandler::new(storage.clone());
    let agent = SimpleAgentInfo::new("my-agent".into(), "1.0.0".into());
    
    // Create processor with all required handlers
    let processor = DefaultRequestProcessor::new(
        message_handler,
        storage.clone(),
        storage.clone(), // storage also implements AsyncNotificationManager
        agent.clone(),
    );
    
    let server = WebSocketServer::new(processor, agent, "127.0.0.1:3001".parse()?);
    
    println!("Starting WebSocket server on ws://127.0.0.1:3001");
    server.start().await?;
    
    Ok(())
}
```

## Making Requests

Once your server is running, you can send requests:

```bash
# HTTP JSON-RPC
curl -X POST http://127.0.0.1:3000 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tasks/list",
    "params": {},
    "id": 1
  }'
```

For gRPC, use a tool like `grpcurl` or the gRPC client.

## Next Steps

- Learn about [Basic Concepts](./concepts.md)
- Explore [gRPC Server Features](../features/grpc-server.md)
- See [Integration Guide](../integration/architecture.md)
