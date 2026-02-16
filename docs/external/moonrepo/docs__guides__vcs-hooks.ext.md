----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/guides/vcs-hooks
- Keywords: moon, moonrepo, docs, monorepo, build, guides, vcs hooks
- Summary: VCS hooks (most popular with [Git](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks)) are a
----

Source: https://moonrepo.dev/docs/guides/vcs-hooks

# VCS hooks

v1.9.0

VCS hooks (most popular with [Git](https://git-scm.com/book/en/v2/Customizing-Git-Git-Hooks)) are a
mechanism for running scripts at pre-defined phases in the VCS's lifecycle, most commonly
pre-commit, pre-push, or pre-merge. With moon, we provide a built-in solution for managing hooks,
and syncing them across developers and machines.

- [Learn more about Git hooks](https://git-scm.com/docs/githooks)

## Defining hooks

Hooks can be configured with the [`vcs.hooks`](/docs/config/workspace#hooks) setting in
[`.moon/workspace.yml`](/docs/config/workspace). This setting requires a map of hook names (in the
format required by your VCS), to a list of arbitrary commands to run within the hook script.
Commands are used as-is and are not formatted or interpolated in any way.

To demonstrate this, let's configure a `pre-commit` hook that runs a moon `lint` task for affected
projects, and also verifies that the commit message abides by a specified format (using
[pre-commit](https://pre-commit.com/) and the
[commitlint hook](https://github.com/alessandrojcm/commitlint-pre-commit-hook), for example).

.moon/workspace.yml

```
vcs:  hooks:    pre-commit:      - 'pre-commit run'      - 'moon run :lint --affected'    commit-msg:      - 'pre-commit run --hook-stage commit-msg --commit-msg-filename $ARG1'
```

info

All commands are executed from the repository root (not moon's workspace root) and must exist on
`PATH`. If `moon` is installed locally, you can execute it using a repository relative path, like
`./node_modules/@moonrepo/cli/moon`.

### Accessing argumentsv1.40.3

To ease interoperability between operating systems and terminal shells, we set passed arguments as
environment variables.

In your hook commands, you can access these arguments using the `$ARG` format, where `` is the
1-indexed position of the argument. For example, to access the first argument, you would use
`$ARG1`, the second argument would be `$ARG2`, and so on. `$ARG0` exists and points to the current
script.

## Enabling hooks

Hooks are a divisive subject, as some developers love them, and others hate them. Finding a viable
solution for everyone can be difficult, so with moon, we opted to support 2 distinct options, but
only 1 can be used at a time. Choose the option that works best for your project, team, or company!

caution

If you have existing VCS hooks, back them up as moon's implementation will overwrite them! To
migrate your existing hooks, [configure them as commands to run](#defining-hooks).

### Automatically for everyone

If you'd like hooks to be enforced for every contributor of the repository, then simply enable the
[`vcs.syncHooks`](/docs/config/workspace#synchooks) setting in
[`.moon/workspace.yml`](/docs/config/workspace). This will automatically generate hook scripts and link
them with the local VCS checkout, everytime a [target](/docs/concepts/target) is ran.

.moon/workspace.yml

```
vcs:  hooks: [...]  syncHooks: true
```

caution

Automatically activating hooks on everyone's computer is considered a sensitive action, because it
enables the execution of arbitrary code on the computers of the team members. Be careful about the
hook commands you define in the [`.moon/workspace.yml`](/docs/config/workspace) file.

### Manually by each developer

If you'd prefer contributors to have a choice in whether or not they want to use hooks, then simply
do nothing, and guide them to run the [`moon sync hooks`](/docs/commands/sync/vcs-hooks) command. This
command will generate hook scripts and link them with the local VCS checkout.

```
$ moon sync hooks
```

## Disabling hooks

If you choose to stop using hooks, you'll need to cleanup the previously generated hook scripts, and
reset the VCS checkout. To start, disable the `vcs.syncHooks` setting.

.moon/workspace.yml

```
vcs:  syncHooks: false
```

And then run the following command, which will delete files from your local filesystem. Every
developer that is using hooks will need to run this command.

```
$ moon sync hooks --clean
```

## How it works

When hooks are [enabled](#enabling-hooks), the following processes will take place.

- The configured [hooks](#defining-hooks) will be generated as individual script files in the `.moon/hooks` directory. Whether or not you commit or ignore these script files is your choice. They are written to the `.moon` directory so that they can be reviewed, audited, and easily tested, but are required.

- We then sync these generated hook scripts with the current VCS. For Git, we create `.git/hooks` files that execute our generated scripts, using repository relative commands. Any existing VCS hooks will be overwritten.

info

The `.moon/hooks` scripts are generated as Bash scripts (use a `.sh` file extension) on Unix, and
PowerShell scripts (use a `.ps1` file extension) on Windows.

### Git

On Unix based operating systems (Linux, macOS, etc), the `.moon/hooks` scripts are executed from
`.git/hooks` Bash files. Because of this, `bash` should be available on the system (which is
typically the case).

On Windows, things get tricky. Since Git has a requirement that `.git/hooks` files must be
extensionless, and older versions of PowerShell require an extension, we have to use a workaround.
To handle this, the `.git/hooks` files are Bash-like scripts (that should work on most machines)
that execute `.moon/hooks` using the `powershell.exe` (or `pwsh.exe`) executables. Because of this,
PowerShell must be available on the system.

## Examples

### Pre-commit

A perfect use case for the `pre-commit` hook is to check linting and formatting of the files being
committed. If either of these tasks fail, the commit will abort until they are fixed. Be sure to use
the [`--affected`](/docs/run-task#running-based-on-affected-files-only) option so that we only run on
changed projects!

.moon/workspace.yml

```
vcs:  hooks:    pre-commit:      - 'moon run :lint :format --affected --status=staged'
```

By default this will run on the entire project (all files). If you want to filter it to only the
changed files, enable the [`affectedFiles`](/docs/config/project#affectedfiles) task option.

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
