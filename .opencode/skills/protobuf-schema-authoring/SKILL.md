---
name: protobuf-schema-authoring
description: Author proto2/proto3 schemas and provide protoc generation workflows for Rust and TypeScript.
---

## What I do

- Turn user requirements into clear, evolvable `.proto` schemas.
- Support both `proto2` and `proto3` syntax.
- Ask for syntax preference when not explicitly provided, and explain version-specific constraints.
- Provide practical `protoc` workflows for Rust and TypeScript.
- Use the full type/layout reference in `docs/readmes/protobuf-schema-research.md`.

## When to use me

Use this when the task includes any of the following:

- Create a new protobuf schema from product or API requirements.
- Update an existing `.proto` safely.
- Explain if a feature is `proto2`-only or `proto3`-only.
- Generate Rust or TypeScript artifacts from `.proto` files.

## Required clarification flow

If not already specified, ask these in order:

1. Which schema syntax should be used: `proto2` or `proto3`?
   - Recommended default: `proto3` with explicit `optional` on scalar fields when presence matters.
2. For TypeScript generation, which plugin should be targeted: `ts-proto` or `@protobuf-ts/plugin`?
   - Recommended default: `ts-proto`.

When answering, always note version-specific features involved in the request.

## Proto2 vs proto3 guardrails

- `required` fields are proto2-only and should be avoided for new design.
- Explicit scalar defaults (`[default = ...]`) are proto2-only.
- Extensions are proto2-oriented; not a general proto3 field pattern.
- Groups are deprecated legacy proto2 and should not be introduced.
- Proto3 enums must start with a zero-valued first entry.
- Proto3 scalar presence should use `optional` when presence semantics are needed.
- Repeated scalar numerics are packed-by-default in proto3; in proto2, use `[packed = true]` for new fields.

## Complete valid type reference (inline)

### Scalar field types

- `double`
- `float`
- `int32`
- `int64`
- `uint32`
- `uint64`
- `sint32`
- `sint64`
- `fixed32`
- `fixed64`
- `sfixed32`
- `sfixed64`
- `bool`
- `string`
- `bytes`

### Non-scalar forms

- Message type fields (local or imported), for example `my.pkg.User`.
- Enum type fields (local or imported).
- `map<K, V>` fields.
  - Valid map key types: `int32`, `int64`, `uint32`, `uint64`, `sint32`, `sint64`, `fixed32`, `fixed64`, `sfixed32`, `sfixed64`, `bool`, `string`.
  - Map value types: scalar, enum, or message.
- `oneof` blocks.

## Complete schema layout reference (inline)

### Top-level statements

- `syntax = "proto2";` or `syntax = "proto3";` (first non-comment line)
- `package ...;`
- `import "...";`, `import public "...";`, `import weak "...";`
- `option ... = ...;`
- `message ... { ... }`
- `enum ... { ... }`
- `service ... { ... }`
- `extend ... { ... }`

### Message body statements

- Field declarations (`optional`/`required`/`repeated` in proto2; `optional`/implicit/repeated in proto3)
- `oneof ... { ... }`
- `map<K, V> ... = N;`
- Nested `message` and nested `enum`
- `reserved ...;`
- `option ... = ...;`
- `extensions ...;` (proto2)
- `group` (deprecated legacy proto2)
- `extend ... { ... }`

### Enum body statements

- Enum values: `NAME = N;`
- Enum options (for example `option allow_alias = true;`)
- `reserved ...;`

### Service body statements

- `rpc Method(Request) returns (Response);`
- `rpc Stream(stream Request) returns (stream Response);`
- Method options blocks on RPC definitions.

## Boilerplate stubs (inline)

### Proto3 starter

```proto
syntax = "proto3";

package example.v1;

import "google/protobuf/timestamp.proto";

message Resource {
  optional string display_name = 1;
  string id = 2;
  google.protobuf.Timestamp created_at = 3;
  map<string, string> labels = 4;

  oneof owner {
    string user_id = 5;
    string team_id = 6;
  }

  reserved 7;
  reserved "legacy_id";
}

enum ResourceState {
  RESOURCE_STATE_UNSPECIFIED = 0;
  RESOURCE_STATE_ACTIVE = 1;
}

service ResourceService {
  rpc GetResource(GetResourceRequest) returns (GetResourceResponse);
}

message GetResourceRequest {
  string id = 1;
}

message GetResourceResponse {
  Resource resource = 1;
}
```

### Proto2 starter

```proto
syntax = "proto2";

package example.v1;

message Resource {
  optional string display_name = 1 [default = "unknown"];
  optional string id = 2;
  repeated string tags = 3;

  extensions 100 to max;

  reserved 4;
  reserved "legacy_id";
}

enum ResourceState {
  RESOURCE_STATE_UNSPECIFIED = 0;
  RESOURCE_STATE_ACTIVE = 1;
}

extend Resource {
  optional string external_profile = 100;
}
```

## Complete reference location

Use `docs/readmes/protobuf-schema-research.md` as the canonical in-repo reference for:

- Complete valid scalar type list and non-scalar type forms.
- Full schema definition layout (top-level, message, enum, service).
- Proto2 and proto3 boilerplate templates.
- Wire constraints and field numbering guardrails.

When drafting schemas, follow that reference and include only the subset needed for the user request.

## Schema authoring workflow

1. Confirm syntax version and target language outputs.
2. Propose package and file layout.
3. Define messages/enums/services with stable field numbering.
4. Add comments only where semantics are non-obvious.
5. Reserve removed field numbers and names.
6. Validate with `protoc` descriptor output when possible.
7. Provide generation commands for Rust and TypeScript.

## Command templates

- Validate schema and emit descriptor:

```bash
protoc -I proto --include_imports --descriptor_set_out=gen/schema.pb proto/path/to/file.proto
```

- Rust via `prost-build` (`build.rs`):

```rust
fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["proto/service.proto"], &["proto"])?;
    Ok(())
}
```

- Rust via `tonic-prost-build` (`build.rs`):

```rust
fn main() {
    tonic_prost_build::configure()
        .compile_protos(&["proto/service.proto"], &["proto"])
        .unwrap();
}
```

- TypeScript via `ts-proto`:

```bash
protoc \
  --plugin=./node_modules/.bin/protoc-gen-ts_proto \
  --ts_proto_out=gen/ts \
  --ts_proto_opt=esModuleInterop=true \
  -I proto \
  proto/path/to/file.proto
```

- TypeScript via `@protobuf-ts/plugin`:

```bash
npx protoc --ts_out gen/ts --proto_path proto proto/path/to/file.proto
```

## Output expectations

- Deliver the requested `.proto` file(s) and a short note on compatibility choices.
- Explicitly call out any use of proto2-only/proto3-only features.
- Include exact generation commands for the selected Rust and TypeScript toolchains.
