# Protobuf Schema Research Notes

Last reviewed: 2026-02-15

## Scope

- Capture practical notes for authoring new `.proto` schemas.
- Support both `proto2` and `proto3` workflows.
- Document `protoc` usage for Rust and TypeScript generation paths.

## Primary web sources

- OpenCode skills docs: https://opencode.ai/docs/skills/
- Protoc install docs: https://protobuf.dev/installation/
- Proto2 guide: https://protobuf.dev/programming-guides/proto2/
- Proto3 guide: https://protobuf.dev/programming-guides/proto3/
- Field presence note: https://protobuf.dev/programming-guides/field_presence/
- Rust generated code guide: https://protobuf.dev/reference/rust/rust-generated/
- Rust build reference: https://protobuf.dev/reference/rust/building-rust-protos/
- `prost-build` docs: https://docs.rs/prost-build/latest/prost_build/
- `tonic-prost-build` docs: https://docs.rs/tonic-prost-build/latest/tonic_prost_build/
- `ts-proto` README: https://github.com/stephenh/ts-proto
- `protobuf-ts` README: https://github.com/timostamm/protobuf-ts

## Proto2 vs proto3 (what matters most)

- **Syntax line**
  - `proto2`: `syntax = "proto2";`
  - `proto3`: `syntax = "proto3";`
- **Cardinality labels**
  - `proto2`: `optional`, `required` (strongly discouraged), `repeated`, `map`.
  - `proto3`: `optional` (recommended), implicit singular (legacy style), `repeated`, `map`.
- **Required fields**
  - `proto2` only, but strongly discouraged for new schemas.
  - Not available in `proto3`.
- **Scalar default overrides**
  - `proto2`: supports `[default = ...]` on singular non-message fields.
  - `proto3`: no custom scalar defaults; language defaults apply.
- **Field presence**
  - `proto2`: explicit presence for singular fields.
  - `proto3`: explicit presence only with `optional` (or oneof/message); otherwise implicit for basic scalars.
  - Proto docs recommend using `optional` in proto3 for smoother compatibility.
- **Enum zero value**
  - `proto3`: first enum value must be `0`.
  - `proto2`: first value is default; using `_UNSPECIFIED = 0` is still safest.
- **Extensions and groups**
  - `proto2`: supports extensions; groups exist but are deprecated.
  - `proto3`: no general extension fields/groups in the same way.
- **Repeated scalar packing**
  - `proto2`: use `[packed = true]` for new repeated numeric fields.
  - `proto3`: repeated scalar numerics are packed by default.
- **Unknown fields**
  - Preserved in both proto2 and proto3 (binary format).
- **Interop note**
  - Proto2 and proto3 messages can interoperate across imports, but proto2 enums cannot be used directly in proto3 syntax.

## Complete valid type list

### Scalar field types (all proto syntaxes)

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

### Non-scalar field type forms

- Message type (local or imported), for example `my.pkg.User`.
- Enum type (local or imported), for example `Status`.
- `map<K, V>` field form.
  - Valid map key types: `int32`, `int64`, `uint32`, `uint64`, `sint32`, `sint64`, `fixed32`, `fixed64`, `sfixed32`, `sfixed64`, `bool`, `string`.
  - Map values can be scalar, enum, or message types.
- `oneof` members (each member is a normal singular field declaration).

### Well-known imported message types (common in schemas)

- `google.protobuf.Any`
- `google.protobuf.Timestamp`
- `google.protobuf.Duration`
- `google.protobuf.Empty`
- `google.protobuf.FieldMask`
- `google.protobuf.Struct`
- Wrapper messages in `google/protobuf/wrappers.proto`

## Complete schema definition layout (reference)

The grammar allows flexible ordering in many places, but this layout is the safest boilerplate for new files.

### Top-level statements

- `syntax = "proto2";` or `syntax = "proto3";` (first non-comment line)
- `package ...;` (optional, recommended)
- `import "...";`, `import public "...";`, or `import weak "...";`
- `option ... = ...;`
- `message ... { ... }`
- `enum ... { ... }`
- `service ... { ... }`
- `extend ... { ... }` (proto2 and custom-option patterns)

### Message body statements

- Field declarations (`optional`/`required`/`repeated` in proto2, `optional`/implicit/repeated in proto3)
- `oneof ... { ... }`
- `map<key, value> ... = N;`
- Nested `message` and nested `enum`
- `reserved ...;` (numbers and/or names)
- `option ... = ...;`
- `extensions ...;` (proto2)
- `group` (proto2 legacy, deprecated)
- `extend ... { ... }`

### Enum body statements

