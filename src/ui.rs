use crate::app::{App, Mode, ToastKind};
use chrono::{Local, LocalResult, TimeZone};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Frame;

const STATUS_HINTS: &str = "enter=accept  y=copy  /=search  s=sort  space=select  q=quit";
const HELP_TEXT: &str = "hline keybindings

Navigation: j/k or arrows, Ctrl+d/u, PageDown/PageUp, g/G
Selection: Space toggle, a select shown, c clear
Accept: Enter print selected/current command(s) to stdout and quit
Search: / enter search, Enter confirm, Esc exit
Search edit: Backspace, Ctrl+w delete word, Ctrl+u clear
Time filter: after:YYYY-MM-DD before:YYYY-MM-DD on:YYYY-MM-DD
Sorting: s cycle sort mode, S reverse sort direction
Copy: y copy selected (or current if none selected)
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
    let list_block = Block::default().title("hline").borders(Borders::ALL);
    let viewport_h = list_area.height.saturating_sub(2) as usize;
    app.set_viewport_height(viewport_h);

    if app.filtered.is_empty() {
        let empty = Paragraph::new("No matches")
            .block(list_block)
            .alignment(Alignment::Center);
        frame.render_widget(empty, list_area);
        return;
    }

    let total = app.filtered.len();
    let scroll = centered_scroll(app.cursor, total, viewport_h);
    let end = (scroll + viewport_h.max(1)).min(total);
    let show_timestamps = app.has_timestamps();

    let mut items = Vec::with_capacity(end.saturating_sub(scroll));
    for pos in scroll..end {
        let idx = app.filtered[pos];
        let entry = &app.all[idx];
        let checked = if app.selected.contains(&entry.id) {
            "[x]"
        } else {
            "[ ]"
        };
        items.push(ListItem::new(format_entry_label(
            checked,
            &entry.cmd,
            entry.timestamp,
            show_timestamps,
        )));
    }

    let mut state = ListState::default();
    state.select(Some(app.cursor.saturating_sub(scroll)));

    let list = List::new(items)
        .block(list_block)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, list_area, &mut state);
}

fn render_status(frame: &mut Frame, app: &App, status_area: Rect) {
    let mut status = format!(
        "sort: {} | filter: {} | {} total | {} shown | {} selected | {}",
        app.status_sort_text(),
        app.status_filter_text(),
        app.all.len(),
        app.filtered.len(),
        app.selected.len(),
        STATUS_HINTS,
    );

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

fn format_entry_label(
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
    use super::{centered_scroll, format_entry_label};

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
        let label = format_entry_label("[ ]", "cargo test", Some(1_700_000_000), true);
        assert!(label.contains("cargo test"));
        assert!(label.contains('|'));
    }
}
