# gRPC Server

The gRPC server provides high-performance binary protocol communication using Protocol Buffers and the official A2A Protocol v0.3.0 proto definitions.

## Features

✅ **Production-Ready Core Methods:**
- `send_message` - Send messages to create/update tasks
- `get_task` - Retrieve task details with history
- `cancel_task` - Cancel running tasks
- `list_tasks` - List tasks with pagination
- `get_extended_agent_card` - Get agent metadata

⏸️ **Optional Methods (TODO):**
- Streaming RPCs (2 methods)
- Push notification management (4 methods)

## Quick Start

```rust
use a2a_rs::{
    GrpcServer, InMemoryTaskStorage, DefaultMessageHandler, SimpleAgentInfo
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let task_storage = InMemoryTaskStorage::new();
    let message_handler = DefaultMessageHandler::new(task_storage.clone());
    let agent_info = SimpleAgentInfo::new("my-agent".into(), "1.0.0".into());
    
    let server = GrpcServer::new(
        task_storage,
        message_handler,
        agent_info,
        "[::1]:50051".parse()?,
    );
    
    server.start().await?;
    Ok(())
}
```

## Architecture

The gRPC server uses **direct manager access** (Option A) for optimal performance:

```rust
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
```

This design:
- Eliminates JSON-RPC overhead
- Provides type-safe compilation
- Enables easy testing
- Supports custom implementations

## Features Flag

Enable gRPC support in your `Cargo.toml`:

```toml
[dependencies]
a2a-rs = { git = "https://github.com/x0f5c3/a2a-rs", features = ["grpc-server"] }
tonic = "0.12"
prost = "0.13"
```

## Proto Infrastructure

Uses official proto definitions with Buf for validation:

- Official `a2a.proto` from A2A Protocol v0.3.0
- Buf integration for linting
- `tonic-build` for code generation
- Google API proto dependencies

## RPC Methods

### send_message

Send a message to create or update a task:

```rust
// Proto request
SendMessageRequest {
    request: Some(Message { ... }),
    configuration: None,
    metadata: None,
}

// Returns task with updated state
SendMessageResponse {
    payload: Some(Payload::Task(task))
}
```

### get_task

Retrieve task by ID:

```rust
GetTaskRequest {
    name: "tasks/task-123",
    history_length: Some(10),
}

// Returns full task details
Task { ... }
```

### cancel_task

Cancel a running task:

```rust
CancelTaskRequest {
    name: "tasks/task-123",
}

// Returns updated task with Canceled state
Task { state: Cancelled, ... }
```

### list_tasks

List tasks with pagination:

```rust
ListTasksRequest {
    page_size: Some(50),
    page_token: "",
}

ListTasksResponse {
    tasks: vec![...],
    next_page_token: "",
    page_size: 50,
    total_size: 100,
}
```

### get_extended_agent_card

Get agent capabilities:

```rust
GetExtendedAgentCardRequest {}

// Returns agent metadata
AgentCard {
    name: "my-agent",
    version: "1.0.0",
    capabilities: [...],
}
```

## Type Conversions

All proto↔domain conversions are implemented:

- ✅ Message conversions (with parts)
- ✅ Task conversions (with status)
- ✅ TaskState enum (all 9 states)
- ✅ Part conversions (Text, File)
- ⚠️ Data part (prost Struct) - pending

## Error Handling

Proper gRPC status codes:

```rust
Status::not_found("Task not found")
Status::failed_precondition("Task not cancelable")
Status::invalid_argument("Invalid task name format")
Status::internal("Internal server error")
```

## Session Context

Context IDs are properly passed as session IDs:

```rust
// context_id from message → session_id for processing
let session_id = message.context_id.as_deref();
self.message_handler.process_message(task_id, &message, session_id).await
```

## Custom Implementations

Integrate with your own backends:

```rust
struct MyTaskManager { /* ... */ }

#[async_trait]
impl AsyncTaskManager for MyTaskManager {
    async fn create_task(&self, task_id: &str, context_id: &str) -> Result<Task> {
        // Your implementation
    }
    // ... other methods
}

let server = GrpcServer::new(
    MyTaskManager::new(),
    MyMessageHandler::new(),
    MyAgentInfo::new(),
    addr,
);
```

## Testing

```bash
# Build with gRPC features
cargo build --features grpc-server

# Run tests
cargo test --features grpc-server

# Test with grpcurl
grpcurl -plaintext localhost:50051 a2a.v1.A2AService/GetExtendedAgentCard
```

## Performance

Benefits of gRPC:
- Binary protocol (smaller payloads)
- HTTP/2 multiplexing
- Streaming support
- Code generation
- Cross-language compatibility

## Next Steps

- See [Custom Message Handlers](../integration/custom-message-handlers.md)
- Learn about [LLM Integration](../integration/llm-providers.md)
- Explore [gRPC complete documentation](../../a2a-rs/GRPC_COMPLETE.md)
