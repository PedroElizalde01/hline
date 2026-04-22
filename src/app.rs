use crate::clipboard::ClipboardManager;
use crate::favorites::{AddFavoriteResult, FavoriteBlock, FavoritesStore};
use crate::history::Entry;
use crate::sort::{apply_sort, SortMode};
use chrono::{Local, LocalResult, NaiveDate, NaiveDateTime, TimeZone};
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
pub enum View {
    History,
    Favorites,
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
    MoveBlock(isize),
    JumpTop,
    JumpBottom,
    ToggleSelection,
    SelectAllShown,
    ClearSelection,
    Copy,
    CopyLine,
    Accept,
    SaveFavorite,
    ToggleFavoritesView,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FavoriteRow {
    pub block_index: usize,
    pub line_index: Option<usize>,
}

pub struct App {
    pub view: View,
    pub all: Vec<Entry>,
    pub filtered: Vec<usize>,
    pub cursor: usize,
    pub favorite_rows: Vec<FavoriteRow>,
    pub favorite_cursor: usize,
    pub mode: Mode,
    pub query: String,
    pub sort: SortMode,
    pub sort_desc: bool,
    pub selected: HashSet<u64>,
    selected_order: Vec<u64>,
    pub toast: Option<Toast>,
    pub show_help: bool,
    pub should_quit: bool,
    accepted_output: Option<String>,
    viewport_height: usize,
    clipboard: ClipboardManager,
    favorites: FavoritesStore,
}

impl App {
    #[cfg(test)]
    pub fn new(all: Vec<Entry>) -> Self {
        Self::with_favorites(all, FavoritesStore::new_in_memory())
    }

    pub fn with_favorites(all: Vec<Entry>, favorites: FavoritesStore) -> Self {
        let mut app = Self {
            view: View::History,
            filtered: Vec::new(),
            all,
            cursor: 0,
            favorite_rows: Vec::new(),
            favorite_cursor: 0,
            mode: Mode::Normal,
            query: String::new(),
            sort: SortMode::Recency,
            sort_desc: false,
            selected: HashSet::new(),
            selected_order: Vec::new(),
            toast: None,
            show_help: false,
            should_quit: false,
            accepted_output: None,
            viewport_height: 10,
            clipboard: ClipboardManager::new(),
            favorites,
        };
        app.rebuild_filtered();
        app.rebuild_favorite_rows();
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
            SortMode::Recency | SortMode::Timestamp => {
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

    pub fn status_hint_text(&self) -> &'static str {
        match self.view {
            View::History => {
                "enter=accept  y=copy  f=save fav  F=favorites  /=search  s=sort  space=select  q=quit"
            }
            View::Favorites => {
                "enter=accept fav  y=copy fav  Y=copy line  J/K=block  F=history  /=search  q=quit"
            }
        }
    }

    pub fn status_context_text(&self) -> String {
        match self.view {
            View::History => format!("sort: {}", self.status_sort_text()),
            View::Favorites => format!("favorites: {} blocks", self.favorites.blocks.len()),
        }
    }

    pub fn list_title(&self) -> &'static str {
        match self.view {
            View::History => "hline",
            View::Favorites => "favorites",
        }
    }

    pub fn visible_count(&self) -> usize {
        match self.view {
            View::History => self.filtered.len(),
            View::Favorites => self.favorite_rows.len(),
        }
    }

    pub fn total_count(&self) -> usize {
        match self.view {
            View::History => self.all.len(),
            View::Favorites => self.favorites.blocks.len(),
        }
    }

    pub fn active_cursor(&self) -> usize {
        match self.view {
            View::History => self.cursor,
            View::Favorites => self.favorite_cursor,
        }
    }

    pub fn has_timestamps(&self) -> bool {
        matches!(self.view, View::History) && self.all.iter().any(|entry| entry.timestamp.is_some())
    }

