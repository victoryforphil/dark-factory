# `dark_cli` TODO

Actionable implementation tasks for current and near-term work.

## Foundation

- [ ] Replace `Hello, world!` entrypoint with a minimal clap-based CLI shell.
- [ ] Add top-level app metadata (`name`, `version`, `about`) and help text.
- [ ] Add a basic command namespace structure (`health`, `core`, or equivalent).

## Config and Runtime

- [ ] Define runtime config struct for core base URL and output format defaults.
- [ ] Add environment-based overrides for core URL and logging level.
- [ ] Initialize logging with `pretty_env_logger` and documented defaults.

## HTTP Integration

- [ ] Implement a small typed `reqwest` client wrapper for `dark_core`.
- [ ] Add one real command that calls a live `dark_core` endpoint.
- [ ] Map transport and HTTP failures into contextual CLI errors.

## Output and UX

- [ ] Add output formatting options: `pretty`, `json`, `toml`.
- [ ] Keep output rendering in a dedicated module/function layer.
- [ ] Ensure non-zero exit codes are used for command failures.

## Docs Hygiene

- [ ] Keep `README.md` aligned with implemented behavior only.
- [ ] Update `PLANNING.md` stage statuses as milestones land.
