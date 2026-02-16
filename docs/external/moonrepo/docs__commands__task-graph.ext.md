----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/task-graph
- Keywords: moon, moonrepo, docs, monorepo, build, commands, task graph
- Summary: The `moon task-graph [target]` (or `moon tg`) command will generate and serve a visual graph of all
----

Source: https://moonrepo.dev/docs/commands/task-graph

# task-graph

v1.30.0

The `moon task-graph [target]` (or `moon tg`) command will generate and serve a visual graph of all
configured tasks as nodes, with dependencies between as edges, and can also output the graph in
[Graphviz DOT format](https://graphviz.org/doc/info/lang.html).

```
# Run the visualizer locally$ moon task-graph# Export to DOT format$ moon task-graph --dot > graph.dot
```

A task target can be passed to focus the graph to only that task and its dependencies. For
example, `moon task-graph app:build`.

### Arguments

- `[target]` - Optional target of task to focus.

### Options

- `--dependents` - Include direct dependents of the focused task.

- `--dot` - Print the graph in DOT format.

- `--host` - The host address. Defaults to `127.0.0.1`. v1.36.0

- `--json` - Print the graph in JSON format.

- `--port` - The port to bind to. Defaults to a random port. v1.36.0

## Example output

The following output is an example of the graph in DOT format.

```
digraph {    0 [ label="types:build" style=filled, shape=oval, fillcolor=gray, fontcolor=black]    1 [ label="runtime:build" style=filled, shape=oval, fillcolor=gray, fontcolor=black]    2 [ label="website:build" style=filled, shape=oval, fillcolor=gray, fontcolor=black]    1 -> 0 [ label="required" arrowhead=box, arrowtail=box]    2 -> 1 [ label="required" arrowhead=box, arrowtail=box]    2 -> 0 [ label="required" arrowhead=box, arrowtail=box]}
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