    pub fn take_accepted_output(&mut self) -> Option<String> {
        self.accepted_output.take()
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

    pub fn current_favorite_row(&self) -> Option<FavoriteRow> {
        self.favorite_rows.get(self.favorite_cursor).copied()
    }

    pub fn current_favorite_block(&self) -> Option<&FavoriteBlock> {
        let row = self.current_favorite_row()?;
        self.favorites.blocks.get(row.block_index)
    }

    pub fn favorite_block(&self, block_index: usize) -> Option<&FavoriteBlock> {
        self.favorites.blocks.get(block_index)
    }

    fn current_favorite_line(&self) -> Option<&str> {
        let row = self.current_favorite_row()?;
        let block = self.favorites.blocks.get(row.block_index)?;
        let line_index = row.line_index.unwrap_or(0);
        block.lines.get(line_index).map(String::as_str)
    }

    fn apply_normal_action(&mut self, action: NormalAction) {
        match action {
            NormalAction::Quit => self.should_quit = true,
            NormalAction::ToggleHelp => self.show_help = true,
            NormalAction::Move(delta) => self.move_by(delta),
            NormalAction::MoveHalfPage(direction) => self.move_half_page(direction),
            NormalAction::MoveBlock(direction) => self.move_block(direction),
            NormalAction::JumpTop => self.jump_top(),
            NormalAction::JumpBottom => self.jump_bottom(),
            NormalAction::ToggleSelection => self.toggle_current_selection(),
            NormalAction::SelectAllShown => self.select_all_filtered(),
            NormalAction::ClearSelection => self.clear_selection(),
            NormalAction::Copy => self.copy_current_scope(),
            NormalAction::CopyLine => self.copy_current_line_only(),
            NormalAction::Accept => self.accept_current_scope(),
            NormalAction::SaveFavorite => self.save_current_or_selected_favorite(),
            NormalAction::ToggleFavoritesView => self.toggle_favorites_view(),
            NormalAction::EnterSearch => self.mode = Mode::Search,
            NormalAction::CycleSort => {
                if matches!(self.view, View::History) {
                    self.sort = self.sort.cycle();
                    self.apply_sort_only();
                } else {
                    self.set_info_toast("Favorites keep saved order".to_string());
                }
            }
            NormalAction::ToggleSortDirection => {
                if matches!(self.view, View::History) {
                    self.sort_desc = !self.sort_desc;
                    self.apply_sort_only();
                } else {
                    self.set_info_toast("Favorites keep saved order".to_string());
                }
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
                    self.rebuild_active_view();
                }
            }
            SearchAction::DeleteLastWord => {
                self.delete_last_word();
                self.rebuild_active_view();
            }
            SearchAction::DeleteLastChar => {
                self.query.pop();
                self.rebuild_active_view();
            }
            SearchAction::InsertChar(c) => {
                self.query.push(c);
                self.rebuild_active_view();
            }
        }
    }

    fn move_by(&mut self, delta: isize) {
        match self.view {
            View::History => {
                if self.filtered.is_empty() {
                    self.cursor = 0;
                    return;
                }

                let current = self.cursor as isize;
                let max = (self.filtered.len() - 1) as isize;
                self.cursor = (current + delta).clamp(0, max) as usize;
            }
            View::Favorites => {
                if self.favorite_rows.is_empty() {
                    self.favorite_cursor = 0;
                    return;
                }

                let current = self.favorite_cursor as isize;
                let max = (self.favorite_rows.len() - 1) as isize;
                self.favorite_cursor = (current + delta).clamp(0, max) as usize;
            }
        }
    }

    fn move_half_page(&mut self, direction: isize) {
        let step = (self.viewport_height / 2).max(1) as isize;
        self.move_by(step * direction);
    }

    fn move_block(&mut self, direction: isize) {
        if !matches!(self.view, View::Favorites) {
            self.move_by(direction);
            return;
        }

        if self.favorite_rows.is_empty() {
            self.favorite_cursor = 0;
            return;
        }

        let visible_blocks: Vec<usize> = self
            .favorite_rows
            .iter()
            .filter_map(|row| row.line_index.is_none().then_some(row.block_index))
            .collect();
        let current_block = self.favorite_rows[self.favorite_cursor].block_index;
        let current_visible = visible_blocks
            .iter()
            .position(|block_index| *block_index == current_block)
            .unwrap_or(0) as isize;
        let target_visible = (current_visible + direction)
            .clamp(0, visible_blocks.len().saturating_sub(1) as isize)
            as usize;
        let target_block = visible_blocks[target_visible];

        if let Some(pos) = self
            .favorite_rows
            .iter()
            .position(|row| row.block_index == target_block)
        {
            self.favorite_cursor = pos;
        }
    }

