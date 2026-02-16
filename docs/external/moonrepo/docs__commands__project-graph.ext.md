----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/commands/project-graph
- Keywords: moon, moonrepo, docs, monorepo, build, commands, project graph
- Summary: The `moon project-graph [id]` (or `moon pg`) command will generate and serve a visual graph of all
----

Source: https://moonrepo.dev/docs/commands/project-graph

# project-graph

The `moon project-graph [id]` (or `moon pg`) command will generate and serve a visual graph of all
configured projects as nodes, with dependencies between as edges, and can also output the graph in
[Graphviz DOT format](https://graphviz.org/doc/info/lang.html).

```
# Run the visualizer locally$ moon project-graph# Export to DOT format$ moon project-graph --dot > graph.dot
```

A project name can be passed to focus the graph to only that project and its dependencies. For
example, `moon project-graph app`.

### Arguments

- `[id]` - Optional ID or alias of a project to focus, as defined in [`projects`](/docs/config/workspace#projects).

### Options

- `--dependents` - Include direct dependents of the focused project.

- `--dot` - Print the graph in DOT format.

- `--host` - The host address. Defaults to `127.0.0.1`. v1.36.0

- `--json` - Print the graph in JSON format.

- `--port` - The port to bind to. Defaults to a random port. v1.36.0

### Configuration

- [`projects`](/docs/config/workspace#projects) in `.moon/workspace.yml`

## Example output

The following output is an example of the graph in DOT format.

```
digraph {    0 [ label="(workspace)" style=filled, shape=circle, fillcolor=black, fontcolor=white]    1 [ label="runtime" style=filled, shape=circle, fillcolor=gray, fontcolor=black]    2 [ label="website" style=filled, shape=circle, fillcolor=gray, fontcolor=black]    0 -> 1 [ arrowhead=none]    0 -> 2 [ arrowhead=none]}
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
