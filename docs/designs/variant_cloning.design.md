# Variant Cloning Design

## Goals

- Use `Product` as the template source for creating new working `Variant` clones.
- Route clone orchestration under product scope: `POST /products/:productId/variants/clone`.
- Keep actor lifecycle separate from clone lifecycle (clone first, then `/actors` spawn).
- Support strategy-by-locator with room for future clone modes per store type.

## Product Workspace Home

- `Product.workspaceLocator` stores the preferred workspace root for future variant clones.
- `workspaceLocator` accepts only local locator syntax (`@local://...`).
- Clone destination resolution order:
  1. request `targetPath`
  2. `product.workspaceLocator`
  3. `config.variants.defaultWorkspaceLocator`
  4. fail with `VARIANTS_CLONE_WORKSPACE_UNRESOLVED`
- If resolved workspace root does not exist, core creates it (`mkdir -p` semantics).

## Auto Workspace Derivation on Product Create

- If create payload omits `workspaceLocator`:
  - local non-git product: parent directory of product path.
  - local git product: parent directory of git `repoRoot`.
  - direct git locator (`@git://...`) without local path context: leave `workspaceLocator = null`.

## Clone Strategies (v1)

- Strategy selection is based on **product locator type**.
- Supported clone types:
  - `local.copy`: recursively copy source directory to target path.
  - `git.clone_branch`: clone remote + create/switch branch.
- Request `cloneType` defaults to `auto`.

## API Surface

- `POST /products/:id/variants/clone`
  - Body:
    - `name?: string`
    - `targetPath?: string`
    - `cloneType?: "auto" | "local.copy" | "git.clone_branch"`
    - `branchName?: string`
    - `sourceVariantId?: string`
  - Response:
    - `{ variant, clone }` where `clone` includes strategy, source kind, target path/locator, branch metadata.
- Product-scoped variant route additions:
  - `GET /products/:id/variants`
  - `POST /products/:id/variants`

## Naming Rules

- Target directory (when omitted): `<slug>_<counter>` with token fallback.
- Git branch (when omitted): `df/<slug>-<token>`.
- Variant name (when omitted): `clone_0000` style with token fallback.

## Error Codes

- `VARIANTS_CLONE_WORKSPACE_UNRESOLVED`
- `VARIANTS_CLONE_TARGET_INVALID`
- `VARIANTS_CLONE_UNSUPPORTED`
- `VARIANTS_CLONE_FAILED`

## Future Extension Points

- Add more per-store clone modes:
  - `git.worktree`
  - `git.branch_only`
  - container-backed clone modes
- Keep strategy registry extensible without changing route contract.
