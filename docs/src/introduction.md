# A2A Protocol - Rust Implementation

Welcome to the **a2a-rs** documentation! This is a production-ready Rust implementation of the [A2A (Agent-to-Agent) Protocol v0.3.0](https://github.com/a2aproject/A2A), designed for building intelligent agent systems with flexible coordination and LLM integration.

## What is A2A?

The A2A Protocol is a standard communication protocol for intelligent agents, enabling:

- **Task Management**: Create, track, and manage asynchronous tasks
- **Message Exchange**: Rich message format supporting text, files, and structured data
- **Multiple Transports**: HTTP, WebSocket, and gRPC support
- **Streaming**: Real-time updates and bidirectional communication
- **Push Notifications**: Webhook-based task completion notifications

## Key Features

### 🚀 Multiple Transport Protocols

- **HTTP Server**: RESTful JSON-RPC API
- **WebSocket Server**: Bidirectional streaming communication
- **gRPC Server**: High-performance binary protocol (NEW!)

### 🔧 Modular Architecture

- **Trait-based design**: Easy to customize and extend
- **Pluggable storage**: In-memory, SQLite, PostgreSQL
- **Flexible authentication**: API Key, Bearer Token, JWT, OAuth2
- **Custom handlers**: Bring your own task and message processing logic

### 🌐 Integration-Ready

Perfect for integrating with:
- Different LLM providers (OpenAI, Anthropic, local models)
- Custom coordination methods
- Existing Rust projects and microservices
- Multi-agent systems like Kowalski

### ✅ Production-Ready

- Comprehensive error handling
- Async/await throughout
- Type-safe domain model
- Well-tested core functionality
- Clear separation of concerns

## Quick Example

```rust
use a2a_rs::{
    GrpcServer, InMemoryTaskStorage, DefaultMessageHandler, 
    SimpleAgentInfo
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up storage and handlers
    let task_storage = InMemoryTaskStorage::new();
    let message_handler = DefaultMessageHandler::new(task_storage.clone());
    let agent_info = SimpleAgentInfo::new(
        "my-agent".to_string(),
        "1.0.0".to_string()
    );
    
    // Create gRPC server
    let server = GrpcServer::new(
        task_storage,
        message_handler,
        agent_info,
        "[::1]:50051".parse()?,
    );
    
    // Start serving
    server.start().await?;
    Ok(())
}
```

## Getting Started

Ready to dive in? Head over to the [Installation](./getting-started/installation.md) guide to get started!

## Protocol Versions

This implementation supports:
- ✅ **A2A Protocol v0.3.0** (current)
- HTTP/JSON-RPC transport
- WebSocket transport
- gRPC transport (core features)

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Community

- **GitHub**: [x0f5c3/a2a-rs](https://github.com/x0f5c3/a2a-rs)
- **Issues**: Report bugs and request features
- **Discussions**: Ask questions and share ideas

## Next Steps

- [Installation Guide](./getting-started/installation.md)
- [Quick Start Tutorial](./getting-started/quick-start.md)
- [Core Concepts](./getting-started/concepts.md)