    fn jump_top(&mut self) {
        match self.view {
            View::History => self.cursor = 0,
            View::Favorites => self.favorite_cursor = 0,
        }
    }

    fn jump_bottom(&mut self) {
        match self.view {
            View::History => self.cursor = self.filtered.len().saturating_sub(1),
            View::Favorites => self.favorite_cursor = self.favorite_rows.len().saturating_sub(1),
        }
    }

    fn toggle_current_selection(&mut self) {
        if !matches!(self.view, View::History) {
            self.set_info_toast("Favorites use block copy, not selection".to_string());
            return;
        }

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
        if !matches!(self.view, View::History) {
            self.set_info_toast("Favorites use block copy, not selection".to_string());
            return;
        }

        let ids: Vec<u64> = self.filtered.iter().map(|idx| self.all[*idx].id).collect();
        for entry_id in ids {
            self.select_entry_by_id(entry_id);
        }
        self.set_info_toast(format!("Selected {} shown entries", self.filtered.len()));
    }

    fn clear_selection(&mut self) {
        if !matches!(self.view, View::History) {
            self.query.clear();
            self.rebuild_active_view();
            self.set_info_toast("Cleared favorites filter".to_string());
            return;
        }

        self.selected.clear();
        self.selected_order.clear();
        self.set_info_toast("Cleared selections".to_string());
    }

    fn copy_current_scope(&mut self) {
        match self.view {
            View::History => {
                let lines = self.selected_lines_in_order();
                if lines.is_empty() {
                    self.set_error_toast("Nothing to copy".to_string());
                    return;
                }

                let count = lines.len();
                let suffix = if count == 1 { "" } else { "s" };
                self.copy_payload(lines.join("\n"), format!("Copied {count} line{suffix}"));
            }
            View::Favorites => {
                let Some(block) = self.current_favorite_block() else {
                    self.set_error_toast("No favorite selected".to_string());
                    return;
                };

                let count = block.lines.len();
                let payload = block.lines.join("\n");
                let suffix = if count == 1 { "" } else { "s" };
                self.copy_payload(
                    payload,
                    format!("Copied favorite with {count} line{suffix}"),
                );
            }
        }
    }

    fn copy_current_line_only(&mut self) {
        let payload = match self.view {
            View::History => self.current_entry().map(|entry| entry.cmd.clone()),
            View::Favorites => self.current_favorite_line().map(str::to_string),
        };

        let Some(payload) = payload else {
            self.set_error_toast("Nothing to copy".to_string());
            return;
        };

        self.copy_payload(payload, "Copied current line".to_string());
    }

    fn copy_payload(&mut self, payload: String, success_message: String) {
        match self.clipboard.copy_text(payload) {
            Ok(_) => self.set_info_toast(success_message),
            Err(err) => self.set_error_toast(format!("Clipboard error: {err}")),
        }
    }

    fn accept_current_scope(&mut self) {
        match self.view {
            View::History => {
                let lines = self.selected_lines_in_order();
                if lines.is_empty() {
                    self.set_error_toast("Nothing to accept".to_string());
                    return;
                }
                self.accepted_output = Some(lines.join("\n"));
            }
            View::Favorites => {
                let Some(block) = self.current_favorite_block() else {
                    self.set_error_toast("No favorite selected".to_string());
                    return;
                };
                self.accepted_output = Some(block.lines.join("\n"));
            }
        }

        self.should_quit = true;
    }

