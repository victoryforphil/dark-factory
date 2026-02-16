----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/apps/config-directories
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, apps, config directories
- Summary: Handling files and directories correctly in a command-line or TUI application ensures that the
----

Source: https://ratatui.rs/recipes/apps/config-directories

# Handle XDG Directories

Handling files and directories correctly in a command-line or TUI application ensures that the
application fits seamlessly into a user’s workflow and adheres to established conventions. One of
the key conventions on Linux-based systems is the XDG Base Directory Specification.

## Why the XDG Base Directory Specification?

[Section titled “Why the XDG Base Directory Specification?”](#why-the-xdg-base-directory-specification)

The XDG Base Directory Specification is a set of standards that define where user files should
reside, ensuring a cleaner home directory and a more organized storage convention. By adhering to
this standard, your application will store files in the expected directories, making it more
predictable and user-friendly.

## Using `directories-rs` for Path Resolution

[Section titled “Using directories-rs for Path Resolution”](#using-directories-rs-for-path-resolution)

The `directories-rs` library offers a Rust-friendly interface to locate common directories (like
config and data directories) based on established conventions, including the XDG Base Directory
Specification.

- Add `directories-rs` to your `Cargo.toml` Terminal window ``` cargo add directories ```

- Use the `ProjectDirs` struct to retrieve paths based on your project’s domain and project name and create helper functions for getting the `data_dir` and `config_dir`.

- Allow users to specify custom locations using environment variables. This flexibility can be crucial for users with unique directory structures or for testing.

- A good practice is to notify the user about the location of the configuration and data directories. An example from the template is to print out these locations when the user invokes the `--version` command-line argument. See the section on [Command line argument parsing](../cli-arguments/)

Here’s an example `get_data_dir()` and `get_config_dir()` functions for your reference:

```
use std::path::PathBuf;
use color_eyre::eyre::{self, WrapErr};use directories::ProjectDirs;
pub fn get_data_dir() -> eyre::Result&#x3C;PathBuf> {  let directory = if let Ok(s) = std::env::var("RATATUI_TEMPLATE_DATA") {    PathBuf::from(s)  } else if let Some(proj_dirs) = ProjectDirs::from("com", "kdheepak", "ratatui-template") {    proj_dirs.data_local_dir().to_path_buf()  } else {    return Err(eyre::eyre!("Unable to find data directory for ratatui-template"));  };  Ok(directory)}
pub fn get_config_dir() -> eyre::Result&#x3C;PathBuf> {  let directory = if let Ok(s) = std::env::var("RATATUI_TEMPLATE_CONFIG") {    PathBuf::from(s)  } else if let Some(proj_dirs) = ProjectDirs::from("com", "kdheepak", "ratatui-template") {    proj_dirs.config_local_dir().to_path_buf()  } else {    return Err(eyre::eyre!("Unable to find config directory for ratatui-template"));  };  Ok(directory)}
```

You will want to replace `kdheepak` with your user name or company name (or any unique name for that
matter); and `ratatui-app` with the name of your CLI.

I own [https://kdheepak.com](https://kdheepak.com) so I tend to use `com.kdheepak.ratatui-app` for my project directories.
That way it is unlikely that any other program will mess with the configuration files for the app I
plan on distributing.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/apps/config-directories.md)

 [Previous CLI Arguments](/recipes/apps/cli-arguments/) [Next Logging with Tracing](/recipes/apps/log-with-tracing/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
