----
## External Docs Snapshot // moonrepo

- Captured: 2026-02-16T03:48:02.472Z
- Source root: https://moonrepo.dev/docs
- Source page: /docs/proto/commands/activate
- Keywords: moon, moonrepo, docs, monorepo, build, proto, commands, activate
- Summary: The `proto activate ` command will activate proto for the current shell session, by exporting
----

Source: https://moonrepo.dev/docs/proto/commands/activate

# activate

v0.38.0

The `proto activate ` command will activate proto for the current shell session, by exporting
environment variables and prepending `PATH` for each tool configured in the current directory.
Activation is ran each time the current directory changes using a shell hook.

info

Learn more about
[shell activation in the official workflow documentation](/docs/proto/workflows#shell-activation)!

### Arguments

- `` - The shell to activate for.

### Options

- `--export` - Print the activate instructions in shell-specific syntax.

- `--json` - Print the activate instructions in JSON format.

- `--no-bin` - Do not include `~/.proto/bin` when appending `PATH`.

- `--no-shim` - Do not include `~/.proto/shims` when prepending `PATH`.

- `--no-init` - Do not trigger activation when initialized in the shell, and instead wait for a cd/prompt change. v0.50.0

### Caveats

- Only tools that have a [version configured in `.prototools`](/docs/proto/config#pinning-versions) will be activated.

- Tool versions configured in the global `~/.proto/.prototools` are not included by default. Pass `--config-mode all` during activation to include them. Do note that this will worsen performance depending on the number of tools.

### Setup

The following activation steps should be added after all environment variable and `PATH`
modifications have happened in your shell, typically at the end of your shell profile.

#### Bash

Add the following line to the end of your `~/.bashrc` or `~/.bash_profile`.

```
eval "$(proto activate bash)"
```

#### Elvish

Generate the hook:

```
proto activate elvish > ~/.elvish/lib/proto-hook.elv
```

Then add the following line to your `~/.elvish/rc.elv` file.

```
use proto-hook
```

#### Fish

Add the following line to the end of your `~/.config/fish/config.fish`.

```
proto activate fish | source
```

#### Murex

Add the following line to the end of your `~/.murex_profile`.

```
proto activate murex -> source
```

#### Nu

Generate the hook:

```
(proto activate nu) | save ~/.config/nushell/proto-hook.nu
```

Then add the following line to your `~/.config/nushell/config.nu` file.

```
use proto-hook.nu
```

#### Pwsh

Add the following line to the end of your profile (`$PROFILE`).

```
proto activate pwsh | Out-String | Invoke-Expression
```

#### Zsh

Add the following line to the end of your `~/.zshrc`.

```
eval "$(proto activate zsh)"
```

----
## Notes / Comments / Lessons

- Collection method: sitemap discovery + markdown conversion.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
