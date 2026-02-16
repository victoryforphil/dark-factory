# `dark_cli` Planning

Staged planning document for future work.

This file is intentionally forward-looking. Keep `README.md` current-state only.

## Stage 0 - Bootstrap CLI Shell

Objective: move from placeholder binary to a real command shell.

- Deliver clap-powered root command and one minimal subcommand path.
- Establish predictable app startup, logging init, and error return flow.

Exit criteria:

- CLI starts with `--help` and subcommand help output.
- Binary exits with success/failure codes correctly.

## Stage 1 - Config + Output Baseline

Objective: add runtime configuration and formatting surface.

- Define config model (core URL, default output format, log level).
- Support env override path for key runtime settings.
- Implement unified output layer for `pretty` / `json` / `toml`.

Exit criteria:

- Commands read config defaults and honor env overrides.
- Output format selection works consistently for at least one command.

## Stage 2 - Core HTTP Command Coverage

Objective: establish reliable HTTP-based command execution.

- Add typed client wrapper over `reqwest`.
- Implement initial set of `dark_core`-backed commands.
- Add robust status-code and network error mapping.

Exit criteria:

- At least one production-intent command path is stable end-to-end.
- Errors are contextual and actionable.

## Stage 3 - Transport Abstraction Readiness

Objective: prepare for more than one transport without overengineering.

- Extract transport-facing trait/interface only when two transports are needed.
- Preserve HTTP path as the maintained hot path.
- Define boundaries for possible future WebSocket support.

Exit criteria:

- HTTP path remains clear and maintainable.
- Future transport integration points are documented and minimal.

## Stage 4 - Quality, Testing, Packaging

Objective: harden developer and release workflows.

- Add unit tests for command parsing and output formatting.
- Add integration tests for selected core commands.
- Document build/run/test workflows for contributors.

Exit criteria:

- Core command paths are covered by reproducible tests.
- Contributor docs match actual workflow commands.
