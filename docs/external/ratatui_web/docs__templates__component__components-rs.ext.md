----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /templates/component/components-rs
- Keywords: ratatui, rust, tui, terminal ui, docs, templates, component, components rs
- Summary: In `components/mod.rs`, we implement a `trait` called `Component`:
----

Source: https://ratatui.rs/templates/component/components-rs

# Components.rs

In `components/mod.rs`, we implement a `trait` called `Component`:

```
pub trait Component {    /// Register an action handler that can send actions for processing if necessary.    ///    /// # Arguments    ///    /// * `tx` - An unbounded sender that can send actions.    ///    /// # Returns    ///    /// * `Result&#x3C;()>` - An Ok result or an error.    fn register_action_handler(&#x26;mut self, tx: UnboundedSender&#x3C;Action>) -> Result&#x3C;()> {        let _ = tx; // to appease clippy        Ok(())    }    /// Register a configuration handler that provides configuration settings if necessary.    ///    /// # Arguments    ///    /// * `config` - Configuration settings.    ///    /// # Returns    ///    /// * `Result&#x3C;()>` - An Ok result or an error.    fn register_config_handler(&#x26;mut self, config: Config) -> Result&#x3C;()> {        let _ = config; // to appease clippy        Ok(())    }    /// Initialize the component with a specified area if necessary.    ///    /// # Arguments    ///    /// * `area` - Rectangular area to initialize the component within.    ///    /// # Returns    ///    /// * `Result&#x3C;()>` - An Ok result or an error.    fn init(&#x26;mut self, area: Size) -> Result&#x3C;()> {        let _ = area; // to appease clippy        Ok(())    }    /// Handle incoming events and produce actions if necessary.    ///    /// # Arguments    ///    /// * `event` - An optional event to be processed.    ///    /// # Returns    ///    /// * `Result&#x3C;Option&#x3C;Action>>` - An action to be processed or none.    fn handle_events(&#x26;mut self, event: Option&#x3C;Event>) -> Result&#x3C;Option&#x3C;Action>> {        let action = match event {            Some(Event::Key(key_event)) => self.handle_key_event(key_event)?,            Some(Event::Mouse(mouse_event)) => self.handle_mouse_event(mouse_event)?,            _ => None,        };        Ok(action)    }    /// Handle key events and produce actions if necessary.    ///    /// # Arguments    ///    /// * `key` - A key event to be processed.    ///    /// # Returns    ///    /// * `Result&#x3C;Option&#x3C;Action>>` - An action to be processed or none.    fn handle_key_event(&#x26;mut self, key: KeyEvent) -> Result&#x3C;Option&#x3C;Action>> {        let _ = key; // to appease clippy        Ok(None)    }    /// Handle mouse events and produce actions if necessary.    ///    /// # Arguments    ///    /// * `mouse` - A mouse event to be processed.    ///    /// # Returns    ///    /// * `Result&#x3C;Option&#x3C;Action>>` - An action to be processed or none.    fn handle_mouse_event(&#x26;mut self, mouse: MouseEvent) -> Result&#x3C;Option&#x3C;Action>> {        let _ = mouse; // to appease clippy        Ok(None)    }    /// Update the state of the component based on a received action. (REQUIRED)    ///    /// # Arguments    ///    /// * `action` - An action that may modify the state of the component.    ///    /// # Returns    ///    /// * `Result&#x3C;Option&#x3C;Action>>` - An action to be processed or none.    fn update(&#x26;mut self, action: Action) -> Result&#x3C;Option&#x3C;Action>> {        let _ = action; // to appease clippy        Ok(None)    }    /// Render the component on the screen. (REQUIRED)    ///    /// # Arguments    ///    /// * `f` - A frame used for rendering.    /// * `area` - The area in which the component should be drawn.    ///    /// # Returns    ///    /// * `Result&#x3C;()>` - An Ok result or an error.    fn draw(&#x26;mut self, frame: &#x26;mut Frame, area: Rect) -> Result&#x3C;()>;}
```

I personally like keeping the functions for `handle_events` (i.e. event -> action mapping),
`dispatch` (i.e. action -> state update mapping) and `render` (i.e. state -> drawing mapping) all in
one file for each component of my application.

Full code for the `components.rs` file is:

```
use color_eyre::Result;use crossterm::event::{KeyEvent, MouseEvent};use ratatui::{    Frame,    layout::{Rect, Size},};use tokio::sync::mpsc::UnboundedSender;
use crate::{action::Action, config::Config, tui::Event};
pub mod fps;pub mod home;
/// `Component` is a trait that represents a visual and interactive element of the user interface.////// Implementors of this trait can be registered with the main application loop and will be able to/// receive events, update state, and be rendered on the screen.pub trait Component {    /// Register an action handler that can send actions for processing if necessary.    ///    /// # Arguments    ///    /// * `tx` - An unbounded sender that can send actions.    ///    /// # Returns    ///    /// * `Result&#x3C;()>` - An Ok result or an error.    fn register_action_handler(&#x26;mut self, tx: UnboundedSender&#x3C;Action>) -> Result&#x3C;()> {        let _ = tx; // to appease clippy        Ok(())    }    /// Register a configuration handler that provides configuration settings if necessary.    ///    /// # Arguments    ///    /// * `config` - Configuration settings.    ///    /// # Returns    ///    /// * `Result&#x3C;()>` - An Ok result or an error.    fn register_config_handler(&#x26;mut self, config: Config) -> Result&#x3C;()> {        let _ = config; // to appease clippy        Ok(())    }    /// Initialize the component with a specified area if necessary.    ///    /// # Arguments    ///    /// * `area` - Rectangular area to initialize the component within.    ///    /// # Returns    ///    /// * `Result&#x3C;()>` - An Ok result or an error.    fn init(&#x26;mut self, area: Size) -> Result&#x3C;()> {        let _ = area; // to appease clippy        Ok(())    }    /// Handle incoming events and produce actions if necessary.    ///    /// # Arguments    ///    /// * `event` - An optional event to be processed.    ///    /// # Returns    ///    /// * `Result&#x3C;Option&#x3C;Action>>` - An action to be processed or none.    fn handle_events(&#x26;mut self, event: Option&#x3C;Event>) -> Result&#x3C;Option&#x3C;Action>> {        let action = match event {            Some(Event::Key(key_event)) => self.handle_key_event(key_event)?,            Some(Event::Mouse(mouse_event)) => self.handle_mouse_event(mouse_event)?,            _ => None,        };        Ok(action)    }    /// Handle key events and produce actions if necessary.    ///    /// # Arguments    ///    /// * `key` - A key event to be processed.    ///    /// # Returns    ///    /// * `Result&#x3C;Option&#x3C;Action>>` - An action to be processed or none.    fn handle_key_event(&#x26;mut self, key: KeyEvent) -> Result&#x3C;Option&#x3C;Action>> {        let _ = key; // to appease clippy        Ok(None)    }    /// Handle mouse events and produce actions if necessary.    ///    /// # Arguments    ///    /// * `mouse` - A mouse event to be processed.    ///    /// # Returns    ///    /// * `Result&#x3C;Option&#x3C;Action>>` - An action to be processed or none.    fn handle_mouse_event(&#x26;mut self, mouse: MouseEvent) -> Result&#x3C;Option&#x3C;Action>> {        let _ = mouse; // to appease clippy        Ok(None)    }    /// Update the state of the component based on a received action. (REQUIRED)    ///    /// # Arguments    ///    /// * `action` - An action that may modify the state of the component.    ///    /// # Returns    ///    /// * `Result&#x3C;Option&#x3C;Action>>` - An action to be processed or none.    fn update(&#x26;mut self, action: Action) -> Result&#x3C;Option&#x3C;Action>> {        let _ = action; // to appease clippy        Ok(None)    }    /// Render the component on the screen. (REQUIRED)    ///    /// # Arguments    ///    /// * `f` - A frame used for rendering.    /// * `area` - The area in which the component should be drawn.    ///    /// # Returns    ///    /// * `Result&#x3C;()>` - An Ok result or an error.    fn draw(&#x26;mut self, frame: &#x26;mut Frame, area: Rect) -> Result&#x3C;()>;}
```

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/templates/component/components-rs.md)

 [Previous Action.rs](/templates/component/action-rs/) [Next Components/home.rs](/templates/component/components-home-rs/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
