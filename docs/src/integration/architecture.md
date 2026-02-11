# Architecture Overview

Understanding the a2a-rs architecture for successful integration.

## Layered Architecture

```
┌─────────────────────────────────────────┐
│         Transport Layer                  │
│  (HTTP, WebSocket, gRPC Servers)        │
└─────────────────────────────────────────┘
                  │
┌─────────────────────────────────────────┐
│         Service Layer                    │
│  (Request Processing, Validation)       │
└─────────────────────────────────────────┘
                  │
┌─────────────────────────────────────────┐
│         Port Layer (Traits)              │
│  (AsyncTaskManager, AsyncMessageHandler) │
└─────────────────────────────────────────┘
                  │
┌─────────────────────────────────────────┐
│         Domain Layer                     │
│  (Message, Task, TaskState, etc.)       │
└─────────────────────────────────────────┘
                  │
┌─────────────────────────────────────────┐
│         Adapter Layer                    │
│  (Storage, Auth, Business Logic)        │
└─────────────────────────────────────────┘
```

## Core Components

### 1. Domain Layer

Pure business logic with no dependencies:

- `Message` - Communication units
- `Task` - Work items
- `TaskState` - State machine
- `AgentCard` - Agent metadata
- `Part` - Message content types

### 2. Port Layer (Traits)

Defines interfaces for extensibility:

```rust
pub trait AsyncTaskManager {
    async fn create_task(&self, task_id: &str, context_id: &str) -> Result<Task>;
    async fn get_task(&self, task_id: &str, history_length: Option<u32>) -> Result<Task>;
    async fn cancel_task(&self, task_id: &str) -> Result<Task>;
    async fn list_tasks(&self, context_id: Option<&str>, limit: Option<u32>) -> Result<Vec<Task>>;
}

pub trait AsyncMessageHandler {
    async fn process_message(
        &self,
        task_id: &str,
        message: &Message,
        session_id: Option<&str>,
    ) -> Result<Task>;
}

pub trait AgentInfoProvider {
    async fn get_agent_card(&self) -> Result<AgentCard>;
}
```

### 3. Adapter Layer

Concrete implementations:

- **Storage**: `InMemoryTaskStorage`, `SqliteTaskStorage`, `PostgresTaskStorage`
- **Auth**: `ApiKeyAuthenticator`, `JwtAuthenticator`, etc.
- **Business**: `DefaultMessageHandler`, `DefaultRequestProcessor`

### 4. Transport Layer

Protocol-specific servers:

- **HTTP Server**: JSON-RPC over HTTP
- **WebSocket Server**: Bidirectional streaming
- **gRPC Server**: Protocol Buffers binary protocol

## Integration Points

### For Custom Task Storage

Implement `AsyncTaskManager`:

```rust
struct MyTaskStorage { /* ... */ }

#[async_trait]
impl AsyncTaskManager for MyTaskStorage {
    async fn create_task(&self, task_id: &str, context_id: &str) -> Result<Task> {
        // Your database/storage logic
    }
    // ... implement other methods
}
```

### For Custom Message Processing

Implement `AsyncMessageHandler`:

```rust
struct MyLLMHandler {
    llm_client: LLMClient,
    task_storage: Arc<dyn AsyncTaskManager>,
}

#[async_trait]
impl AsyncMessageHandler for MyLLMHandler {
    async fn process_message(
        &self,
        task_id: &str,
        message: &Message,
        session_id: Option<&str>,
    ) -> Result<Task> {
        // 1. Call your LLM
        let response = self.llm_client.generate(message).await?;
        
        // 2. Update task with response
        let response_msg = Message { /* ... */ };
        self.task_storage.update_task_status(
            task_id,
            TaskState::Completed,
            Some(response_msg),
        ).await
    }
}
```

### For Custom Agent Metadata

Implement `AgentInfoProvider`:

```rust
struct MyAgentInfo { /* ... */ }

#[async_trait]
impl AgentInfoProvider for MyAgentInfo {
    async fn get_agent_card(&self) -> Result<AgentCard> {
        Ok(AgentCard {
            name: "my-agent".into(),
            version: "1.0.0".into(),
            capabilities: vec![/* ... */],
            // ...
        })
    }
}
```

## Dependency Injection

All components use constructor injection:

```rust
// Create your components
let task_storage = MyTaskStorage::new();
let message_handler = MyLLMHandler::new(task_storage.clone());
let agent_info = MyAgentInfo::new();

// Inject into server
let server = GrpcServer::new(
    task_storage,
    message_handler,
    agent_info,
    addr,
);
```

## Async/Await Throughout

All interfaces are async for non-blocking I/O:

```rust
#[async_trait]
pub trait AsyncTaskManager: Send + Sync {
    async fn get_task<'a>(&self, task_id: &'a str, ...) -> Result<Task>;
}
```

## Error Handling

Custom error type `A2AError` with variants:

```rust
pub enum A2AError {
    TaskNotFound(String),
    TaskNotCancelable(String),
    ValidationError { field: String, message: String },
    InvalidRequest(String),
    Internal(String),
    // ...
}
```

## Thread Safety

All managers must be `Send + Sync`:

```rust
pub trait AsyncTaskManager: Send + Sync {
    // Methods can be called from multiple threads
}
```

## Testing Strategy

1. **Unit Tests**: Test domain logic
2. **Integration Tests**: Test with in-memory storage
3. **E2E Tests**: Test full server stack

```rust
#[tokio::test]
async fn test_custom_handler() {
    let storage = InMemoryTaskStorage::new();
    let handler = MyHandler::new(storage.clone());
    
    let task = handler.process_message("task-1", &message, None).await?;
    assert_eq!(task.status.state, TaskState::Completed);
}
```

## Next Steps

- [Custom Task Managers](./custom-task-managers.md)
- [Custom Message Handlers](./custom-message-handlers.md)
- [LLM Provider Integration](./llm-providers.md)
