----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /recipes/apps/release-your-app
- Keywords: ratatui, rust, tui, terminal ui, docs, recipes, apps, release your app
- Summary: At some point you’ll probably want to release your app to the public. Here are some guidelines that
----

Source: https://ratatui.rs/recipes/apps/release-your-app

# Releasing Your App

At some point you’ll probably want to release your app to the public. Here are some guidelines that
capture some advice regularly given on apps that are put in the showcase channel of the Ratatui
Discord server / forum. This is not a comprehensive list, but it should help you get started.

## Release Checklist

[Section titled “Release Checklist”](#release-checklist)

- Commit your `cargo.lock` file.

- Add `--locked` to your `cargo install` command. This makes sure that your users will install the same version of your dependencies that you used to build your app.

- Add a `README.md` file to your project. This file should contain: A short description of your app.

- A list of hotkeys and their functions.

- A screenshot of your app in action.

- Add a `LICENSE` file to your project. This file should contain the license you are using for your app. The most common licenses are MIT and Apache 2.0. You can use [ChooseALicense](https://choosealicense.com/) to help you choose a license.

- Enable additional [optimizations](https://ratatui.rs/recipes/apps/release-your-app/#optimizations) in your `Cargo.toml` file.

- Consider using `Release-plz` to automate your GitHub releases. This makes doing a release as easy as clicking merge on an automatically generated PR.

- Submit your app to the [Awesome Ratatui](https://github.com/ratatui-org/awesome-ratatui) list, the [Ratatui Discord](https://discord.gg/pMCEU9hNEj), and the [Ratatui Forum](https://forum.ratatui.rs). This is a great way to get feedback on your app and to get it in front of more users.

## Screenshots

[Section titled “Screenshots”](#screenshots)

Don’t forget to add a screenshot / gif of your app in action. This will help users understand what
your app does and how it looks. We recommend using a tool like [VHS](https://github.com/charmbracelet/vhs) to automate the process of
creating screenshots and gifs. See the `.tape` files in the [Ratatui repository] for examples of how
we use VHS to create screenshots and gifs for our all the examples and widgets.

Some tips for creating good screenshots:

TL;DR: Use [VHS](https://github.com/charmbracelet/vhs), `Set Width 1200`, `Set Theme "Aardvark Blue"`, `Sleep 2s`, `Hide`/`Show` around
CLI command to run the app,

- Use a dark background for your terminal. Don’t flashbang devs with light screens.

- Choose a good color scheme that shows off your app. Aardvark Blue is the choice we use in Ratatui, but Catppuccin and Solarized are also good options. There are many bad options :D

- Use an image width 1200px or less. This works well with the GitHub UI and the crates.io page.

- Use a font size that is similar that when rendered is approximately the same size as the text on the github README. The default VHS font size is good here.

- Skip the command line in the screenshot by using the `Hide` and `Show` commands in VHS. This will make your screenshots look cleaner and more professional.

- Wait around 2 seconds after making some changes to let the viewier read and understand the changes. This is especially important for gifs with a lot of information changing on each screen.

- Avoid storing the screenshot / gifs in your repo. This tends to bloat the repo and make it harder to clone. Instead, use VHS to publish the screenshots (`vhs publish`), or store them in a PR comment, an image hosting service, or your own website.

- Try to get to the meat of your app as quickly as possible. Don’t waste time on the loading screen or splash screen. This is especially important for gifs, where you have a limited amount of time to show off your app.

## Optimizations

[Section titled “Optimizations”](#optimizations)

Make sure you enable additional compiler optimizations for the release build. This will help reduce
the size of the resulting binary. Add the following lines to your `Cargo.toml` file:

- ``` [profile.release]codegen-units = 1 # Allows compiler to perform better optimization.lto = true # Enables Link-time Optimization.opt-level = "s" # Prioritizes small binary size. Use `3` if you prefer speed.strip = true # Ensures debug symbols are removed. ``` ### References [Section titled “References”](#references) [codegen-units](https://doc.rust-lang.org/cargo/reference/profiles.html#codegen-units): Tweaks a tradeoff between compile times and compile time optimizations.

- [lto](https://doc.rust-lang.org/cargo/reference/profiles.html#lto): Enables Link-time Optimization.

- [opt-level](https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level): Determines the focus of the compiler in optimizations. Use `3` to optimize performance, `z` to optimize for size, and `s` for something in-between.

- [strip](https://doc.rust-lang.org/cargo/reference/profiles.html#strip): Strip either symbols or debuginfo from a binary.

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/recipes/apps/release-your-app.md)

 [Previous Spawn External Editor (Vim)](/recipes/apps/spawn-vim/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
