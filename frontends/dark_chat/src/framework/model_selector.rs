use dark_tui_components::{next_index, previous_index};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectorKind {
    Model,
    Agent,
}

#[derive(Debug, Clone)]
pub struct ItemSelector {
    pub open: bool,
    pub query: String,
    pub selected: usize,
    pub raw_mode: bool,
    pub raw_input: String,
    pub anchor_col: Option<u16>,
}

impl ItemSelector {
    pub fn new() -> Self {
        Self {
            open: false,
            query: String::new(),
            selected: 0,
            raw_mode: false,
            raw_input: String::new(),
            anchor_col: None,
        }
    }

    pub fn open(&mut self) {
        self.open = true;
        self.query.clear();
        self.selected = 0;
    }

    pub fn close(&mut self) {
        self.open = false;
        self.query.clear();
        self.selected = 0;
        self.raw_mode = false;
        self.raw_input.clear();
        self.anchor_col = None;
    }

    pub fn insert_query_char(&mut self, ch: char) {
        if self.raw_mode {
            self.raw_input.push(ch);
        } else {
            self.query.push(ch);
            self.selected = 0;
        }
    }

    pub fn backspace_query(&mut self) {
        if self.raw_mode {
            self.raw_input.pop();
        } else {
            self.query.pop();
            self.selected = 0;
        }
    }

    pub fn clear_query(&mut self) {
        if self.raw_mode {
            self.raw_input.clear();
        } else {
            self.query.clear();
            self.selected = 0;
        }
    }

    pub fn move_up(&mut self, len: usize) {
        if len == 0 {
            self.selected = 0;
            return;
        }

        self.selected = previous_index(self.selected, len);
    }

    pub fn move_down(&mut self, len: usize) {
        if len == 0 {
            self.selected = 0;
            return;
        }

        self.selected = next_index(self.selected, len);
    }

    pub fn set_selected(&mut self, index: usize, len: usize) {
        if len == 0 {
            self.selected = 0;
            return;
        }

        self.selected = index.min(len.saturating_sub(1));
    }

    pub fn filtered_items<'a>(&self, items: &'a [String]) -> Vec<&'a str> {
        let filter = self.query.trim().to_ascii_lowercase();
        items
            .iter()
            .filter(|item| filter.is_empty() || item.to_ascii_lowercase().contains(&filter))
            .map(String::as_str)
            .collect()
    }

    pub fn confirm(&self, items: &[String]) -> Option<String> {
        if self.raw_mode {
            return Some(self.raw_input.trim().to_string()).filter(|value| !value.is_empty());
        }

        let filtered = self.filtered_items(items);
        filtered
            .get(self.selected)
            .map(|value| (*value).to_string())
    }

    pub fn toggle_raw_mode(&mut self) {
        self.raw_mode = !self.raw_mode;
    }
}
