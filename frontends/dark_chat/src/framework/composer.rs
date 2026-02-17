#[derive(Debug, Clone, Default)]
pub struct ComposerState {
    draft: String,
    cursor: usize,
    composing: bool,
}

impl ComposerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self) {
        self.composing = true;
        self.cursor = self.draft.chars().count();
    }

    pub fn close(&mut self) {
        self.composing = false;
    }

    pub fn clear(&mut self) {
        self.draft.clear();
        self.cursor = 0;
    }

    pub fn insert_char(&mut self, ch: char) {
        let mut chars = self.draft.chars().collect::<Vec<_>>();
        let index = self.cursor.min(chars.len());
        chars.insert(index, ch);
        self.draft = chars.into_iter().collect();
        self.cursor += 1;
    }

    pub fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }

        self.cursor -= 1;
        let mut chars = self.draft.chars().collect::<Vec<_>>();
        if self.cursor < chars.len() {
            chars.remove(self.cursor);
            self.draft = chars.into_iter().collect();
        }
    }

    pub fn draft(&self) -> &str {
        &self.draft
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn composing(&self) -> bool {
        self.composing
    }
}
