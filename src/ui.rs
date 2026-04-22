use crate::app::{App, FavoriteRow, Mode, ToastKind, View};
use chrono::{Local, LocalResult, TimeZone};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Frame;

const HELP_TEXT: &str = "hline keybindings

Views: F toggle history/favorites
History: Space toggle, a select shown, c clear, f save selected/current as favorite
Favorites: y copy whole block, Y copy current line, J/K or Shift+Up/Down jump blocks
Accept: Enter print selected/current item to stdout and quit
Search: / enter search, Enter confirm, Esc exit
Search edit: Backspace, Ctrl+w delete word, Ctrl+u clear
Time filter: after:YYYY-MM-DD before:YYYY-MM-DD on:YYYY-MM-DD
Sorting: s cycle sort mode, S reverse sort direction (history only)
Copy: y copy selected/current item
Quit: q

Close help: Esc or ?";

pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.size();
    let chunks = split_layout(area, app.mode);

    render_list(frame, app, chunks[0]);
    render_status(frame, app, chunks[1]);

    if matches!(app.mode, Mode::Search) {
        render_search_prompt(frame, app, chunks[2]);
    }

    if app.show_help {
        render_help(frame, area);
    }
}

fn split_layout(area: Rect, mode: Mode) -> Vec<Rect> {
    if matches!(mode, Mode::Search) {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(area)
            .to_vec()
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area)
            .to_vec()
    }
}

fn render_list(frame: &mut Frame, app: &mut App, list_area: Rect) {
    let list_block = Block::default()
        .title(app.list_title())
        .borders(Borders::ALL);
    let viewport_h = list_area.height.saturating_sub(2) as usize;
    app.set_viewport_height(viewport_h);

    let total = app.visible_count();
    if total == 0 {
        let message = match app.view {
            View::History => "No matches",
            View::Favorites => "No favorites",
        };
        let empty = Paragraph::new(message)
            .block(list_block)
            .alignment(Alignment::Center);
        frame.render_widget(empty, list_area);
        return;
    }

    let cursor = app.active_cursor();
    let scroll = centered_scroll(cursor, total, viewport_h);
    let end = (scroll + viewport_h.max(1)).min(total);
    let show_timestamps = app.has_timestamps();

    let items = match app.view {
        View::History => render_history_rows(app, scroll, end, show_timestamps),
        View::Favorites => render_favorite_rows(app, scroll, end),
    };

    let mut state = ListState::default();
    state.select(Some(cursor.saturating_sub(scroll)));

    let list = List::new(items)
        .block(list_block)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, list_area, &mut state);
}

fn render_history_rows(
    app: &App,
    scroll: usize,
    end: usize,
    show_timestamps: bool,
) -> Vec<ListItem<'static>> {
    let mut items = Vec::with_capacity(end.saturating_sub(scroll));
    for pos in scroll..end {
        let idx = app.filtered[pos];
        let entry = &app.all[idx];
        let checked = if app.selected.contains(&entry.id) {
            "[x]"
        } else {
            "[ ]"
        };
        items.push(ListItem::new(format_history_label(
            checked,
            &entry.cmd,
            entry.timestamp,
            show_timestamps,
        )));
    }
    items
}

fn render_favorite_rows(app: &App, scroll: usize, end: usize) -> Vec<ListItem<'static>> {
    let mut items = Vec::with_capacity(end.saturating_sub(scroll));
    for pos in scroll..end {
        let row = app.favorite_rows[pos];
        items.push(ListItem::new(format_favorite_label(app, row)));
    }
    items
}

fn format_favorite_label(app: &App, row: FavoriteRow) -> String {
    let Some(block) = app.favorite_block(row.block_index) else {
        return "[ ] missing favorite".to_string();
    };
    match row.line_index {
        None => {
            let suffix = if block.lines.len() == 1 { "" } else { "s" };
            format!(
                "[ ] favorite {} ({} line{})",
                block.id,
                block.lines.len(),
                suffix
            )
        }
        Some(line_index) => format!("    [ ] {}", block.lines[line_index]),
    }
}

