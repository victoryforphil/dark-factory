use insta::assert_snapshot;
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use serde_json::json;

use crate::app::App;
use crate::models::{ActorRow, DashboardSnapshot, ProductRow, SubAgentRow, VariantRow};
use crate::theme::Theme;

use super::UnifiedCatalogView;

#[test]
fn graphical_tree_station_snapshot() {
    let app = build_test_app();
    let output = render_view(160, 56, |frame| {
        UnifiedCatalogView::render(frame, Rect::new(0, 0, 160, 56), &app)
    });
    assert_snapshot!("dark_tui_graphical_tree_station", output);
}

fn build_test_app() -> App {
    let mut app = App::new(".".to_string(), 5, Theme::default());
    app.apply_snapshot(snapshot());
    app
}

fn render_view(width: u16, height: u16, draw: impl FnOnce(&mut ratatui::Frame<'_>)) -> String {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).expect("terminal should initialize");

    terminal.draw(draw).expect("draw should succeed");

    let mut lines = Vec::with_capacity(height as usize);
    for y in 0..height {
        let mut line = String::with_capacity(width as usize);
        for x in 0..width {
            line.push_str(
                terminal
                    .backend()
                    .buffer()
                    .cell((x, y))
                    .expect("cell")
                    .symbol(),
            );
        }
        lines.push(line.trim_end().to_string());
    }
    lines.join("\n")
}

fn snapshot() -> DashboardSnapshot {
    DashboardSnapshot {
        products: vec![ProductRow {
            id: "prd_2o02efhjtdbr3".to_string(),
            display_name: "dark-factory".to_string(),
            locator: "@git://https://github.com/victoryforphil/dark-factory#main".to_string(),
            workspace_locator: "@local:///Users/alex/repos/vfp/dark-factory".to_string(),
            product_type: "git".to_string(),
            is_git_repo: true,
            branch: "main".to_string(),
            branches: "df/dark-factory-3ckk2y, main".to_string(),
            repo_name: "dark-factory".to_string(),
            updated_at: "2026-02-17 04:51:36".to_string(),
            status: "dirty".to_string(),
            variant_total: 4,
            variant_dirty: 1,
            variant_drift: 1,
        }],
        variants: vec![
            variant(
                "var_north",
                "prd_2o02efhjtdbr3",
                "default",
                "main",
                "dirty",
                21,
                0,
            ),
            variant(
                "var_east",
                "prd_2o02efhjtdbr3",
                "clone_0000",
                "df/dark-factory-3ckk2y",
                "clean",
                0,
                0,
            ),
            variant(
                "var_south",
                "prd_2o02efhjtdbr3",
                "clone_0001",
                "feature/ux",
                "clean",
                0,
                0,
            ),
            variant(
                "var_west",
                "prd_2o02efhjtdbr3",
                "feature/auth",
                "hotfix/auth",
                "dirty",
                3,
                1,
            ),
        ],
        actors: vec![
            actor(
                "act_a1",
                "var_north",
                "Dark TUI // dark-factory",
                "running",
                vec![
                    sub("sa_frontend", "Quick frontend", "running"),
                    sub("sa_routes", "Quick routes", "stopped"),
                ],
            ),
            actor(
                "act_a2",
                "var_east",
                "Dark TUI // dark-factory",
                "stopped",
                vec![
                    sub("sa_docs", "Docs pass", "idle"),
                    sub("sa_tests", "Test pass", "running"),
                    sub("sa_lint", "Lint sweep", "idle"),
                ],
            ),
            actor(
                "act_a3",
                "var_west",
                "Dark TUI // dark-factory",
                "idle",
                vec![
                    sub("sa_review", "Review", "running"),
                    sub("sa_verify", "Verify", "running"),
                ],
            ),
            actor(
                "act_a4",
                "var_south",
                "Dark TUI // dark-factory",
                "running",
                vec![
                    sub("sa_deploy", "Deploy", "running"),
                    sub("sa_watch", "Watch", "idle"),
                    sub("sa_sync", "Sync", "idle"),
                ],
            ),
        ],
        runtime_status: "ok".to_string(),
        last_updated: "2026-02-17 09:30:30".to_string(),
    }
}

fn variant(
    id: &str,
    product_id: &str,
    name: &str,
    branch: &str,
    git_state: &str,
    ahead: u64,
    behind: u64,
) -> VariantRow {
    VariantRow {
        id: id.to_string(),
        product_id: product_id.to_string(),
        locator: format!("@local:///Users/alex/repos/vfp/dark-factory/{name}"),
        name: name.to_string(),
        branch: branch.to_string(),
        git_state: git_state.to_string(),
        clone_status: "-".to_string(),
        clone_last_line: "-".to_string(),
        has_git: true,
        is_dirty: git_state == "dirty",
        ahead,
        behind,
        worktree: branch.to_string(),
        last_polled_at: "2026-02-17 09:30:30".to_string(),
        updated_at: "2026-02-17 09:30:30".to_string(),
    }
}

fn actor(
    id: &str,
    variant_id: &str,
    title: &str,
    status: &str,
    sub_agents: Vec<SubAgentRow>,
) -> ActorRow {
    ActorRow {
        id: id.to_string(),
        variant_id: variant_id.to_string(),
        title: title.to_string(),
        description: "Spawned from dark_tui".to_string(),
        provider: "opencode/server".to_string(),
        provider_session_id: None,
        status: status.to_string(),
        directory: "/Users/alex/repos/vfp/dark-factory".to_string(),
        connection_info: json!({}),
        sub_agents,
        created_at: "2026-02-17 09:20:02".to_string(),
        updated_at: "2026-02-17 09:20:02".to_string(),
    }
}

fn sub(id: &str, title: &str, status: &str) -> SubAgentRow {
    SubAgentRow {
        id: id.to_string(),
        parent_id: None,
        title: title.to_string(),
        status: status.to_string(),
        summary: title.to_string(),
        updated_at: "2026-02-17 09:20:02".to_string(),
        depth: 0,
    }
}
