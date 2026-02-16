----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/apps/log-with-tracing
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, apps, log with tracing
- Summary: You’ll need to install `tracing` and a few related dependencies:
----

Source: https://ratatui.rs/recipes/apps/log-with-tracing

# Setup Logging with tracing

You’ll need to install `tracing` and a few related dependencies:

Terminal window

```
cargo add tracing-error tracingcargo add tracing-subscriber --features env-filtercargo add directories lazy_static color-eyre # (optional)
```

You can paste the following in any module in your project.

```
use std::path::PathBuf;
use color_eyre::eyre::{Context, Result};use directories::ProjectDirs;use lazy_static::lazy_static;use tracing::error;use tracing_error::ErrorLayer;use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt, Layer};
lazy_static! {  pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME").to_uppercase().to_string();  pub static ref DATA_FOLDER: Option&#x3C;PathBuf> =    std::env::var(format!("{}_DATA", PROJECT_NAME.clone())).ok().map(PathBuf::from);  pub static ref LOG_ENV: String = format!("{}_LOGLEVEL", PROJECT_NAME.clone());  pub static ref LOG_FILE: String = format!("{}.log", env!("CARGO_PKG_NAME"));}
fn project_directory() -> Option&#x3C;ProjectDirs> {  ProjectDirs::from("com", "kdheepak", env!("CARGO_PKG_NAME"))}
pub fn get_data_dir() -> PathBuf {  let directory = if let Some(s) = DATA_FOLDER.clone() {    s  } else if let Some(proj_dirs) = project_directory() {    proj_dirs.data_local_dir().to_path_buf()  } else {    PathBuf::from(".").join(".data")  };  directory}
pub fn initialize_logging() -> Result&#x3C;()> {  let directory = get_data_dir();  std::fs::create_dir_all(directory.clone())?;  let log_path = directory.join(LOG_FILE.clone());  let log_file = std::fs::File::create(log_path)?;  let log_filter = std::env::var("RUST_LOG")    .or_else(|_| std::env::var(LOG_ENV.clone()))    .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")));  let file_subscriber = tracing_subscriber::fmt::layer()    .with_file(true)    .with_line_number(true)    .with_writer(log_file)    .with_target(false)    .with_ansi(false)    .with_filter(tracing_subscriber::filter::EnvFilter::builder().parse_lossy(log_filter));  tracing_subscriber::registry().with(file_subscriber).with(ErrorLayer::default()).init();  Ok(())}
/// Similar to the `std::dbg!` macro, but generates `tracing` events rather/// than printing to stdout.////// By default, the verbosity level for the generated events is `DEBUG`, but/// this can be customized.#[macro_export]macro_rules! trace_dbg {    (target: $target:expr, level: $level:expr, $ex:expr) => {{        match $ex {            value => {                tracing::event!(target: $target, $level, ?value, stringify!($ex));                value            }        }    }};    (level: $level:expr, $ex:expr) => {        trace_dbg!(target: module_path!(), level: $level, $ex)    };    (target: $target:expr, $ex:expr) => {        trace_dbg!(target: $target, level: tracing::Level::DEBUG, $ex)    };    ($ex:expr) => {        trace_dbg!(level: tracing::Level::DEBUG, $ex)    };}
```

Call `initialize_logging()?` in your `main()` function.

The log level is decided by the `${YOUR_CRATE_NAME}_LOGLEVEL` environment variable (default =
`log::LevelFilter::Info`).

Additionally, the location of the log files would be decided by your environment variables. See
[the section on XDG directories](../config-directories/) for more information.

Tip

Check out [`tui-logger`](https://github.com/gin66/tui-logger) for setting up a tui logger widget
with tracing.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/apps/log-with-tracing.md)

 [Previous Configuration Directories](/recipes/apps/config-directories/) [Next Terminal and Event Handler](/recipes/apps/terminal-and-event-handler/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
