----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/notifications
- Keywords: moon, moonrepo, docs, monorepo, build, guides, notifications
- Summary: moon is able to send operating system desktop notifications for specific events in the action
----

Source: https://moonrepo.dev/docs/guides/notifications

# Terminal notifications

v1.38.0

moon is able to send operating system desktop notifications for specific events in the action
pipeline, on behalf of your terminal application. This is useful for continuous feedback loops and
reacting to long-running commands while multi-tasking.

Notifications are opt-in and must be enabled with the
[`notify.terminalNotifications`](/docs/config/workspace#terminalnotifications) setting.

.moon/workspace.yml

```
notifier:  terminalNotifications: 'always'
```

## Setup

Notifications must be enabled at the operating system level.

### Linux

Linux support is based on the [XDG specification](https://en.wikipedia.org/wiki/XDG) and utilizes
D-BUS APIs, primarily the
[`org.freedesktop.Notifications.Notify`](https://www.galago-project.org/specs/notification/0.9/x408.html#command-notify)
method. Refer to your desktop distribution for more information.

Notifications will be sent using the `moon` application name (the current executable).

### macOS

- Open "System Settings" or "System Preferences"

- Select "Notifications" in the left sidebar

- Select your terminal application from the list (e.g., "Terminal", "iTerm", etc)

- Ensure "Allow notifications" is enabled

- Customize the other settings as desired

Notifications will be sent from your currently running terminal application, derived from the
`TERM_PROGRAM` environment variable. If we fail to detect the terminal, it will default to "Finder".

### Windows

Requires Windows 10 or later.

- Open "Settings"

- Go to the "System" panel

- Select "Notifications & Actions" in the left sidebar

- Ensure notifications are enabled

Notifications will be sent from the "Windows Terminal" app if it's currently in use, otherwise from
"Microsoft PowerShell".

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
