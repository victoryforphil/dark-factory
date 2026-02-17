use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::tui::app::{App, ResizeTarget};
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

#[derive(Debug, Clone)]
pub struct ViewLayout {
    pub root: Rect,
    pub sessions: Rect,
    pub chat: Rect,
    pub runtime: Rect,
    pub chat_composer: Rect,
    pub divider_cols: Vec<u16>,
    pub narrow: bool,
}

impl MainView {
    pub fn render(frame: &mut Frame, app: &App) {
        let layout = Self::layout(frame.area(), app);
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

    pub fn layout(root: Rect, app: &App) -> ViewLayout {
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
            let top = app.body_split_narrow_top().resolve(vertical[0]);
            let divider_cols = app
                .body_split_narrow_top()
                .divider_col(vertical[0], 0)
                .map(|value| vec![value])
                .unwrap_or_default();

            if top.len() < 2 {
                return ViewLayout {
                    root,
                    sessions: vertical[0],
                    chat: vertical[0],
                    runtime: vertical[1],
                    chat_composer: chat_composer_area(vertical[0]),
                    divider_cols,
                    narrow: true,
                };
            }

            return ViewLayout {
                root,
                sessions: top[0],
                chat: top[1],
                runtime: vertical[1],
                chat_composer: chat_composer_area(top[1]),
                divider_cols,
                narrow: true,
            };
        }

        let columns = app.body_split_wide().resolve(body);
        let mut divider_cols = Vec::new();
        if let Some(col) = app.body_split_wide().divider_col(body, 0) {
            divider_cols.push(col);
        }
        if let Some(col) = app.body_split_wide().divider_col(body, 1) {
            divider_cols.push(col);
        }

        if columns.len() < 3 {
            return ViewLayout {
                root,
                sessions: body,
                chat: body,
                runtime: body,
                chat_composer: chat_composer_area(body),
                divider_cols,
                narrow: false,
            };
        }

        ViewLayout {
            root,
            sessions: columns[0],
            chat: columns[1],
            runtime: columns[2],
            chat_composer: chat_composer_area(columns[1]),
            divider_cols,
            narrow: false,
        }
    }

    pub fn divider_hit(layout: &ViewLayout, col: u16) -> Option<ResizeTarget> {
        for (index, divider_col) in layout.divider_cols.iter().copied().enumerate() {
            if divider_col.abs_diff(col) <= 1 {
                if layout.narrow {
                    return Some(ResizeTarget::NarrowTop(index));
                }

                return Some(ResizeTarget::Wide(index));
            }
        }

        None
    }

    pub fn hit_test(layout: &ViewLayout, col: u16, row: u16) -> PanelHit {
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