fn render_status(frame: &mut Frame, app: &App, status_area: Rect) {
    let mut status = match app.view {
        View::History => format!(
            "{} | filter: {} | {} total | {} shown | {} selected | {}",
            app.status_context_text(),
            app.status_filter_text(),
            app.total_count(),
            app.visible_count(),
            app.selected.len(),
            app.status_hint_text(),
        ),
        View::Favorites => format!(
            "{} | filter: {} | {} rows shown | {}",
            app.status_context_text(),
            app.status_filter_text(),
            app.visible_count(),
            app.status_hint_text(),
        ),
    };

    let mut status_style = Style::default().bg(Color::DarkGray).fg(Color::White);
    if let Some(toast) = &app.toast {
        status.push_str(" | ");
        status.push_str(&toast.message);
        if toast.kind == ToastKind::Error {
            status_style = status_style.bg(Color::Red);
        }
    }

    let status_bar = Paragraph::new(status)
        .style(status_style)
        .wrap(Wrap { trim: true });
    frame.render_widget(status_bar, status_area);
}

fn render_search_prompt(frame: &mut Frame, app: &App, prompt_area: Rect) {
    let prompt = Paragraph::new(format!("/{}", app.query))
        .style(Style::default().bg(Color::Black).fg(Color::Yellow));
    frame.render_widget(prompt, prompt_area);
}

fn render_help(frame: &mut Frame, area: Rect) {
    let popup = centered_rect(80, 80, area);

    let block = Block::default().title("Help").borders(Borders::ALL);
    let paragraph = Paragraph::new(HELP_TEXT)
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, popup);
    frame.render_widget(paragraph, popup);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

pub fn centered_scroll(cursor: usize, total: usize, viewport_h: usize) -> usize {
    if total == 0 || viewport_h == 0 || total <= viewport_h {
        return 0;
    }

    let mid = viewport_h / 2;
    let max_scroll = total - viewport_h;
    cursor.saturating_sub(mid).min(max_scroll)
}

fn format_history_label(
    checked: &str,
    cmd: &str,
    timestamp: Option<i64>,
    show_timestamps: bool,
) -> String {
    if !show_timestamps {
        return format!("{checked} {cmd}");
    }

    let stamp = timestamp
        .map(format_timestamp)
        .unwrap_or_else(|| "-------------------".to_string());
    format!("{checked} {stamp} | {cmd}")
}

fn format_timestamp(timestamp: i64) -> String {
    match Local.timestamp_opt(timestamp, 0) {
        LocalResult::Single(datetime) => datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
        LocalResult::Ambiguous(datetime, _) => datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
        LocalResult::None => format!("ts:{timestamp}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{centered_scroll, format_favorite_label, format_history_label};
    use crate::app::{App, FavoriteRow};
    use crate::favorites::FavoritesStore;
    use crate::history::Entry;

    #[test]
    fn centered_scroll_stays_zero_when_list_fits() {
        assert_eq!(centered_scroll(5, 10, 12), 0);
    }

    #[test]
    fn centered_scroll_keeps_cursor_centered() {
        assert_eq!(centered_scroll(10, 100, 20), 0);
        assert_eq!(centered_scroll(15, 100, 20), 5);
        assert_eq!(centered_scroll(95, 100, 20), 80);
    }

    #[test]
    fn centered_scroll_clamps_near_end() {
        assert_eq!(centered_scroll(9, 10, 4), 6);
    }

    #[test]
    fn entry_label_includes_timestamp_when_enabled() {
        let label = format_history_label("[ ]", "cargo test", Some(1_700_000_000), true);
        assert!(label.contains("cargo test"));
        assert!(label.contains('|'));
    }

    #[test]
    fn favorite_label_indents_lines() {
        let mut favorites = FavoritesStore::new_in_memory();
        favorites
            .add_block(vec![
                "npm run db:migrate".to_string(),
                "npx prisma generate".to_string(),
            ])
            .expect("add favorite");
        let app = App::with_favorites(
            vec![Entry {
                id: 0,
                cmd: "echo hi".to_string(),
                timestamp: None,
                duration_secs: None,
            }],
            favorites,
        );

        let label = format_favorite_label(
            &app,
            FavoriteRow {
                block_index: 0,
                line_index: Some(1),
            },
        );
        assert!(label.starts_with("    [ ] "));
    }
}
