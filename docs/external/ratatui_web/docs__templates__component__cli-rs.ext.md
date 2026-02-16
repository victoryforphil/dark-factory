----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /templates/component/cli-rs
- Keywords: ratatui, rust, tui, terminal ui, docs, templates, component, cli rs
- Summary: The `cli.rs` file is where we define the command line arguments for our app.
----

Source: https://ratatui.rs/templates/component/cli-rs

# Cli.rs

The `cli.rs` file is where we define the command line arguments for our app.

```
use clap::Parser;
use crate::config::{get_config_dir, get_data_dir};
#[derive(Parser, Debug)]#[command(author, version = version(), about)]pub struct Cli {    /// Tick rate, i.e. number of ticks per second    #[arg(short, long, value_name = "FLOAT", default_value_t = 4.0)]    pub tick_rate: f64,
    /// Frame rate, i.e. number of frames per second    #[arg(short, long, value_name = "FLOAT", default_value_t = 60.0)]    pub frame_rate: f64,}
const VERSION_MESSAGE: &#x26;str = concat!(    env!("CARGO_PKG_VERSION"),    "-",    env!("VERGEN_GIT_DESCRIBE"),    " (",    env!("VERGEN_BUILD_DATE"),    ")");
pub fn version() -> String {    let author = clap::crate_authors!();
    // let current_exe_path = PathBuf::from(clap::crate_name!()).display().to_string();    let config_dir_path = get_config_dir().display().to_string();    let data_dir_path = get_data_dir().display().to_string();
    format!(        "\{VERSION_MESSAGE}
Authors: {author}
Config directory: {config_dir_path}Data directory: {data_dir_path}"    )}
```

It uses the `clap` crate to define the command line interface.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/templates/component/cli-rs.md)

 [Previous App.rs](/templates/component/app-rs/) [Next Components/fps.rs](/templates/component/components-fps-rs/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
