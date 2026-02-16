----
## External Docs Snapshot // ratatui_web

- Captured: 2026-02-16T10:16:59.527Z
- Source root: https://ratatui.rs/
- Source page: /tutorials/json-editor/app
- Keywords: ratatui, rust, tui, terminal ui, docs, tutorials, json editor, app
- Summary: As we saw in the previous section, a common model for smaller `ratatui` applications is to have one
----

Source: https://ratatui.rs/tutorials/json-editor/app

# App.rs

As we saw in the previous section, a common model for smaller `ratatui` applications is to have one
application state struct called `App` or some variant of that name. We will be using this paradigm
in this application as well.

This struct will contain all of our “persistent” data and will be passed to any function that needs
to know the current state of the application.

## Application modes

[Section titled “Application modes”](#application-modes)

It is useful to think about the several “modes” that your application can be in. Thinking in “modes”
will make it easier to segregate everything from what window is getting drawn, to what keybinds to
listen for.

We will be using the application’s state to track two things:

- what screen the user is seeing,

- which box should be highlighted, the “key” or “value” (this only applies when the user is editing a key-value pair).

### Current Screen Enum

[Section titled “Current Screen Enum”](#current-screen-enum)

In this tutorial application, we will have three “screens”:

- `Main`: the main summary screen showing all past key-value pairs entered

- `Editing`: the screen shown when the user wishes to create a new key-value pair

- `Exiting`: displays a prompt asking if the user wants to output the key-value pairs they have entered.

We represent these possible modes with a simple enum:

- ``` pub enum CurrentScreen { Main, Editing, Exiting,} ``` ### Currently Editing Enum [Section titled “Currently Editing Enum”](#currently-editing-enum) As you may already know, `ratatui` does not automatically redraw the screen[1](#user-content-fn-note). `ratatui` also does not remember anything about what it drew last frame. This means that the programmer is responsible for handling all state and updating widgets to reflect changes. In this case, we will allow the user to input two strings in the `Editing` mode - a key and a value. The programmer is responsible for knowing which the user is trying to edit. For this purpose, we will create another enum for our application state called `CurrentlyEditing` to keep track of which field the user is currently entering: ``` pub enum CurrentlyEditing { Key, Value,} ``` ## The full application state [Section titled “The full application state”](#the-full-application-state) Now that we have enums to help us track where the user is, we will create the struct that actually stores this data which can be passed around where it is needed. ``` pub struct App { pub key_input: String, // the currently being edited json key. pub value_input: String, // the currently being edited json value. pub pairs: HashMap&#x3C;String, String>, // The representation of our key and value pairs with serde Serialize support pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered. pub currently_editing: Option&#x3C;CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.} ``` ## Helper functions [Section titled “Helper functions”](#helper-functions) While we could simply keep our application state as simply a holder of values, we can also create a few helper functions which will make our life easier elsewhere. Of course, these functions should only affect the application state itself, and nothing outside of it. ### `new()` [Section titled “new()”](#new) We will be adding this function simply to make creating the state easier. While this could be avoided by specifying it all in the instantiation of the variable, doing it here allows for easy to change universal defaults for the state. ``` impl App { pub fn new() -> App { App { key_input: String::new(), value_input: String::new(), pairs: HashMap::new(), current_screen: CurrentScreen::Main, currently_editing: None, } } // --snip-- ``` ### `save_key_value()` [Section titled “save_key_value()”](#save_key_value) This function will be called when the user saves a key-value pair in the editor. It adds the two stored variables to the key-value pairs `HashMap`, and resets the status of all of the editing variables. ``` // --snip-- pub fn save_key_value(&#x26;mut self) { self.pairs .insert(self.key_input.clone(), self.value_input.clone()); self.key_input = String::new(); self.value_input = String::new(); self.currently_editing = None; } // --snip-- ``` ### `toggle_editing()` [Section titled “toggle_editing()”](#toggle_editing) Sometimes it is easier to put simple logic into a convenience function so we don’t have to worry about it in the main code block. `toggle_editing` is one of those cases. All we are doing, is checking if something is currently being edited, and if it is, swapping between editing the Key and Value fields. ``` // --snip-- pub fn toggle_editing(&#x26;mut self) { if let Some(edit_mode) = &#x26;self.currently_editing { match edit_mode { CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value), CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key), }; } else { self.currently_editing = Some(CurrentlyEditing::Key); } } // --snip-- ``` ### `print_json()` [Section titled “print_json()”](#print_json) Finally, is another convenience function to print out the serialized json from all of our key-value pairs. ``` // --snip-- pub fn print_json(&#x26;self) -> serde_json::Result&#x3C;()> { let output = serde_json::to_string(&#x26;self.pairs)?; println!("{output}"); Ok(()) } // --snip-- ``` ## Footnotes [Section titled “Footnotes”](#footnote-label) In ratatui, every frame draws the UI anew. See the [Rendering section](/concepts/rendering/) for more information. [↩](#user-content-fnref-note)

 [Edit page](https://github.com/ratatui/ratatui-website/edit/main/src/content/docs/tutorials/json-editor/app.md)

 [Previous JSON Editor](/tutorials/json-editor/) [Next Main.rs](/tutorials/json-editor/main/)

----
## Notes / Comments / Lessons

- Collection method: sitemap-index-first discovery with direct HTML fallback support.
- Conversion path: direct HTML fallback parser.
- This file is one page-level external snapshot in markdown `.ext.md` format.
----