- Enum values: `NAME = N;`
- `option allow_alias = true;` and other enum options
- `reserved ...;` (numbers and/or names)

### Service body statements

- Unary RPC:

```proto
rpc GetThing(GetThingRequest) returns (GetThingResponse);
```

- Streaming RPC:

```proto
rpc Chat(stream ChatMessage) returns (stream ChatMessage);
```

- RPC with method options block:

```proto
rpc GetThing(GetThingRequest) returns (GetThingResponse) {
  option deprecated = true;
}
```

## Boilerplate templates

### Proto3 boilerplate

```proto
syntax = "proto3";

package example.v1;

import "google/protobuf/timestamp.proto";

option java_multiple_files = true;

message User {
  // Use optional for explicit scalar presence in proto3.
  optional string display_name = 1;
  string email = 2;
  google.protobuf.Timestamp created_at = 3;

  oneof contact_method {
    string phone = 4;
    string slack = 5;
  }

  map<string, string> labels = 6;

  reserved 7, 8;
  reserved "legacy_id";
}

enum UserState {
  USER_STATE_UNSPECIFIED = 0;
  USER_STATE_ACTIVE = 1;
  USER_STATE_DISABLED = 2;
}

service UserService {
  rpc GetUser(GetUserRequest) returns (GetUserResponse);
}

message GetUserRequest {
  string user_id = 1;
}

message GetUserResponse {
  User user = 1;
}
```

### Proto2 boilerplate

```proto
syntax = "proto2";

package example.v1;

message User {
  optional string display_name = 1 [default = "unknown"];
  optional string email = 2;
  repeated string tags = 3;

  // Required is proto2-only and discouraged for new design.
  // required string id = 4;

  extensions 100 to max;

  reserved 5;
  reserved "legacy_id";
}

enum UserState {
  USER_STATE_UNSPECIFIED = 0;
  USER_STATE_ACTIVE = 1;
}

extend User {
  optional string external_profile = 100;
}
```

## Numbering and wire constraints quick reference

- Field numbers must be in `1..536870911`.
- Field numbers `19000..19999` are reserved by protobuf implementation.
- Never renumber existing fields.
- Never reuse removed field numbers or names.

## Safe schema evolution checklist

- Never renumber existing fields.
- Never reuse deleted field numbers.
- Reserve removed field numbers and names using `reserved`.
- Prefer additive changes: add new fields, do not mutate existing wire meaning.
- Avoid `required` for new work.
- Use `oneof` when exactly one field must be present.

## Protoc usage basics

- Verify installation:

```bash
protoc --version
```

- General compile shape:

```bash
protoc -I proto --<LANG>_out=gen/<lang> proto/path/to/file.proto
```

- Validate schema and emit descriptor set (useful for CI checks):

```bash
protoc -I proto --include_imports --descriptor_set_out=gen/schema.pb proto/path/to/file.proto
```

## Rust generation patterns

Rust codegen is commonly done in `build.rs` wrappers that call `protoc` for you.

- `prost-build` example (`build.rs`):

```rust
fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["proto/service.proto"], &["proto"])?;
    Ok(())
}
```

- `tonic-prost-build` example (`build.rs`):

```rust
fn main() {
    tonic_prost_build::configure()
        .compile_protos(&["proto/service.proto"], &["proto"])
        .unwrap();
}
```

- `protobuf` crate ecosystem example (`build.rs`, from protobuf-example):

```rust
use protobuf_codegen::CodeGen;

fn main() {
    CodeGen::new()
        .inputs(["proto_example/foo.proto", "proto_example/bar/bar.proto"])
        .include("proto")
        .dependency(protobuf_well_known_types::get_dependency("protobuf_well_known_types"))
        .generate_and_compile()
        .unwrap();
}
```

## TypeScript generation patterns

TypeScript typically uses `protoc` plugins.

- `ts-proto` quickstart:

```bash
protoc \
  --plugin=./node_modules/.bin/protoc-gen-ts_proto \
  --ts_proto_out=gen/ts \
  --ts_proto_opt=esModuleInterop=true \
  -I proto \
  proto/path/to/file.proto
```

- `@protobuf-ts/plugin` quickstart:

```bash
npx protoc --ts_out gen/ts --proto_path proto proto/path/to/file.proto
```

## Agent defaults for new schema requests

- Ask which syntax to use: `proto2` or `proto3`.
- If unspecified, recommend `proto3` with explicit `optional` for scalar presence.
- Ask which TypeScript generator to target: `ts-proto` or `@protobuf-ts/plugin`.
- If unspecified, recommend `ts-proto` for idiomatic TS interfaces and flexible options.
- Always call out features that are syntax-specific (for example `required`/extensions/default overrides in proto2).
