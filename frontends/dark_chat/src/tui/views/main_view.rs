use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::tui::app::App;
use crate::tui::panels::{
    ChatPanel, FooterPanel, HeaderPanel, KeyBarPanel, SessionsPanel, StatusPanel,
};

pub struct MainView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelHit {
    Sessions,
    Chat,
    ChatComposer,
    Runtime,
    Other,
}

#[derive(Debug, Clone, Copy)]
pub struct ViewLayout {
    pub root: Rect,
    pub sessions: Rect,
    pub chat: Rect,
    pub runtime: Rect,
    pub chat_composer: Rect,
}

impl MainView {
    pub fn render(frame: &mut Frame, app: &App) {
        let layout = Self::layout(frame.area());
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(layout.root);

        HeaderPanel::render(frame, rows[0], app);
        KeyBarPanel::render(frame, rows[1], app);
        Self::render_body(frame, layout, app);
        FooterPanel::render(frame, rows[3], app);
    }

    pub fn layout(root: Rect) -> ViewLayout {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(root);

        let body = rows[2];

        if body.width < 120 {
            let vertical = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(8), Constraint::Length(9)])
                .split(body);

            let top = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(36), Constraint::Percentage(64)])
                .split(vertical[0]);

            return ViewLayout {
                root,
                sessions: top[0],
                chat: top[1],
                runtime: vertical[1],
                chat_composer: chat_composer_area(top[1]),
            };
        }

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(24),
                Constraint::Percentage(54),
                Constraint::Percentage(22),
            ])
            .split(body);

        ViewLayout {
            root,
            sessions: columns[0],
            chat: columns[1],
            runtime: columns[2],
            chat_composer: chat_composer_area(columns[1]),
        }
    }

    pub fn hit_test(layout: ViewLayout, col: u16, row: u16) -> PanelHit {
        if contains(layout.chat_composer, col, row) {
            return PanelHit::ChatComposer;
        }

        if contains(layout.sessions, col, row) {
            return PanelHit::Sessions;
        }

        if contains(layout.chat, col, row) {
            return PanelHit::Chat;
        }

        if contains(layout.runtime, col, row) {
            return PanelHit::Runtime;
        }

        PanelHit::Other
    }

    fn render_body(frame: &mut Frame, layout: ViewLayout, app: &App) {
        SessionsPanel::render(frame, layout.sessions, app);
        ChatPanel::render(frame, layout.chat, app);
        StatusPanel::render(frame, layout.runtime, app);
    }
}

fn contains(area: Rect, col: u16, row: u16) -> bool {
    col >= area.x
        && col < area.x.saturating_add(area.width)
        && row >= area.y
        && row < area.y.saturating_add(area.height)
}

fn chat_composer_area(chat_area: Rect) -> Rect {
    let inner = Rect {
        x: chat_area.x.saturating_add(1),
        y: chat_area.y.saturating_add(1),
        width: chat_area.width.saturating_sub(2),
        height: chat_area.height.saturating_sub(2),
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(4),
            Constraint::Length(5),
        ])
        .split(inner);

    chunks[2]
}
