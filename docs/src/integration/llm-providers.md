# LLM Provider Integration

Integrate a2a-rs with different LLM providers for intelligent agent functionality.

## Overview

The `AsyncMessageHandler` trait is the key integration point for LLMs. It processes incoming messages and generates responses using your LLM of choice.

## Architecture

```
User Message → gRPC/HTTP Server → AsyncMessageHandler
                                         ↓
                                    LLM Provider
                                    (OpenAI, Anthropic, Local)
                                         ↓
                                    Response Message
                                         ↓
                                    Task Update → User
```

## Generic LLM Handler

Here's a template for any LLM provider:

```rust
use a2a_rs::{AsyncMessageHandler, AsyncTaskManager};
use async_trait::async_trait;

pub struct LLMMessageHandler<T> {
    task_storage: Arc<T>,
    llm_client: Arc<dyn LLMClient>,
}

#[async_trait]
pub trait LLMClient: Send + Sync {
    async fn generate(&self, prompt: &str, context: &[Message]) -> Result<String>;
}

#[async_trait]
impl<T: AsyncTaskManager> AsyncMessageHandler for LLMMessageHandler<T> {
    async fn process_message(
        &self,
        task_id: &str,
        message: &Message,
        session_id: Option<&str>,
    ) -> Result<Task> {
        // 1. Get task for context
        let task = self.task_storage.get_task(task_id, None).await?;
        
        // 2. Build prompt from message
        let prompt = self.build_prompt(message, &task)?;
        
        // 3. Call LLM
        let response_text = self.llm_client.generate(&prompt, &task.history).await?;
        
        // 4. Create response message
        let response = Message {
            message_id: format!("resp-{}", uuid::Uuid::new_v4()),
            role: Role::Agent,
            parts: vec![Part::Text {
                text: response_text,
                metadata: None,
            }],
            task_id: Some(task_id.to_string()),
            context_id: session_id.map(|s| s.to_string()),
            // ...
        };
        
        // 5. Update task with response
        self.task_storage.update_task_status(
            task_id,
            TaskState::Completed,
            Some(response),
        ).await
    }
}
```

## OpenAI Integration

```rust
use async_openai::{Client, types::{ChatCompletionRequestMessage, CreateChatCompletionRequest}};

pub struct OpenAIClient {
    client: Client,
    model: String,
}

#[async_trait]
impl LLMClient for OpenAIClient {
    async fn generate(&self, prompt: &str, context: &[Message]) -> Result<String> {
        let messages = self.build_openai_messages(prompt, context);
        
        let request = CreateChatCompletionRequest {
            model: self.model.clone(),
            messages,
            ..Default::default()
        };
        
        let response = self.client
            .chat()
            .create(request)
            .await?;
        
        Ok(response.choices[0].message.content.clone())
    }
}

impl OpenAIClient {
    fn build_openai_messages(&self, prompt: &str, context: &[Message]) -> Vec<ChatCompletionRequestMessage> {
        let mut messages = vec![];
        
        // Add context from history
        for msg in context {
            messages.push(ChatCompletionRequestMessage {
                role: match msg.role {
                    Role::User => "user",
                    Role::Agent => "assistant",
                }.into(),
                content: self.extract_text(msg),
                ..Default::default()
            });
        }
        
        // Add current prompt
        messages.push(ChatCompletionRequestMessage {
            role: "user".into(),
            content: prompt.into(),
            ..Default::default()
        });
        
        messages
    }
}
```

## Anthropic Integration

```rust
use anthropic::{Client, types::{Message as AnthropicMessage, Role as AnthropicRole}};

pub struct AnthropicClient {
    client: Client,
    model: String,
}

#[async_trait]
impl LLMClient for AnthropicClient {
    async fn generate(&self, prompt: &str, context: &[Message]) -> Result<String> {
        let messages = self.build_anthropic_messages(prompt, context);
        
        let response = self.client
            .messages()
            .create(messages, &self.model)
            .await?;
        
        Ok(response.content[0].text.clone())
    }
}
```

## Local Model Integration (Ollama)

```rust
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};

pub struct OllamaClient {
    client: Ollama,
    model: String,
}

#[async_trait]
impl LLMClient for OllamaClient {
    async fn generate(&self, prompt: &str, _context: &[Message]) -> Result<String> {
        let request = GenerationRequest::new(self.model.clone(), prompt.into());
        
        let response = self.client
            .generate(request)
            .await?;
        
        Ok(response.response)
    }
}
```

## Usage Example

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create storage
    let task_storage = InMemoryTaskStorage::new();
    
    // Create LLM client
    let llm_client = Arc::new(OpenAIClient::new(
        std::env::var("OPENAI_API_KEY")?,
        "gpt-4".into(),
    ));
    
    // Create message handler
    let message_handler = LLMMessageHandler::new(
        task_storage.clone(),
        llm_client,
    );
    
    // Create agent info
    let agent_info = SimpleAgentInfo::new(
        "openai-agent".into(),
        "1.0.0".into(),
    );
    
    // Create gRPC server
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

## Streaming Responses

For streaming LLM responses:

```rust
pub async fn process_message_streaming(
    &self,
    task_id: &str,
    message: &Message,
) -> Result<impl Stream<Item = Result<String>>> {
    let stream = self.llm_client.generate_stream(prompt).await?;
    
    // Convert LLM stream to task updates
    let task_stream = stream.map(|chunk| {
        // Update task with chunk
        // Return progress update
    });
    
    Ok(task_stream)
}
```

## Error Handling

Handle LLM-specific errors:

```rust
impl From<OpenAIError> for A2AError {
    fn from(err: OpenAIError) -> Self {
        match err {
            OpenAIError::RateLimitExceeded => A2AError::RateLimitExceeded,
            OpenAIError::InvalidApiKey => A2AError::AuthenticationFailed,
            _ => A2AError::Internal(format!("LLM error: {}", err)),
        }
    }
}
```

## Configuration

```toml
[dependencies]
a2a-rs = { git = "https://github.com/x0f5c3/a2a-rs", features = ["grpc-server"] }
async-openai = "0.20"  # For OpenAI
anthropic = "0.1"       # For Anthropic
ollama-rs = "0.1"       # For Ollama
```

## Best Practices

1. **Context Management**: Keep message history within token limits
2. **Error Recovery**: Retry with exponential backoff
3. **Rate Limiting**: Implement rate limiting for API calls
4. **Timeouts**: Set appropriate timeouts for LLM calls
5. **Logging**: Log all LLM interactions for debugging

## Kowalski Integration Example

For projects like Kowalski that use multiple coordination methods:

```rust
pub struct KowalskiMessageHandler {
    task_storage: Arc<InMemoryTaskStorage>,
    coordinator: Arc<dyn Coordinator>,
    llm_router: Arc<LLMRouter>,
}

impl KowalskiMessageHandler {
    async fn route_to_llm(&self, message: &Message) -> Result<Arc<dyn LLMClient>> {
        // Route based on message content, user preferences, etc.
        self.llm_router.select(message).await
    }
}
```

## Next Steps

- [Custom Message Handlers](./custom-message-handlers.md)
- [Coordination Methods](./coordination.md)
- [Error Handling](../advanced/errors.md)
