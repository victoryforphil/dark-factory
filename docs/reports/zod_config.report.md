Below is an **RFC / context doc** you can drop into your repo (e.g. `docs/rfc/rfc-config-system.md`). It captures what you asked for, the decisions we made, why, and enough implementation detail for a future agent to build it cleanly.

---

# RFC: Internal Config System (Bun) using Zod + TOML + ENV overrides

**Status:** Draft
**Owner:** Andreas Labs (internal tooling)
**Last updated:** 2026-02-15
**Audience:** Future agent implementing config loader + schema composition

## 1) Context & Problem Statement

We want an internal configuration system for a TypeScript/JavaScript runtime (Bun preferred; Node-compatible is nice). Requirements from the request:

* Define config via **TypeScript types** with **runtime validation**.
* Config definition must be **composable** across subsystems (each subsystem can contribute types and defaults, and import/merge other types).
* Support **default values** in code.
* Support reading/writing config from/to **TOML**.
* Support overriding config values via **environment variables** (ENV), with ENV having highest precedence.
* This is **internal**: not tied to Prisma schemas and **not exposed via Elysia API** (future config update endpoint is “maybe”, but not a concern).

We explicitly *like* Zod, so we want an approach that is “serde-like” in developer ergonomics: one definition that gives types + validation + defaults + composability.

## 2) Non-goals

* Generating OpenAPI schemas.
* Sharing schemas with Elysia routes (not needed for internal config).
* A full remote config service.
* Secret management (Vault/Infisical/etc.) beyond reading env vars.

## 3) Decision Summary

### Decision: Use **Zod** for schema, defaults, and runtime validation.

* Zod provides TS inference and runtime validation.
* Zod supports “coercion” for common env-file / string inputs via `z.coerce.*`. ([Zod][1])
* Defaults can be expressed in the schema, and applied by parsing an empty object: `Schema.parse({})` (implementation pattern).

### Decision: Use TOML file as a persisted human-editable layer via `@iarna/toml`.

* TOML is used for “checked-in / local dev / ops friendly” config.
* `@iarna/toml` provides parse + stringify in a straightforward JSON-like interface. ([npm][2])

### Decision: Use a **layered merge** strategy

**Precedence order** (lowest → highest):

1. **Defaults** (schema-defined)
2. **TOML file** (optional)
3. **ENV overrides** (highest; CI/prod-friendly)

### Decision: Use a **nested ENV naming convention**

Adopt an env prefix and path separator convention (example):

* `APP__DB__URL=...` → `config.db.url`
* `APP__LOGGING__LEVEL=debug` → `config.logging.level`

This approach is:

* predictable
* easy to implement
* composes naturally with nested config objects

### Decision: Target Bun, but keep Node compatibility

* In Bun, env vars can be accessed via both `Bun.env` and `process.env`. ([Bun][3])
* Bun supports `.env` loading and custom env file selection via `--env-file`. ([Bun][4])

## 4) Rationale & Tradeoffs

### Why Zod (and not TypeBox/Ajv/Convict)

* This system is **internal-only** and not an HTTP boundary, so TypeBox’s biggest win (tight integration with OpenAPI generation and certain web frameworks) is not relevant.
* Zod is already known/liked and provides strong ergonomics for defaults + merging + validation.
* Zod coercion exists, but we must handle a known nuance: `z.coerce.boolean()` uses JS `Boolean(value)`, meaning `"false"` is truthy (→ `true`). The Zod docs explicitly describe this behavior. ([odocs-zod.vercel.app][5])
  → We will implement **custom boolean parsing** for env strings instead of relying on `z.coerce.boolean()` for env-sourced values.

### Why TOML + `@iarna/toml`

* TOML is readable and friendly for local config.
* `@iarna/toml` is a simple parse/stringify tool with wide prior usage. ([npm][2])
* Tradeoff: the package is not very recently published. That’s acceptable because TOML grammar is stable; we also keep the parser behind an interface so swapping later is easy.

### Why a layered merge

* Mirrors common config systems (defaults → file → env).
* Works well with “subsystem config” composition.
* Makes it easy to print “effective config” for debugging.

## 5) Proposed Design

### 5.1 Public API (module interface)

* `loadConfig(options?): AppConfig`
* `writeConfig(path, config): void` (optional)
* `getConfig(): AppConfig` (optional singleton)
* `printConfig(config): void` (optional debugging)

Where:

* `AppConfig` is inferred from the Zod schema `AppSchema`.
* `options` includes:

  * `path?: string` (default `./config.toml`)
  * `envPrefix?: string` (default `APP__`)
  * `strict?: boolean` (default `true` in prod, configurable)
  * `allowUnknown?: boolean` (dev convenience)

### 5.2 Schema Composition

Each subsystem exports a Zod schema (and optionally helper types). Example structure:

```
src/
  config/
    schema.ts        // AppSchema composition root
    load.ts          // loader (defaults + toml + env)
    env.ts           // env parsing/mapping helpers
    toml.ts          // toml helpers
    subsystems/
      db.ts
      logging.ts
      telemetry.ts
```

Subsystem example:

* `config/subsystems/db.ts` exports `DbSchema`
* `config/schema.ts` composes them into `AppSchema`

Composition patterns:

* Nested object property: `db: DbSchema.default({})`
* Or `AppSchema = z.object({...}).merge(OtherSchema)` if top-level keys are disjoint.

### 5.3 Loading Flow

1. **Defaults**

* `defaults = AppSchema.parse({})`

  * This applies schema defaults.

2. **TOML file (optional)**

