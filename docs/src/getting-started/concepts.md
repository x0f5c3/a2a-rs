# Basic Concepts

Understanding the core concepts of A2A Protocol and how they're implemented in a2a-rs.

## Tasks

**Tasks** are the fundamental unit of work in A2A. Each task:

- Has a unique ID
- Tracks state (Created, InProgress, Completed, Failed, Canceled)
- Contains a message history
- Can have artifacts (results, files, etc.)
- Belongs to a context (session/conversation)

```rust
use a2a_rs::domain::core::{Task, TaskState};

// Tasks are managed by AsyncTaskManager trait
pub trait AsyncTaskManager {
    async fn create_task(&self, task_id: &str, context_id: &str) -> Result<Task>;
    async fn get_task(&self, task_id: &str, history_length: Option<u32>) -> Result<Task>;
    async fn cancel_task(&self, task_id: &str) -> Result<Task>;
    async fn list_tasks(&self, context_id: Option<&str>, limit: Option<u32>) -> Result<Vec<Task>>;
}
```

## Messages

**Messages** are exchanged between agents and contain the actual content:

- Parts (text, files, or structured data)
- Role (User or Agent)
- Context and task references
- Metadata

```rust
use a2a_rs::domain::core::{Message, Part, Role};

let message = Message {
    message_id: "msg-123".into(),
    role: Role::User,
    parts: vec![Part::Text {
        text: "Hello, agent!".into(),
        metadata: None,
    }],
    task_id: Some("task-456".into()),
    context_id: Some("ctx-789".into()),
    // ...
};
```

## Managers and Handlers

The architecture uses traits for flexibility:

### AsyncTaskManager

Handles task lifecycle operations:

```rust
impl AsyncTaskManager for InMemoryTaskStorage {
    async fn create_task(&self, task_id: &str, context_id: &str) -> Result<Task> {
        // Your implementation
    }
    // ... other methods
}
```

### AsyncMessageHandler

Processes incoming messages:

```rust
impl AsyncMessageHandler for DefaultMessageHandler {
    async fn process_message(
        &self,
        task_id: &str,
        message: &Message,
        session_id: Option<&str>,
    ) -> Result<Task> {
        // Your implementation
    }
}
```

### AgentInfoProvider

Provides agent metadata:

```rust
impl AgentInfoProvider for SimpleAgentInfo {
    async fn get_agent_card(&self) -> Result<AgentCard> {
        // Return agent capabilities
    }
}
```

## Transports

Three transport protocols are supported:

1. **HTTP** - RESTful JSON-RPC API
2. **WebSocket** - Bidirectional streaming
3. **gRPC** - High-performance binary protocol

Each transport wraps the same core managers and handlers, providing protocol-specific implementation.

## Protocol Flow

1. **Client sends message** → Server receives via transport
2. **Server creates/updates task** → TaskManager handles state
3. **Message is processed** → MessageHandler executes logic
4. **Task state updated** → Response sent back
5. **Client polls/streams** → Gets updates on task progress

## Storage Options

- `InMemoryTaskStorage` - For development and testing
- `SqliteTaskStorage` - SQLite persistence
- `PostgresTaskStorage` - PostgreSQL for production

## Authentication

Multiple authentication methods supported:

- API Key
- Bearer Token
- JWT (JSON Web Tokens)
- OAuth2
- OpenID Connect

## Next Steps

- See [gRPC Server](../features/grpc-server.md) for gRPC-specific features
- Learn about [Custom Task Managers](../integration/custom-task-managers.md)
- Explore [LLM Integration](../integration/llm-providers.md)
