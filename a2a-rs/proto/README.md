# A2A Protocol Buffer Definitions

This directory contains the Protocol Buffer definitions for the A2A (Agent-to-Agent) Protocol gRPC implementation.

## Files

- `a2a.proto` - Official A2A Protocol v0.3.0 gRPC specification
- `buf.yaml` - Buf configuration for linting and breaking change detection
- `buf.gen.yaml` - Buf code generation configuration (currently unused, using tonic-build instead)
- `google/` - Google API proto dependencies required by a2a.proto

## Buf Integration

This project uses [Buf](https://docs.buf.build/) to:
- **Lint** proto files to ensure they follow best practices
- **Validate** breaking changes between proto file versions
- **Format** proto files consistently

### Using Buf

```bash
# Lint proto files
buf lint proto

# Format proto files
buf format proto -w

# Check for breaking changes
buf breaking proto --against '.git#branch=main'
```

## Code Generation

The Rust code is generated using `tonic-build` during the Cargo build process (see `build.rs`).
Buf is used for validation and linting only.

The generated code will be placed in `src/adapter/grpc/generated/`.

## Proto File Source

The `a2a.proto` file is sourced from the official A2A Protocol specification:
https://github.com/a2aproject/A2A/blob/main/specification/grpc/a2a.proto

## Dependencies

The proto file requires these Google API dependencies:
- google/api/annotations.proto
- google/api/client.proto  
- google/api/field_behavior.proto
- google/protobuf/empty.proto
- google/protobuf/struct.proto
- google/protobuf/timestamp.proto

These are included in the `google/` subdirectory.
