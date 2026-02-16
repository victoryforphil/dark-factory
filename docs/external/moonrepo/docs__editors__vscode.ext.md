----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/editors/vscode
- Keywords: moon, moonrepo, docs, monorepo, build, editors, vscode
- Summary: Enhance your VS Code experience with our integrated moon console! Whether you're a fan of the
----

Source: https://moonrepo.dev/docs/editors/vscode

# VS Code extension

Enhance your VS Code experience with our integrated moon console! Whether you're a fan of the
command line, or prefer interactive interfaces, our console will be a welcome experience.

This extension is in its early stages. Expect more advanced features in the future, like
autocompletion, config validation, and more!

## Views

All views are available within the moon sidebar. Simply click the moon icon in the left activity
bar!

### Projects

The backbone of moon is the projects view. In this view, all moon configured projects will be
listed, categorized by their [`layer`](/docs/config/project#layer), [`stack`](/docs/config/project#stack),
and designated with their [`language`](/docs/config/project#language).

Each project can then be expanded to view all available tasks. Tasks can be ran by clicking the `▶`
icon, or using the command palette.

This view is available in both the "Explorer" and "moon" sidebars.

### Tags

Similar to the projects view, the tags view displays projects grouped by their
[`tags`](/docs/config/project#tags).

This view is only available in the "moon" sidebar.

### Last run

Information about the last ran task will be displayed in a beautiful table with detailed stats.

This table displays all actions that were ran alongside the primary target(s). They are ordered
topologically via the action graph.

## Features

### YAML validation

To enable accurate validation of our YAML configuration files, you'll need to update the
`yaml.schemas` setting in `.vscode/settings.json` to point to the local schemas at
`.moon/cache/schemas`.

This can be automated by running the "moon: Append YAML schemas configuration to settings" in the
command palette, after the extension has been installed.

## Troubleshooting

View the
[official VS Code marketplace](https://marketplace.visualstudio.com/items?itemName=moonrepo.moon-console)
for more information on the extension, its commands, available settings, and more!

If you encounter a bug, or have a feature request, please submit them to the
[moonrepo/dev](https://github.com/moonrepo/dev/tree/master/packages/vscode-extension) repository!

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
