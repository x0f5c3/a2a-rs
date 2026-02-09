# gRPC Support in a2a-rs

This document describes the gRPC implementation in a2a-rs, which follows the official A2A Protocol v0.3.0 specification.

## Overview

The gRPC implementation provides an alternative transport layer for the A2A protocol, complementing the existing HTTP/JSON-RPC and WebSocket implementations. It uses Protocol Buffers for efficient serialization and supports all A2A operations defined in the specification.

## Features

- ✅ Official Protocol Buffer definitions from [a2aproject/A2A](https://github.com/a2aproject/A2A)
- ✅ [Buf](https://buf.build/) integration for proto file management and validation
- ✅ [tonic](https://github.com/hyperium/tonic) for high-performance gRPC client and server
- ✅ Automatic code generation from proto files during build
- ✅ Type-safe conversions between proto and domain types
- ✅ Core service method implementations: `send_message`, `get_task`, `cancel_task`, `list_tasks`, `get_extended_agent_card`
- ⚠️ Additional/advanced service methods and features (in progress)

## Getting Started

### Prerequisites

1. **Rust** (1.85 or later)
2. **Buf CLI** (optional, for linting): https://docs.buf.build/installation
3. **Protocol Buffers compiler** (protoc) - included via tonic-build

### Adding gRPC to Your Project

Add a2a-rs with gRPC features to your `Cargo.toml`:

```toml
[dependencies]
# For gRPC client
a2a-rs = { version = "0.1.0", features = ["grpc-client"] }

# For gRPC server
a2a-rs = { version = "0.1.0", features = ["grpc-server"] }

# For both client and server
a2a-rs = { version = "0.1.0", features = ["grpc-client", "grpc-server"] }
```

## Client Usage

### Basic Client Example

> **Note**: The `GrpcClient` currently provides a basic connection skeleton. Full client-side RPC method implementations are in progress. For now, you can connect to a gRPC server and use the low-level proto client methods directly.

```rust
use a2a_rs::GrpcClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to a gRPC server
    let mut client = GrpcClient::connect("http://localhost:50051").await?;
    
    println!("Connected to gRPC server");
    
    // TODO: Client-side RPC methods (send_message, get_task, etc.) are in progress
    // For now, you can use the underlying tonic client directly via client.inner()
    
    Ok(())
}
```

### Future Client API (In Development)

Once complete, the client will support these operations:

```rust
// Send a message (planned)
// let message = Message::user_text("Hello, agent!".to_string());
// let response = client.send_message(message, context_id).await?;

// Get a task (planned)
// let task = client.get_task("tasks/123", Some(10)).await?;

// Cancel a task (planned)
// let task = client.cancel_task("tasks/123").await?;

// List tasks (planned)
// let response = client.list_tasks(page_size, page_token).await?;
```

## Server Usage

### Basic Server Example

```rust
use a2a_rs::{GrpcServer, SimpleAgentInfo, DefaultRequestProcessor};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging
    tracing_subscriber::fmt::init();
    
    // Create request processor with your business logic
    let processor = DefaultRequestProcessor::new();
    
    // Define agent metadata
    let agent_info = SimpleAgentInfo::new(
        "my-grpc-agent".to_string(),
        "1.0.0".to_string(),
    );
    
    // Create and start the gRPC server
    let addr: SocketAddr = "[::1]:50051".parse()?;
    let server = GrpcServer::new(processor, agent_info, addr);
    
    println!("Starting gRPC server on {}", addr);
    server.start().await?;
    
    Ok(())
}
```

## Protocol Buffer Definitions

The Protocol Buffer definitions are located in `proto/a2a.proto` and are sourced from the official A2A Protocol specification.

### Key Message Types

- **Task** - Core unit of action with status, artifacts, and history
- **Message** - Communication content with parts (text, file, data)
- **AgentCard** - Agent capabilities and metadata
- **TaskStatus** - Current state and status message
- **Artifact** - Output results from task processing

### Service Definition

```protobuf
service A2AService {
  rpc SendMessage(SendMessageRequest) returns (SendMessageResponse);
  rpc SendStreamingMessage(SendMessageRequest) returns (stream StreamResponse);
  rpc GetTask(GetTaskRequest) returns (Task);
  rpc ListTasks(ListTasksRequest) returns (ListTasksResponse);
  rpc CancelTask(CancelTaskRequest) returns (Task);
  rpc SubscribeToTask(SubscribeToTaskRequest) returns (stream StreamResponse);
  rpc SetTaskPushNotificationConfig(...) returns (TaskPushNotificationConfig);
  rpc GetTaskPushNotificationConfig(...) returns (TaskPushNotificationConfig);
  rpc ListTaskPushNotificationConfig(...) returns (...);
  rpc DeleteTaskPushNotificationConfig(...) returns (Empty);
  rpc GetExtendedAgentCard(...) returns (AgentCard);
}
```

## Buf Integration

[Buf](https://buf.build/) is used for proto file management, linting, and validation.

### Linting Proto Files

```bash
# Lint the proto files
cd a2a-rs/proto
buf lint

# Format proto files
buf format -w
```

### Buf Configuration

- `proto/buf.yaml` - Main Buf configuration for linting and breaking change detection
- `proto/buf.gen.yaml` - Code generation configuration (currently unused)

The actual code generation is handled by `tonic-build` in the `build.rs` script, which integrates better with Cargo's build system.

## Architecture

The gRPC implementation follows the same hexagonal architecture as the rest of a2a-rs:

```
┌─────────────────────────────────────┐
│     Application Layer (gRPC)        │
│  ┌────────────────────────────────┐ │
│  │   A2AService Implementation    │ │
│  │  (tonic gRPC server/client)    │ │
│  └────────────────────────────────┘ │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│  Adapter Layer (Type Conversion)    │
│  ┌────────────────────────────────┐ │
│  │  Proto ↔ Domain Converters     │ │
│  └────────────────────────────────┘ │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│         Port Layer                  │
│  ┌────────────────────────────────┐ │
│  │  RequestProcessor              │ │
│  │  MessageHandler                │ │
│  │  TaskManager                   │ │
│  └────────────────────────────────┘ │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│         Domain Layer                │
│  ┌────────────────────────────────┐ │
│  │  Task, Message, AgentCard      │ │
│  │  Business Logic                │ │
│  └────────────────────────────────┘ │
└─────────────────────────────────────┘
```

## Development Status

### Implemented ✅

- [x] Protocol Buffer definitions (official a2a.proto)
- [x] Build system integration with tonic-build
- [x] Buf integration for linting
- [x] gRPC client skeleton
- [x] gRPC server skeleton
- [x] Type conversion stubs

### In Progress 🚧

- [ ] Complete proto↔domain type conversions
- [ ] Implement SendMessage RPC
- [ ] Implement SendStreamingMessage RPC
- [ ] Implement GetTask RPC
- [ ] Implement ListTasks RPC
- [ ] Implement CancelTask RPC
- [ ] Implement SubscribeToTask RPC
- [ ] Implement push notification RPCs
- [ ] Implement GetExtendedAgentCard RPC

### Planned 📋

- [ ] gRPC client/server integration tests
- [ ] gRPC examples
- [ ] Performance benchmarks vs HTTP/JSON-RPC
- [ ] TLS/mTLS support
- [ ] gRPC health checks
- [ ] gRPC reflection support

## Testing

```bash
# Build with gRPC features
cargo build --features grpc-client,grpc-server

# Run tests (when implemented)
cargo test --features grpc-client,grpc-server
```

## Performance Considerations

gRPC offers several advantages over HTTP/JSON-RPC:

- **Binary Protocol** - More efficient serialization with Protocol Buffers
- **HTTP/2** - Multiplexing, header compression, server push
- **Streaming** - Native bidirectional streaming support
- **Type Safety** - Strongly-typed interfaces across languages

## Comparison with Other Transports

| Feature | HTTP/JSON-RPC | WebSocket | gRPC |
|---------|--------------|-----------|------|
| Protocol | JSON | JSON | Protocol Buffers |
| Efficiency | Medium | Medium | High |
| Streaming | No | Yes | Yes (native) |
| Browser Support | Yes | Yes | Limited |
| Type Safety | Medium | Medium | High |
| Tooling | Good | Good | Excellent |
| Status | ✅ Production | ✅ Production | 🚧 In Progress |

## Resources

- [Official A2A Protocol Specification](https://a2a-protocol.org/latest/specification/)
- [A2A gRPC Proto Files](https://github.com/a2aproject/A2A/tree/main/specification/grpc)
- [tonic Documentation](https://docs.rs/tonic/)
- [Buf Documentation](https://docs.buf.build/)
- [Protocol Buffers Guide](https://protobuf.dev/)

## Contributing

Contributions to complete the gRPC implementation are welcome! See the "In Progress" section above for areas that need work.

1. Fork the repository
2. Create a feature branch
3. Implement missing functionality
4. Add tests
5. Submit a pull request

## License

The gRPC implementation follows the same MIT license as the rest of a2a-rs.
