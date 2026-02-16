----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/sharing-config
- Keywords: moon, moonrepo, docs, monorepo, build, guides, sharing config
- Summary: For large companies, open source maintainers, and those that love reusability, more often than not
----

Source: https://moonrepo.dev/docs/guides/sharing-config

# Sharing workspace configuration

For large companies, open source maintainers, and those that love reusability, more often than not
you'll want to use the same configuration across all repositories for consistency. This helps reduce
the maintenance burden while ensuring a similar developer experience.

To help streamline this process, moon provides an `extends` setting in both
[`.moon/workspace.yml`](/docs/config/workspace#extends),
[`.moon/toolchains.yml`](/docs/config/toolchain#extends), and
[`.moon/tasks/all.yml`](/docs/config/tasks#extends). This setting requires a HTTPS URL or relative
file system path that points to a valid YAML document for the configuration in question.

A great way to share configuration is by using GitHub's "raw file view", as demonstrated below using
our very own [examples repository](https://github.com/moonrepo/examples).

.moon/tasks/all.yml

```
extends: 'https://raw.githubusercontent.com/moonrepo/examples/master/.moon/tasks/all.yml'
```

## Versioning

Inheriting an upstream configuration can be dangerous, as the settings may change at any point,
resulting in broken builds. To mitigate this, you can used a "versioned" upstream configuration,
which is ideally a fixed point in time. How this is implemented is up to you or your company, but we
suggest the following patterns:

### Using versioned filenames

A rudimentary solution is to append a version to the upstream filename. When the file is modified, a
new version should be created, while the previous version remains untouched.

```
-extends: '../shared/project.yml'+extends: '../shared/project-v1.yml'
```

### Using branches, tags, or commits

When using a version control platform, like GitHub above, you can reference the upstream
configuration through a branch, tag, commit, or sha. Since these are a reference point in time, they
are relatively safe.

```
-extends: 'https://raw.githubusercontent.com/moonrepo/examples/master/.moon/tasks/all.yml'+extends: 'https://raw.githubusercontent.com/moonrepo/examples/c3f10160bcd16b48b8d4d21b208bb50f6b09bd96/.moon/tasks/all.yml'
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