    fn save_current_or_selected_favorite(&mut self) {
        if !matches!(self.view, View::History) {
            self.set_info_toast("Already in favorites view".to_string());
            return;
        }

        let lines = self.selected_lines_in_order_owned();
        match self.favorites.add_block(lines) {
            Ok(AddFavoriteResult::Added(_)) => {
                self.rebuild_favorite_rows();
                self.set_info_toast("Saved favorite".to_string());
            }
            Ok(AddFavoriteResult::Duplicate(_)) => {
                self.set_info_toast("Favorite already saved".to_string());
            }
            Ok(AddFavoriteResult::Empty) => {
                self.set_error_toast("Nothing to favorite".to_string());
            }
            Err(err) => self.set_error_toast(format!("Favorites error: {err}")),
        }
    }

    fn toggle_favorites_view(&mut self) {
        self.mode = Mode::Normal;
        self.view = match self.view {
            View::History => View::Favorites,
            View::Favorites => View::History,
        };
        self.rebuild_active_view();
    }

    fn selected_lines_in_order(&self) -> Vec<&str> {
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

    fn selected_lines_in_order_owned(&self) -> Vec<String> {
        self.selected_lines_in_order()
            .into_iter()
            .map(str::to_string)
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
        self.clamp_history_cursor();
    }

    fn rebuild_active_view(&mut self) {
        match self.view {
            View::History => self.rebuild_filtered(),
            View::Favorites => self.rebuild_favorite_rows(),
        }
    }

    pub fn rebuild_filtered(&mut self) {
        self.filtered.clear();
        let filters = ParsedQuery::parse(&self.query);

        for (idx, entry) in self.all.iter().enumerate() {
            if filters.matches(entry) {
                self.filtered.push(idx);
            }
        }

        apply_sort(&mut self.filtered, &self.all, self.sort, self.sort_desc);
        self.clamp_history_cursor();
    }

    pub fn rebuild_favorite_rows(&mut self) {
        self.favorite_rows.clear();
        let needle = ParsedQuery::parse(&self.query).text;

        for (block_index, block) in self.favorites.blocks.iter().enumerate() {
            if !favorite_matches_query(block, &needle) {
                continue;
            }

            self.favorite_rows.push(FavoriteRow {
                block_index,
                line_index: None,
            });

            for line_index in 0..block.lines.len() {
                self.favorite_rows.push(FavoriteRow {
                    block_index,
                    line_index: Some(line_index),
                });
            }
        }

        self.clamp_favorite_cursor();
    }

    fn clamp_history_cursor(&mut self) {
        if self.filtered.is_empty() {
            self.cursor = 0;
        } else {
            self.cursor = self.cursor.min(self.filtered.len() - 1);
        }
    }

    fn clamp_favorite_cursor(&mut self) {
        if self.favorite_rows.is_empty() {
            self.favorite_cursor = 0;
        } else {
            self.favorite_cursor = self.favorite_cursor.min(self.favorite_rows.len() - 1);
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
    if key.modifiers.contains(KeyModifiers::SHIFT)
        && matches!(key.code, KeyCode::Down | KeyCode::Up)
    {
        return Some(match key.code {
            KeyCode::Down => NormalAction::MoveBlock(1),
            KeyCode::Up => NormalAction::MoveBlock(-1),
            _ => unreachable!(),
        });
    }

    if is_ctrl_char(&key, 'd') || matches!(key.code, KeyCode::PageDown) {
        return Some(NormalAction::MoveHalfPage(1));
    }

    if is_ctrl_char(&key, 'u') || matches!(key.code, KeyCode::PageUp) {
        return Some(NormalAction::MoveHalfPage(-1));
    }

    match key.code {
        KeyCode::Char('q') => Some(NormalAction::Quit),
        KeyCode::Char('?') => Some(NormalAction::ToggleHelp),
        KeyCode::Enter => Some(NormalAction::Accept),
        KeyCode::Char('j') | KeyCode::Down => Some(NormalAction::Move(1)),
        KeyCode::Char('k') | KeyCode::Up => Some(NormalAction::Move(-1)),
        KeyCode::Char('J') => Some(NormalAction::MoveBlock(1)),
        KeyCode::Char('K') => Some(NormalAction::MoveBlock(-1)),
        KeyCode::Char('g') => Some(NormalAction::JumpTop),
        KeyCode::Char('G') => Some(NormalAction::JumpBottom),
        KeyCode::Char(' ') => Some(NormalAction::ToggleSelection),
        KeyCode::Char('a') => Some(NormalAction::SelectAllShown),
        KeyCode::Char('c') => Some(NormalAction::ClearSelection),
        KeyCode::Char('y') => Some(NormalAction::Copy),
        KeyCode::Char('Y') => Some(NormalAction::CopyLine),
        KeyCode::Char('f') => Some(NormalAction::SaveFavorite),
        KeyCode::Char('F') => Some(NormalAction::ToggleFavoritesView),
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

fn text_matches(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }

    if needle.is_ascii() {
        contains_ascii_case_insensitive(haystack.as_bytes(), needle.as_bytes())
    } else {
        haystack.to_lowercase().contains(&needle.to_lowercase())
    }
}

fn favorite_matches_query(block: &FavoriteBlock, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }

    block.lines.iter().any(|line| text_matches(line, needle))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedQuery {
    text: String,
    after: Option<i64>,
    before: Option<i64>,
}

impl ParsedQuery {
    fn parse(query: &str) -> Self {
        let mut text_tokens = Vec::new();
        let mut after = None;
        let mut before = None;

        for token in query.split_whitespace() {
            if let Some(value) = token
                .strip_prefix("after:")
                .or_else(|| token.strip_prefix("since:"))
            {
                if let Some(timestamp) = parse_time_value(value, false) {
                    after = Some(after.map_or(timestamp, |current: i64| current.max(timestamp)));
                    continue;
                }
            }

            if let Some(value) = token
                .strip_prefix("before:")
                .or_else(|| token.strip_prefix("until:"))
            {
                if let Some(timestamp) = parse_time_value(value, true) {
                    before = Some(before.map_or(timestamp, |current: i64| current.min(timestamp)));
                    continue;
                }
            }

            if let Some(value) = token.strip_prefix("on:") {
                if let Some((start, end)) = parse_day_range(value) {
                    after = Some(after.map_or(start, |current: i64| current.max(start)));
                    before = Some(before.map_or(end, |current: i64| current.min(end)));
                    continue;
                }
            }

            text_tokens.push(token);
        }

        Self {
            text: text_tokens.join(" "),
            after,
            before,
        }
    }

    fn matches(&self, entry: &Entry) -> bool {
        if let Some(after) = self.after {
            let Some(timestamp) = entry.timestamp else {
                return false;
            };
            if timestamp < after {
                return false;
            }
        }

        if let Some(before) = self.before {
            let Some(timestamp) = entry.timestamp else {
                return false;
            };
            if timestamp > before {
                return false;
            }
        }

        text_matches(&entry.cmd, &self.text)
    }
}

fn parse_time_value(value: &str, end_of_day: bool) -> Option<i64> {
    if let Ok(timestamp) = value.parse::<i64>() {
        return Some(timestamp);
    }

    for format in ["%Y-%m-%dT%H:%M:%S", "%Y-%m-%dT%H:%M"] {
        if let Ok(naive) = NaiveDateTime::parse_from_str(value, format) {
            return local_timestamp(naive);
        }
    }

    let date = NaiveDate::parse_from_str(value, "%Y-%m-%d").ok()?;
    let naive = if end_of_day {
        date.and_hms_opt(23, 59, 59)?
    } else {
        date.and_hms_opt(0, 0, 0)?
    };
    local_timestamp(naive)
}

fn parse_day_range(value: &str) -> Option<(i64, i64)> {
    let date = NaiveDate::parse_from_str(value, "%Y-%m-%d").ok()?;
    let start = local_timestamp(date.and_hms_opt(0, 0, 0)?)?;
    let end = local_timestamp(date.and_hms_opt(23, 59, 59)?)?;
    Some((start, end))
}

fn local_timestamp(naive: NaiveDateTime) -> Option<i64> {
    match Local.from_local_datetime(&naive) {
        LocalResult::Single(datetime) => Some(datetime.timestamp()),
        LocalResult::Ambiguous(first, _) => Some(first.timestamp()),
        LocalResult::None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_entry(id: u64, cmd: &str) -> Entry {
        Entry {
            id,
            cmd: cmd.to_string(),
            timestamp: None,
            duration_secs: None,
        }
    }

    fn mk_timed_entry(id: u64, cmd: &str, timestamp: i64) -> Entry {
        Entry {
            id,
            cmd: cmd.to_string(),
            timestamp: Some(timestamp),
            duration_secs: None,
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

        assert_eq!(app.selected_lines_in_order(), vec!["oldest", "newest"]);

        app.selected.clear();
        app.selected_order.clear();
        app.cursor = 1;
        assert_eq!(app.selected_lines_in_order(), vec!["middle"]);
    }

    #[test]
    fn slash_enters_search_mode() {
        let mut app = App::new(vec![mk_entry(0, "echo hi")]);

        app.handle_key(key(KeyCode::Char('/')));

        assert_eq!(app.mode, Mode::Search);
    }

    #[test]
    fn enter_accepts_current_line_and_quits() {
        let mut app = App::new(vec![mk_entry(0, "echo hi")]);

        app.handle_key(key(KeyCode::Enter));

        assert!(app.should_quit);
        assert_eq!(app.take_accepted_output(), Some("echo hi".to_string()));
    }

    #[test]
    fn time_filters_apply_alongside_text_search() {
        let mut app = App::new(vec![
            mk_timed_entry(0, "cargo test", 1_700_000_000),
            mk_timed_entry(1, "git status", 1_700_100_000),
            mk_entry(2, "cargo fmt"),
        ]);

        app.query = "cargo after:2023-11-14 before:2023-11-15".to_string();
        app.rebuild_filtered();

        assert_eq!(app.filtered, vec![0]);
    }

    #[test]
    fn on_filter_excludes_entries_without_timestamps() {
        let mut app = App::new(vec![
            mk_timed_entry(0, "cargo test", 1_700_000_000),
            mk_entry(1, "cargo fmt"),
        ]);

        app.query = "on:2023-11-14".to_string();
        app.rebuild_filtered();

        assert_eq!(app.filtered, vec![0]);
    }

    #[test]
    fn saving_selection_creates_favorite_block() {
        let mut app = App::new(vec![
            mk_entry(0, "npm run db:migrate"),
            mk_entry(1, "npx prisma generate"),
        ]);

        app.select_entry_by_id(0);
        app.select_entry_by_id(1);
        app.handle_key(key(KeyCode::Char('f')));

        assert_eq!(app.favorites.blocks.len(), 1);
        assert_eq!(
            app.favorites.blocks[0].lines,
            vec!["npm run db:migrate", "npx prisma generate"]
        );
    }

    #[test]
    fn favorites_view_accepts_whole_block_and_tracks_current_line() {
        let mut favorites = FavoritesStore::new_in_memory();
        favorites
            .add_block(vec![
                "npm run db:migrate".to_string(),
                "npx prisma generate".to_string(),
            ])
            .expect("add favorite");
        let mut app = App::with_favorites(vec![mk_entry(0, "echo hi")], favorites);

        app.handle_key(key(KeyCode::Char('F')));
        app.favorite_cursor = 1;
        assert_eq!(app.current_favorite_line(), Some("npm run db:migrate"));

        app.handle_key(key(KeyCode::Enter));
        assert_eq!(
            app.take_accepted_output(),
            Some("npm run db:migrate\nnpx prisma generate".to_string())
        );
    }

    #[test]
    fn favorites_view_moves_by_block_with_uppercase_j_k() {
        let mut favorites = FavoritesStore::new_in_memory();
        favorites
            .add_block(vec!["one".to_string(), "two".to_string()])
            .expect("add favorite 1");
        favorites
            .add_block(vec!["three".to_string()])
            .expect("add favorite 2");
        let mut app = App::with_favorites(vec![mk_entry(0, "echo hi")], favorites);

        app.handle_key(key(KeyCode::Char('F')));
        assert_eq!(app.favorite_rows[0].block_index, 0);

        app.handle_key(key(KeyCode::Char('J')));
        assert_eq!(app.favorite_rows[app.favorite_cursor].block_index, 1);

        app.handle_key(key(KeyCode::Char('K')));
        assert_eq!(app.favorite_rows[app.favorite_cursor].block_index, 0);
    }
}
