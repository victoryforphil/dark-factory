----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/apps/better-panic
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, apps, better panic
- Summary: Your application may panic for a number of reasons (e.g. when you call `.unwrap()` on a `None`). And
----

Source: https://ratatui.rs/recipes/apps/better-panic

# Better Panic Hooks

Your application may panic for a number of reasons (e.g. when you call `.unwrap()` on a `None`). And
when this happens, you want to be a good citizen and:

- provide a useful stacktrace so that they can report errors back to you.

- not leave the users terminal state in a botched condition, resetting it back to the way it was.

## `better-panic`

[Section titled “better-panic”](#better-panic)

[`better-panic`](https://github.com/mitsuhiko/better-panic) gives you pretty backtraces for panics.

Terminal window

```
cargo add better-panic
```

Here’s an example of `initialize_panic_handler()` using `better-panic` to provide a prettier
backtrace by default.

```
use better_panic::Settings;
pub fn initialize_panic_handler() {  std::panic::set_hook(Box::new(|panic_info| {    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();    crossterm::terminal::disable_raw_mode().unwrap();    Settings::auto().most_recent_first(false).lineno_suffix(true).create_panic_handler()(panic_info);  }));}
```

I personally like to reuse the [`Tui`](../terminal-and-event-handler/) struct in the panic handler.
That way, if I ever decide to move from `crossterm` to `termion` in the future, there’s one less
place in the project that I have to worry about refactoring.

Here’s an example of `initialize_panic_handler()` using
[`better_panic`](https://docs.rs/better-panic/latest/better_panic/) and
[`libc`](https://docs.rs/libc/latest/libc/) to provide a prettier backtrace by default.

```
use better_panic::Settings;
pub fn initialize_panic_handler() {  std::panic::set_hook(Box::new(|panic_info| {    match crate::tui::Tui::new() {      Ok(t) => {        if let Err(r) = t.exit() {          error!("Unable to exit Terminal: {r:?}");        }      },      Err(r) => error!("Unable to exit Terminal: {r:?}"),    }    better_panic::Settings::auto()      .most_recent_first(false)      .lineno_suffix(true)      .verbosity(better_panic::Verbosity::Full)      .create_panic_handler()(panic_info);    std::process::exit(libc::EXIT_FAILURE);  }));}
```

Now, let’s say I added a `panic!` to an application as an example:

```
diff --git a/src/components/app.rs b/src/components/app.rsindex 289e40b..de48392 100644--- a/src/components/app.rs+++ b/src/components/app.rs@@ -77,6 +77,7 @@ impl App {   }
   pub fn increment(&#x26;mut self, i: usize) {+    panic!("At the disco");     self.counter = self.counter.saturating_add(i);   }
```

This is what a prettier stacktrace would look like with `better-panic`:

```
Backtrace (most recent call last):  File "/Users/kd/gitrepos/myapp/src/main.rs:46", in ratatui_async_template::main    Ok(())  File "/Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/runtime.rs:304", in tokio::runtime::runtime::Runtime::block_on    Scheduler::MultiThread(exec) => exec.block_on(&#x26;self.handle.inner, future),  File "/Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/scheduler/multi_thread/mod.rs:66", in tokio::runtime::scheduler::multi_thread::MultiThread::block_on    enter  File "/Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/context.rs:315", in tokio::runtime::context::BlockingRegionGuard::block_on    park.block_on(f)  File "/Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/park.rs:283", in tokio::runtime::park::CachedParkThread::block_on    if let Ready(v) = crate::runtime::coop::budget(|| f.as_mut().poll(&#x26;mut cx)) {  File "/Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/coop.rs:73", in tokio::runtime::coop::budget    with_budget(Budget::initial(), f)  File "/Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/coop.rs:107", in tokio::runtime::coop::with_budget    f()  File "/Users/kd/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.28.2/src/runtime/park.rs:283", in tokio::runtime::park::CachedParkThread::block_on::{{closure}}    if let Ready(v) = crate::runtime::coop::budget(|| f.as_mut().poll(&#x26;mut cx)) {  File "/Users/kd/gitrepos/myapp/src/main.rs:44", in ratatui_async_template::main::{{closure}}    runner.run().await?;  File "/Users/kd/gitrepos/myapp/src/runner.rs:80", in ratatui_async_template::runner::Runner::run::{{closure}}    if let Some(action) = component.update(action.clone())? {  File "/Users/kd/gitrepos/myapp/src/components/app.rs:132", in &#x3C;ratatui_async_template::components::app::App as ratatui_async_template::components::Component>::update    Action::Increment(i) => self.increment(i),  File "/Users/kd/gitrepos/myapp/src/components/app.rs:80", in ratatui_async_template::components::app::App::increment    panic!("At the disco");
The application panicked (crashed).  At the discoin src/components/app.rs:80thread: main
```

With `.most_recent_first(false)` the last line of the stacktrace is typically where the error has
occurred. This makes it fast and easy to find the error without having to scroll up the terminal
history, and iterate on your application rapidly during development.

This kind of detailed stacktrace is only available in debug builds. For release builds, you may get
inlined or truncated stacktraces.

For example, here’s what I get when I compile with all optimizations on:

```
Backtrace (most recent call last):  File "&#x3C;unknown>:0", in __mh_execute_header  File "&#x3C;unknown>:0", in __mh_execute_header  File "&#x3C;unknown>:0", in __mh_execute_header  File "&#x3C;unknown>:0", in __mh_execute_header  File "&#x3C;unknown>:0", in __mh_execute_header  File "&#x3C;unknown>:0", in __mh_execute_header  File "&#x3C;unknown>:0", in __mh_execute_header  File "&#x3C;unknown>:0", in __mh_execute_header  File "&#x3C;unknown>:0", in __mh_execute_header  File "&#x3C;unknown>:0", in __mh_execute_header  File "&#x3C;unknown>:0", in __mh_execute_header  File "&#x3C;unknown>:0", in __mh_execute_header  File "&#x3C;unknown>:0", in __mh_execute_header
The application panicked (crashed).  At the discoin src/components/app.rs:80thread: main
```

This is not particularly useful to show to the average user. We’ll discuss better solutions for what
to show the users of your application in the following subsections.

## human-panic

[Section titled “human-panic”](#human-panic)

To use [human-panic](https://github.com/rust-cli/human-panic), you’ll have to install it as a
dependency:

Terminal window

```
cargo add human-panic
```

Personally, I think `human-panic` provides the most user friendly panic handling functionality out
of the box when users experience an unexpected panic:

```
Well, this is embarrassing.
myapp had a problem and crashed. To help us diagnose the problem you can send us a crash report.
We have generated a report file at "/var/folders/l4/bnjjc6p15zd3jnty8c_qkrtr0000gn/T/report-ce1e29cb-c17c-4684-b9d4-92d9678242b7.toml". Submit an issue or email with the subject of "myapp Crash Report" and include the report as an attachment.
- Authors: Dheepak Krishnamurthy
We take privacy seriously, and do not perform any automated error collection. In order to improve the software, we rely on people to submit reports.
Thank you kindly!
```

It generates a report where information relevant to the crash is logged. Here’s the content of the
temporary report file that `human-panic` creates (with optimizations turned on):

```
name = "myapp"operating_system = "Mac OS 13.5.2 [64-bit]"crate_version = "0.1.0"explanation = """Panic occurred in file 'src/components/app.rs' at line 80"""cause = "At the disco"method = "Panic"backtrace = """
   0: 0x10448f5f8 - __mh_execute_header   1: 0x1044a43c8 - __mh_execute_header   2: 0x1044a01ac - __mh_execute_header   3: 0x10446f8c0 - __mh_execute_header   4: 0x1044ac850 - __mh_execute_header"""
```

In debug mode, the stacktrace is as descriptive as earlier.

## Configuration

[Section titled “Configuration”](#configuration)

You can mix and match these different panic handlers, using `better-panic` for debug builds and
`color-eyre` and `human-panic` for release builds. The code below also prints the `color-eyre`
stacktrace to `log::error!` for good measure (after striping ansi escape sequences).

Terminal window

```
cargo add color-eyre human-panic libc better-panic strip-ansi-escapes
```

Here’s code you can copy paste into your project (if you use the
[`Tui`](./terminal-and-event-handler/) struct to handle terminal exits):

```
pub fn initialize_panic_handler() -> Result&#x3C;()> {  let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default()    .panic_section(format!("This is a bug. Consider reporting it at {}", env!("CARGO_PKG_REPOSITORY")))    .display_location_section(true)    .display_env_section(true)    .into_hooks();  eyre_hook.install()?;  std::panic::set_hook(Box::new(move |panic_info| {    if let Ok(t) = crate::tui::Tui::new() {      if let Err(r) = t.exit() {        error!("Unable to exit Terminal: {:?}", r);      }    }
    let msg = format!("{}", panic_hook.panic_report(panic_info));    #[cfg(not(debug_assertions))]    {        eprintln!("{msg}");        use human_panic::{handle_dump, print_msg, Metadata};        let author = format!("authored by {}", env!("CARGO_PKG_AUTHORS"));        let support = format!(            "You can open a support request at {}",            env!("CARGO_PKG_REPOSITORY")        );        let meta = Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))            .authors(author)            .support(support);
        let file_path = handle_dump(&#x26;meta, panic_info);        print_msg(file_path, &#x26;meta).expect("human-panic: printing error message to console failed");    }    log::error!("Error: {}", strip_ansi_escapes::strip_str(msg));
    #[cfg(debug_assertions)]    {      // Better Panic stacktrace that is only enabled when debugging.      better_panic::Settings::auto()        .most_recent_first(false)        .lineno_suffix(true)        .verbosity(better_panic::Verbosity::Full)        .create_panic_handler()(panic_info);    }
    std::process::exit(libc::EXIT_FAILURE);  }));  Ok(())}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/apps/better-panic.md)

 [Previous color_eyre Error Hooks](/recipes/apps/color-eyre/) [Next Migrate from tui-rs](/recipes/apps/migrate-from-tui-rs/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
