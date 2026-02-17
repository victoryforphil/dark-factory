# TUI Unification — Agent Kickoff Prompt

Paste this into a fresh long-running agent session to execute the full TUI unification plan.
Once out of plan mode, move this file to `docs/prompts/tui_unification.md`.

---

## Prompt

You are executing a multi-phase refactoring plan for three Ratatui TUI crates in this workspace. The full plan lives in `.opencode/plans/tui_unification_index.md` with six phase docs alongside it. Your job is to execute each phase sequentially, verify compilation after every step, commit after each phase, and move on.

### Orientation (do this first)

1. Read `.opencode/plans/tui_unification_index.md` to understand the full scope, dependency graph, and target architecture.
2. Read `AGENTS.md` and `STYLE.md` for repo conventions and commit message format.
3. Read `docs/lessons/*.lessons.md` for any relevant lessons.
4. Skim the three target crate READMEs:
   - `lib/dark_tui_components/README.md`
   - `frontends/dark_chat/README.md`
   - `frontends/dark_tui/README.md`

### Context gathering strategy

**IMPORTANT: Use explorer sub-agents (`@explore`) for all codebase context gathering instead of reading files directly.**

Your context window is precious over a 6-phase execution. Direct file reads accumulate fast and will exhaust your context before you finish. Instead:

- When a plan step says "find the current implementation of X" or "locate all usages of Y", delegate to an `@explore` agent with a targeted question and let it return just the answer (file paths, line numbers, relevant snippets).
- When you need to understand module structure, imports, or function signatures before editing, ask `@explore` to summarize them.
- Only read files directly when you are about to edit them (the Edit tool requires a prior Read).
- After editing, you do NOT need to re-read the file to verify — use `cargo check` instead.

**Explorer prompt pattern:**

```
Thoroughness: quick

I need to find [specific thing] in [specific crate/path].
Return: [exact deliverable — file paths, line numbers, function signatures, etc.]
```

Keep explorer requests focused and specific. Broad "tell me everything about this crate" requests waste sub-agent context too.

### Execution loop

For each phase (1 through 6), follow this cycle:

1. **Read the plan**: Read `.opencode/plans/tui_unification_phase{N}.md` fully.
2. **Create a todo list**: Use TodoWrite to track each step in the phase. Mark steps in_progress/completed as you go.
3. **Gather context via explorer**: Before each step, use `@explore` to locate the exact code you need to move, extract, or modify. Only then Read the specific file you will Edit.
4. **Execute the step**: Create/edit files as described in the plan. The plans include code snippets — use them as starting points but adapt to the actual current code (it may have drifted since the plan was written).
5. **Verify after each step**:
   ```bash
   cargo check -p dark_tui_components && cargo check -p dark_chat && cargo check -p dark_tui
   ```
   If check fails, fix before moving on. Do NOT proceed with a broken workspace.
6. **Run tests after each step** (when the phase adds or modifies tests):
   ```bash
   cargo test -p dark_tui_components && cargo test -p dark_chat && cargo test -p dark_tui
   ```
7. **Commit after the phase completes** (all steps done + checks pass):
   - Use the commit format from `AGENTS.md` Section 6.
   - Example: `TUI Components // Phase 1 // Extract shared utility functions to dark_tui_components`
   - Include a brief rationale in the commit body.

### Phase execution order

Execute in this order (matches dependency graph):

1. Phase 1: Extract shared utilities (independent)
2. Phase 2: Component trait (independent)
3. Phase 3: Split app.rs (independent, but touches same crates as 1/2 — execute after to avoid merge pain)
4. Phase 4: Unify shared panels (requires 1 + 2)
5. Phase 5: Mature dark_chat framework (requires 1 + 2 + 4)
6. Phase 6: Final cleanup (requires all above)

### Recovery and adaptation

- **Plan drift**: The plans reference specific line numbers. These WILL shift as you make edits. Use `@explore` or grep to re-locate targets rather than trusting stale line numbers.
- **Compilation failures**: Fix them immediately. Common causes: missing imports after moves, visibility (`pub` vs `pub(crate)`), type mismatches from signature changes.
- **Skipping steps**: If a step is already done (e.g., a function was already extracted), mark it completed and note why you skipped it.
- **Blocking issues**: If a step requires a design decision not covered in the plan, stop and ask the user rather than guessing.

### Completion

After all 6 phases:

1. Run full verification:
   ```bash
   cargo check -p dark_tui_components && cargo check -p dark_chat && cargo check -p dark_tui
   cargo test -p dark_tui_components && cargo test -p dark_chat && cargo test -p dark_tui
   ```
2. Summarize what was accomplished: phases completed, commits made, any deferred items.
3. If applicable, run `/reflect` to capture lessons learned during execution.
