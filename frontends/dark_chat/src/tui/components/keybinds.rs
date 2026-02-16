use dark_tui_components::KeyBind;

pub const KEY_BINDS: [KeyBind; 11] = [
    KeyBind::new("q", "quit"),
    KeyBind::new("tab", "focus panel"),
    KeyBind::new("j/k", "focus nav/scroll"),
    KeyBind::new("r", "refresh"),
    KeyBind::new("n", "new session"),
    KeyBind::new("a", "agent"),
    KeyBind::new("m", "model picker"),
    KeyBind::new("c", "compose"),
    KeyBind::new("enter", "send; S-enter nl"),
    KeyBind::new("esc", "cancel"),
    KeyBind::new("h", "help"),
];
