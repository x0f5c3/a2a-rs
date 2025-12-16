# gRPC Implementation TODO

## Current Status

The gRPC implementation skeleton has been set up with:

✅ Official a2a.proto file from A2A Protocol v0.3.0
✅ Buf integration for proto linting
✅ tonic-build integration for code generation
✅ Module structure (client, server, convert)
✅ Build system configuration
✅ Documentation (GRPC.md)
✅ Test skeleton (grpc_integration.rs)

## Remaining Work

### 1. Proto Field Mapping

The generated Protocol Buffer types from `a2a.proto` have different field names/structures than initially assumed. Need to:

- [ ] Review generated proto types in target/debug/build/a2a-rs-*/out/
- [ ] Update all proto struct field references to match generated code
- [ ] Complete the conversion functions in `convert.rs` with correct field mappings

### 2. Type Conversions

Complete implementations in `src/adapter/grpc/convert.rs`:

- [ ] `to_proto_message` - Convert domain Message to proto Message
- [ ] `from_proto_message` - Convert proto Message to domain Message  
- [ ] `to_proto_task` - Convert domain Task to proto Task
- [ ] `from_proto_task` - Convert proto Task to domain Task
- [ ] `to_proto_part` - Convert domain Part to proto Part
- [ ] `from_proto_part` - Convert proto Part to domain Part
- [ ] `to_proto_artifact` - Convert domain Artifact to proto Artifact
- [ ] `from_proto_artifact` - Convert proto Artifact to domain Artifact
- [ ] Complete `to_proto_agent_card` with all fields
- [ ] Complete `from_proto_agent_card`

### 3. Client Implementation

In `src/adapter/grpc/client.rs`:

- [ ] Implement AsyncA2AClient trait methods
- [ ] Or create gRPC-specific client interface if trait doesn't fit
- [ ] Handle request/response conversions
- [ ] Implement error mapping from tonic errors to A2AError

### 4. Server Implementation

In `src/adapter/grpc/server.rs`:

- [ ] Implement all RPC methods in A2aService trait:
  - [ ] `send_message`
  - [ ] `send_streaming_message`  
  - [ ] `get_task`
  - [ ] `list_tasks`
  - [ ] `cancel_task`
  - [ ] `subscribe_to_task`
  - [ ] `set_task_push_notification_config`
  - [ ] `get_task_push_notification_config`
  - [ ] `list_task_push_notification_config`
  - [ ] `delete_task_push_notification_config`
  - [x] `get_extended_agent_card` (partially done)

### 5. Streaming Support

- [ ] Implement server-side streaming for `send_streaming_message`
- [ ] Implement server-side streaming for `subscribe_to_task`
- [ ] Handle stream lifecycle and error handling

### 6. Testing

- [ ] Complete integration tests in `tests/grpc_integration.rs`
- [ ] Add unit tests for all conversion functions
- [ ] Test streaming functionality
- [ ] Test error scenarios

### 7. Build Issues

Current build errors to resolve:

```
error[E0560]: struct `proto::*` has no field named `*`
```

This is because the generated proto structs have different field names than expected. 

**Solution**: After running `cargo build`, inspect the generated file at:
```
target/debug/build/a2a-rs-*/out/a2a.v1.rs
```

Then update all field references in client.rs, server.rs, and convert.rs to match the actual generated struct fields.

## Development Steps

1. Run `cargo build --features grpc-server,grpc-client` to generate proto code
2. Find generated code in `target/debug/build/a2a-rs-*/out/`
3. Review proto struct definitions
4. Update convert.rs with correct field mappings
5. Update client.rs and server.rs to use correct proto fields
6. Implement missing conversion functions
7. Test and iterate

## Notes

- Proto file is correctly set up and validated with Buf
- Build system correctly generates code with tonic-build
- Architecture and module structure are sound
- Main blocker is completing the proto↔domain type mappings

## Help Needed

Completing this implementation requires:
1. Deep inspection of generated proto types
2. Careful mapping between A2A domain types and proto types
3. Testing against a real gRPC server/client pair

Estimated effort: 4-8 hours of focused development
