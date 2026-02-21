use crate::clipboard::ClipboardManager;
use crate::history::Entry;
use crate::sort::{apply_sort, SortMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashSet;
use std::time::{Duration, Instant};

const INFO_TOAST_TTL: Duration = Duration::from_secs(4);
const ERROR_TOAST_TTL: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Search,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    Info,
    Error,
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub kind: ToastKind,
    expires_at: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NormalAction {
    Quit,
    ToggleHelp,
    Move(isize),
    MoveHalfPage(isize),
    JumpTop,
    JumpBottom,
    ToggleSelection,
    SelectAllShown,
    ClearSelection,
    Copy,
    EnterSearch,
    CycleSort,
    ToggleSortDirection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SearchAction {
    ExitSearch,
    ClearQuery,
    DeleteLastWord,
    DeleteLastChar,
    InsertChar(char),
}

pub struct App {
    pub all: Vec<Entry>,
    pub filtered: Vec<usize>,
    pub cursor: usize,
    pub mode: Mode,
    pub query: String,
    pub sort: SortMode,
    pub sort_desc: bool,
    pub selected: HashSet<u64>,
    selected_order: Vec<u64>,
    pub toast: Option<Toast>,
    pub show_help: bool,
    pub should_quit: bool,
    viewport_height: usize,
    clipboard: ClipboardManager,
}

impl App {
    pub fn new(all: Vec<Entry>) -> Self {
        let mut app = Self {
            filtered: Vec::new(),
            all,
            cursor: 0,
            mode: Mode::Normal,
            query: String::new(),
            sort: SortMode::Recency,
            sort_desc: false,
            selected: HashSet::new(),
            selected_order: Vec::new(),
            toast: None,
            show_help: false,
            should_quit: false,
            viewport_height: 10,
            clipboard: ClipboardManager::new(),
        };
        app.rebuild_filtered();
        app
    }

    pub fn tick(&mut self) {
        if self
            .toast
            .as_ref()
            .is_some_and(|toast| Instant::now() >= toast.expires_at)
        {
            self.toast = None;
        }
    }

    pub fn set_viewport_height(&mut self, viewport_height: usize) {
        self.viewport_height = viewport_height.max(1);
    }

    pub fn status_sort_text(&self) -> String {
        let direction = match self.sort {
            SortMode::Recency => {
                if self.sort_desc {
                    "oldest-first"
                } else {
                    "newest-first"
                }
            }
            SortMode::Alpha => {
                if self.sort_desc {
                    "Z->A"
                } else {
                    "A->Z"
                }
            }
            SortMode::Length => {
                if self.sort_desc {
                    "long->short"
                } else {
                    "short->long"
                }
            }
        };

        format!("{} ({})", self.sort.label(), direction)
    }

    pub fn status_filter_text(&self) -> &str {
        if self.query.is_empty() {
            "no filter"
        } else {
            self.query.as_str()
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if self.show_help {
            if matches!(key.code, KeyCode::Esc | KeyCode::Char('?')) {
                self.show_help = false;
            }
            return;
        }

        match self.mode {
            Mode::Normal => {
                if let Some(action) = normal_action_for_key(key) {
                    self.apply_normal_action(action);
                }
            }
            Mode::Search => {
                if let Some(action) = search_action_for_key(key) {
                    self.apply_search_action(action);
                }
            }
        }
    }

    pub fn current_entry(&self) -> Option<&Entry> {
        let idx = *self.filtered.get(self.cursor)?;
        self.all.get(idx)
    }

    fn apply_normal_action(&mut self, action: NormalAction) {
        match action {
            NormalAction::Quit => self.should_quit = true,
            NormalAction::ToggleHelp => self.show_help = true,
            NormalAction::Move(delta) => self.move_by(delta),
            NormalAction::MoveHalfPage(direction) => self.move_half_page(direction),
            NormalAction::JumpTop => self.jump_top(),
            NormalAction::JumpBottom => self.jump_bottom(),
            NormalAction::ToggleSelection => self.toggle_current_selection(),
            NormalAction::SelectAllShown => self.select_all_filtered(),
            NormalAction::ClearSelection => self.clear_selection(),
            NormalAction::Copy => self.copy_current_or_selected(),
            NormalAction::EnterSearch => self.mode = Mode::Search,
            NormalAction::CycleSort => {
                self.sort = self.sort.cycle();
                self.apply_sort_only();
            }
            NormalAction::ToggleSortDirection => {
                self.sort_desc = !self.sort_desc;
                self.apply_sort_only();
            }
        }
    }

    fn apply_search_action(&mut self, action: SearchAction) {
        match action {
            SearchAction::ExitSearch => {
                self.mode = Mode::Normal;
            }
            SearchAction::ClearQuery => {
                if !self.query.is_empty() {
                    self.query.clear();
                    self.rebuild_filtered();
                }
            }
            SearchAction::DeleteLastWord => {
                self.delete_last_word();
                self.rebuild_filtered();
            }
            SearchAction::DeleteLastChar => {
                self.query.pop();
                self.rebuild_filtered();
            }
            SearchAction::InsertChar(c) => {
                self.query.push(c);
                self.rebuild_filtered();
            }
        }
    }

    fn move_by(&mut self, delta: isize) {
        if self.filtered.is_empty() {
            self.cursor = 0;
            return;
        }

        let current = self.cursor as isize;
        let max = (self.filtered.len() - 1) as isize;
        self.cursor = (current + delta).clamp(0, max) as usize;
    }

    fn move_half_page(&mut self, direction: isize) {
        let step = (self.viewport_height / 2).max(1) as isize;
        self.move_by(step * direction);
    }

    fn jump_top(&mut self) {
        self.cursor = 0;
    }

    fn jump_bottom(&mut self) {
        self.cursor = self.filtered.len().saturating_sub(1);
    }

    fn toggle_current_selection(&mut self) {
        let Some(entry_id) = self.current_entry().map(|entry| entry.id) else {
            return;
        };

        if self.selected.contains(&entry_id) {
            self.deselect_entry_by_id(entry_id);
        } else {
            self.select_entry_by_id(entry_id);
        }
    }

    fn select_all_filtered(&mut self) {
        let ids: Vec<u64> = self.filtered.iter().map(|idx| self.all[*idx].id).collect();
        for entry_id in ids {
            self.select_entry_by_id(entry_id);
        }
        self.set_info_toast(format!("Selected {} shown entries", self.filtered.len()));
    }

    fn clear_selection(&mut self) {
        self.selected.clear();
        self.selected_order.clear();
        self.set_info_toast("Cleared selections".to_string());
    }

    fn copy_current_or_selected(&mut self) {
        let lines = self.copy_lines_in_selection_order();

        if lines.is_empty() {
            self.set_error_toast("Nothing to copy".to_string());
            return;
        }

        let count = lines.len();
        let payload = lines.join("\n");

        match self.clipboard.copy_text(payload) {
            Ok(_) => {
                let suffix = if count == 1 { "" } else { "s" };
                self.set_info_toast(format!("Copied {count} line{suffix}"));
            }
            Err(err) => self.set_error_toast(format!("Clipboard error: {err}")),
        }
    }

    fn copy_lines_in_selection_order(&self) -> Vec<&str> {
        if self.selected.is_empty() {
            return self
                .current_entry()
                .map(|entry| vec![entry.cmd.as_str()])
                .unwrap_or_default();
        }

        self.selected_order
            .iter()
            .filter_map(|id| self.entry_by_id(*id))
            .map(|entry| entry.cmd.as_str())
            .collect()
    }

    fn select_entry_by_id(&mut self, entry_id: u64) {
        if self.selected.insert(entry_id) {
            self.selected_order.push(entry_id);
        }
    }

    fn deselect_entry_by_id(&mut self, entry_id: u64) {
        self.selected.remove(&entry_id);
        self.selected_order.retain(|id| *id != entry_id);
    }

    fn entry_by_id(&self, entry_id: u64) -> Option<&Entry> {
        self.all
            .get(entry_id as usize)
            .filter(|entry| entry.id == entry_id)
            .or_else(|| self.all.iter().find(|entry| entry.id == entry_id))
    }

    fn apply_sort_only(&mut self) {
        apply_sort(&mut self.filtered, &self.all, self.sort, self.sort_desc);
        self.clamp_cursor();
    }

    pub fn rebuild_filtered(&mut self) {
        self.filtered.clear();

        if self.query.is_empty() {
            self.filtered.extend(0..self.all.len());
        } else if self.query.is_ascii() {
            let needle = self.query.as_bytes();
            for (idx, entry) in self.all.iter().enumerate() {
                if contains_ascii_case_insensitive(entry.cmd.as_bytes(), needle) {
                    self.filtered.push(idx);
                }
            }
        } else {
            let needle = self.query.to_lowercase();
            for (idx, entry) in self.all.iter().enumerate() {
                if entry.cmd.to_lowercase().contains(&needle) {
                    self.filtered.push(idx);
                }
            }
        }

        apply_sort(&mut self.filtered, &self.all, self.sort, self.sort_desc);
        self.clamp_cursor();
    }

    fn clamp_cursor(&mut self) {
        if self.filtered.is_empty() {
            self.cursor = 0;
        } else {
            self.cursor = self.cursor.min(self.filtered.len() - 1);
        }
    }

    fn delete_last_word(&mut self) {
        while self.query.chars().last().is_some_and(|c| c.is_whitespace()) {
            self.query.pop();
        }

        while self
            .query
            .chars()
            .last()
            .is_some_and(|c| !c.is_whitespace())
        {
            self.query.pop();
        }
    }

    fn set_info_toast(&mut self, message: String) {
        self.set_toast(ToastKind::Info, message, INFO_TOAST_TTL);
    }

    fn set_error_toast(&mut self, message: String) {
        self.set_toast(ToastKind::Error, message, ERROR_TOAST_TTL);
    }

    fn set_toast(&mut self, kind: ToastKind, message: String, ttl: Duration) {
        self.toast = Some(Toast {
            message,
            kind,
            expires_at: Instant::now() + ttl,
        });
    }
}

fn normal_action_for_key(key: KeyEvent) -> Option<NormalAction> {
    if is_ctrl_char(&key, 'd') || matches!(key.code, KeyCode::PageDown) {
        return Some(NormalAction::MoveHalfPage(1));
    }

    if is_ctrl_char(&key, 'u') || matches!(key.code, KeyCode::PageUp) {
        return Some(NormalAction::MoveHalfPage(-1));
    }

    match key.code {
        KeyCode::Char('q') => Some(NormalAction::Quit),
        KeyCode::Char('?') => Some(NormalAction::ToggleHelp),
        KeyCode::Char('j') | KeyCode::Down => Some(NormalAction::Move(1)),
        KeyCode::Char('k') | KeyCode::Up => Some(NormalAction::Move(-1)),
        KeyCode::Char('g') => Some(NormalAction::JumpTop),
        KeyCode::Char('G') => Some(NormalAction::JumpBottom),
        KeyCode::Char(' ') => Some(NormalAction::ToggleSelection),
        KeyCode::Char('a') => Some(NormalAction::SelectAllShown),
        KeyCode::Char('c') => Some(NormalAction::ClearSelection),
        KeyCode::Char('y') => Some(NormalAction::Copy),
        KeyCode::Char('/') => Some(NormalAction::EnterSearch),
        KeyCode::Char('s') => Some(NormalAction::CycleSort),
        KeyCode::Char('S') => Some(NormalAction::ToggleSortDirection),
        _ => None,
    }
}

fn search_action_for_key(key: KeyEvent) -> Option<SearchAction> {
    if matches!(key.code, KeyCode::Esc | KeyCode::Enter) {
        return Some(SearchAction::ExitSearch);
    }

    if is_ctrl_char(&key, 'u') {
        return Some(SearchAction::ClearQuery);
    }

    if is_ctrl_char(&key, 'w')
        || (key.modifiers.contains(KeyModifiers::CONTROL) && matches!(key.code, KeyCode::Backspace))
    {
        return Some(SearchAction::DeleteLastWord);
    }

    match key.code {
        KeyCode::Backspace => Some(SearchAction::DeleteLastChar),
        KeyCode::Char(c)
            if !key.modifiers.contains(KeyModifiers::CONTROL)
                && !key.modifiers.contains(KeyModifiers::ALT) =>
        {
            Some(SearchAction::InsertChar(c))
        }
        _ => None,
    }
}

fn is_ctrl_char(key: &KeyEvent, c: char) -> bool {
    key.code == KeyCode::Char(c) && key.modifiers.contains(KeyModifiers::CONTROL)
}

fn contains_ascii_case_insensitive(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }

    haystack
        .windows(needle.len())
        .any(|window| window.eq_ignore_ascii_case(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_entry(id: u64, cmd: &str) -> Entry {
        Entry {
            id,
            cmd: cmd.to_string(),
        }
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn ctrl(c: char) -> KeyEvent {
        KeyEvent::new(KeyCode::Char(c), KeyModifiers::CONTROL)
    }

    #[test]
    fn filtering_is_case_insensitive() {
        let mut app = App::new(vec![
            mk_entry(0, "git status"),
            mk_entry(1, "Cargo test"),
            mk_entry(2, "ls -la"),
        ]);

        app.query = "CARGO".to_string();
        app.rebuild_filtered();

        assert_eq!(app.filtered, vec![1]);
    }

    #[test]
    fn cursor_clamps_when_results_shrink() {
        let mut app = App::new(vec![mk_entry(0, "one"), mk_entry(1, "two")]);
        app.cursor = 1;

        app.query = "one".to_string();
        app.rebuild_filtered();

        assert_eq!(app.cursor, 0);
        assert_eq!(app.filtered, vec![0]);
    }

    #[test]
    fn ctrl_u_moves_half_page_in_normal_mode() {
        let mut app = App::new(vec![
            mk_entry(0, "0"),
            mk_entry(1, "1"),
            mk_entry(2, "2"),
            mk_entry(3, "3"),
            mk_entry(4, "4"),
            mk_entry(5, "5"),
            mk_entry(6, "6"),
        ]);

        app.cursor = 5;
        app.set_viewport_height(6);
        app.handle_key(ctrl('u'));

        assert_eq!(app.cursor, 2);
    }

    #[test]
    fn ctrl_u_clears_query_in_search_mode() {
        let mut app = App::new(vec![mk_entry(0, "one"), mk_entry(1, "two")]);
        app.mode = Mode::Search;
        app.query = "on".to_string();

        app.handle_key(ctrl('u'));

        assert!(app.query.is_empty());
        assert_eq!(app.mode, Mode::Search);
        assert_eq!(app.filtered.len(), 2);
    }

    #[test]
    fn copy_lines_follow_selection_order() {
        let mut app = App::new(vec![
            mk_entry(0, "newest"),
            mk_entry(1, "middle"),
            mk_entry(2, "oldest"),
        ]);

        app.select_entry_by_id(2);
        app.select_entry_by_id(0);

        assert_eq!(
            app.copy_lines_in_selection_order(),
            vec!["oldest", "newest"]
        );

        app.selected.clear();
        app.selected_order.clear();
        app.cursor = 1;
        assert_eq!(app.copy_lines_in_selection_order(), vec!["middle"]);
    }

    #[test]
    fn slash_enters_search_mode() {
        let mut app = App::new(vec![mk_entry(0, "echo hi")]);

        app.handle_key(key(KeyCode::Char('/')));

        assert_eq!(app.mode, Mode::Search);
    }
}
