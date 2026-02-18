use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Clear, Paragraph, Wrap};

use crate::app::App;

use dark_tui_components::PaneBlockComponent;

pub(crate) struct SshPanel;

impl SshPanel {
    pub(crate) fn render(frame: &mut Frame, area: Rect, app: &App) {
        let theme = app.theme();
        let popup = centered_rect(area, 82, 72);

        frame.render_widget(Clear, popup);

        let block = PaneBlockComponent::build("SSH Hosts + Port Forwards", true, theme);
        let inner = block.inner(popup);
        frame.render_widget(block, popup);

        let selected_forward = app.ssh_panel_selected_forward_index().unwrap_or(0);
        let selected_tmux = app.ssh_panel_selected_tmux_index().unwrap_or(0);
        let selected_host = app.ssh_panel_selected_host_index().unwrap_or(0);
        let host_focus = app.ssh_panel_focus_is_hosts();
        let tmux_focus = app.ssh_panel_focus_is_tmux();
        let mut lines = vec![Line::from(Span::styled(
            format!("Hosts:{}", if host_focus { " [focus]" } else { "" }),
            Style::default().fg(if host_focus {
                theme.entity_variant
            } else {
                theme.text_muted
            }),
        ))];

        if app.ssh_hosts().is_empty() {
            lines.push(Line::from(Span::styled(
                "  (no hosts discovered)",
                Style::default().fg(theme.text_muted),
            )));
        } else {
            for (index, host) in app.ssh_hosts().iter().enumerate() {
                let marker = if index == selected_host { ">" } else { " " };
                let style = if index == selected_host {
                    Style::default().fg(theme.entity_variant)
                } else {
                    Style::default().fg(theme.text_secondary)
                };
                lines.push(Line::from(Span::styled(
                    format!(
                        "  {marker} {} [{}] key={} host={} user={} port={} path={}",
                        host.label,
                        host.source,
                        host.key,
                        host.host,
                        host.user,
                        host.port,
                        host.default_path
                    ),
                    style,
                )));
            }
        }

        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            format!(
                "Port Forward Presets:{}",
                if tmux_focus { "" } else { " [focus]" }
            ),
            Style::default().fg(if tmux_focus {
                theme.text_muted
            } else {
                theme.entity_variant
            }),
        )));

        if app.ssh_port_forwards().is_empty() {
            lines.push(Line::from(Span::styled(
                "  (configure [ssh].portForwards in config.toml)",
                Style::default().fg(theme.text_muted),
            )));
        } else {
            for (index, preset) in app.ssh_port_forwards().iter().enumerate() {
                let marker = if index == selected_forward { ">" } else { " " };
                let style = if index == selected_forward {
                    Style::default().fg(theme.entity_variant)
                } else {
                    Style::default().fg(theme.text_primary)
                };
                let host = if preset.host.trim().is_empty() {
                    "(host required)"
                } else {
                    preset.host.as_str()
                };
                lines.push(Line::from(Span::styled(
                    format!(
                        "  {marker} {} {} -> {}:{} (host={})",
                        preset.name,
                        preset.local_port,
                        preset.remote_host,
                        preset.remote_port,
                        host
                    ),
                    style,
                )));
                if preset.description != "-" {
                    lines.push(Line::from(Span::styled(
                        format!("      {}", preset.description),
                        Style::default().fg(theme.text_muted),
                    )));
                }
            }
        }

        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            "Active SSH Forwards:",
            Style::default().fg(theme.text_muted),
        )));
        if app.ssh_active_forwards().is_empty() {
            lines.push(Line::from(Span::styled(
                "  (no active tunnel sessions)",
                Style::default().fg(theme.text_muted),
            )));
        } else {
            for session in app.ssh_active_forwards() {
                lines.push(Line::from(Span::styled(
                    format!(
                        "  - {} cmd={} windows={} attached={}",
                        session.name,
                        session.current_command,
                        session.windows,
                        if session.attached { "yes" } else { "no" }
                    ),
                    Style::default().fg(theme.text_secondary),
                )));
            }
        }

        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            format!("tmux Sessions:{}", if tmux_focus { " [focus]" } else { "" }),
            Style::default().fg(if tmux_focus {
                theme.entity_variant
            } else {
                theme.text_muted
            }),
        )));
        if app.tmux_sessions().is_empty() {
            lines.push(Line::from(Span::styled(
                "  (tmux not running or no sessions)",
                Style::default().fg(theme.text_muted),
            )));
        } else {
            for (index, session) in app.tmux_sessions().iter().enumerate() {
                let marker = if index == selected_tmux { ">" } else { " " };
                let style = if index == selected_tmux {
                    Style::default().fg(theme.entity_variant)
                } else {
                    Style::default().fg(theme.text_secondary)
                };
                lines.push(Line::from(Span::styled(
                    format!(
                        "  {marker} {} cmd={} windows={} attached={}",
                        session.name,
                        session.current_command,
                        session.windows,
                        if session.attached { "yes" } else { "no" }
                    ),
                    style,
                )));
            }
        }

        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            "Tab: focus list   Enter: action   c/a: local tmux copy/attach",
            Style::default().fg(theme.text_muted),
        )));
        lines.push(Line::from(Span::styled(
            "g: ensure remote agent tmux   o: copy remote attach   A: remote attach",
            Style::default().fg(theme.text_muted),
        )));
        lines.push(Line::from(Span::styled(
            "Up/Down: select focused row   Esc: close",
            Style::default().fg(theme.text_muted),
        )));

        let content = Paragraph::new(lines).wrap(Wrap { trim: false });
        frame.render_widget(content, inner);
    }
}

fn centered_rect(area: Rect, width_percent: u16, height_percent: u16) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height_percent) / 2),
            Constraint::Percentage(height_percent),
            Constraint::Percentage((100 - height_percent) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - width_percent) / 2),
            Constraint::Percentage(width_percent),
            Constraint::Percentage((100 - width_percent) / 2),
        ])
        .split(vertical[1])[1]
}