* If file exists: `fileCfg = TOML.parse(text)`
* Else: `{}`

3. **ENV overrides**

* Build `envCfg` object by reading env vars with prefix.
* Convert key path segments to object nesting.

4. **Merge**

* `merged = deepMerge(defaults, fileCfg, envCfg)`
  (ENV wins)

5. **Validate**

* `final = AppSchema.parse(merged)`

### 5.4 ENV parsing rules

**Key mapping**

* Prefix: `APP__`
* Separator: `__`
* Case: convert segments to `camelCase` or `lowercase` and match schema keys (choose one; recommend *lowercase* only if schema keys are lowercase already—otherwise implement a `toCamel()`).
* Example:

  * `APP__DB__POOL_SIZE=10` → `db.poolSize` (requires `SNAKE_CASE`→`camelCase` transform on each segment)
  * `APP__LOGGING__LEVEL=info` → `logging.level`

**Value parsing**
ENV values arrive as strings. We will parse into primitives before merging:

* If value is `""` (empty string): treat as **undefined** (do not override), unless explicitly allowed by schema.
* If value matches `/^(true|false)$/i`: parse to boolean.
* If value matches numeric: parse to number (but be careful with leading zeros and NaN).
* If value starts with `{` or `[` : attempt `JSON.parse` (for arrays/objects).
* Otherwise: keep string.

**Important boolean nuance**
Do **not** rely on `z.coerce.boolean()` for env strings like `"false"` because JS `Boolean("false") === true`. Zod docs confirm `z.coerce.boolean()` is just `Boolean(value)`. ([odocs-zod.vercel.app][5])

### 5.5 Strictness & Unknown keys

By default, config should fail on unknown keys to catch typos:

* `AppSchema.strict()` in production-like runs

In dev, we may allow unknown keys:

* `AppSchema.passthrough()` or a runtime option that chooses strictness.

The agent should implement this as:

* `const Schema = strict ? AppSchema.strict() : AppSchema.passthrough()`

### 5.6 Writing TOML

Two modes:

1. **Write full config**: stringify the validated final object.
2. **Write minimal overrides** (recommended for cleanliness): compute a deep diff between defaults and final, then write only the diff.

Recommendation:

* Implement (1) first for simplicity.
* Add (2) if/when desired.

### 5.7 Bun-specific behavior notes

* Env access: `Bun.env` and `process.env` both work in Bun. ([Bun][3])
* `.env` files: Bun supports specifying env files via `--env-file`. ([Bun][4])
  This RFC assumes that `.env` loading is handled by Bun invocation (CI/dev scripts), not by our config module. We can add a fallback `dotenv` dependency later if needed (not required now).

## 6) Security Considerations

* Treat config as potentially containing secrets (db URLs, tokens).
* Never log full config in prod without redaction.
* Provide a `redactConfig(config)` helper that masks known secret fields (`password`, `token`, `secret`, etc.).

## 7) Testing Strategy

* Unit tests for:

  * deep merge precedence
  * env key → nested object mapping
  * env value parsing (booleans, numbers, json arrays)
  * strict/passthrough behavior
  * TOML read/write round-trip for typical config shapes

* Integration tests:

  * With a sample `config.toml` and env overrides set.

## 8) Implementation Notes (starter checklist for the agent)

* [ ] Choose env key casing approach: `APP__DB__POOL_SIZE` → `db.poolSize` via per-segment `snake_to_camel`.
* [ ] Implement `parseEnvValue(string) -> unknown` with boolean/number/json detection.
* [ ] Implement `envOverlay(prefix) -> object` by iterating env vars.
* [ ] Implement `deepMerge(a,b,c)`; prefer a deterministic “plain object only” deep merge (arrays replaced, not concatenated).
* [ ] Wrap TOML parsing errors with file path + line context if possible.
* [ ] Add `strict` option toggling `.strict()` vs `.passthrough()`.
* [ ] Add `configPath` default and allow override.

## 9) Alternatives Considered (and why not)

* **TypeBox/Ajv**: Great for JSON Schema and HTTP/OpenAPI alignment, but config is internal-only; Zod ergonomics + existing preference wins.
* **Convict**: A classic layered config system, but less “native TS schema-first” than Zod.
* **dotenv-only**: Not sufficient; we want defaults + TOML + validation, and Bun already has env file support via `--env-file`. ([Bun][4])

---

## Appendix A: Relevant References

* Zod coercion API (`z.coerce.*`) ([Zod][1])
* Zod boolean coercion behavior is `Boolean(value)` (important nuance) ([odocs-zod.vercel.app][5])
* `@iarna/toml` package (parse/stringify) ([npm][2])
* Bun environment variables (`Bun.env` and `process.env`) ([Bun][3])
* Bun `--env-file` support ([Bun][4])

---

If you want, I can also generate:

* a **skeleton implementation** (`src/config/*`) that matches this RFC,
* plus a **sample `config.toml`** and **env override examples** that mirror your likely subsystems (db, telemetry, drone pipeline toggles, etc.).

[1]: https://zod.dev/api?utm_source=chatgpt.com "Defining schemas"
[2]: https://www.npmjs.com/package/%40iarna/toml?utm_source=chatgpt.com "iarna/toml"
[3]: https://bun.com/docs/guides/runtime/set-env?utm_source=chatgpt.com "Set environment variables"
[4]: https://bun.com/docs/runtime/environment-variables?utm_source=chatgpt.com "Environment Variables"
[5]: https://odocs-zod.vercel.app/?utm_source=chatgpt.com "Zod | Documentation"


// ChatGpt - Gpt-5.2 (Web)