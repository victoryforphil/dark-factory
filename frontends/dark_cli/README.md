# Dark Factory - Frontend - CLI 

- Basic Rust-based CLI
- Using Clap for argument parsing and Serde for config management (toml)
- Supports requesting / executign the commands defined / hosted by dark_core (HTTP and future WebSockets)\

# Notes
- Supports `--format` flag for output formatting (pretty (default), json, toml)
- Can use http://localhost:4150/llms.txt / http://localhost:4150/openapi/json for rerfence on API

# Commands
- (All HTTP commands / routes defined in dark_core are supported by default, this is just a reference of the custom ones defined in the CLI)

# Stack / Tech Choices

- Clap / Serde defined structs for CLI args w/ ENV (clap -F env) override support and .env / .env.template support
- Anyhow returned results for functiosn
- thiserror for custom error types
- pretty_env_logger for logging
- Default RUST_LOG=info
- reqwest for HTTP client to dark_core