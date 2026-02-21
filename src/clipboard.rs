use anyhow::{Context, Result};

#[derive(Default)]
pub struct ClipboardManager {
    clipboard: Option<arboard::Clipboard>,
}

impl ClipboardManager {
    pub fn new() -> Self {
        Self { clipboard: None }
    }

    pub fn copy_text(&mut self, text: String) -> Result<()> {
        if self.clipboard.is_none() {
            let clipboard = arboard::Clipboard::new().context("failed to initialize clipboard")?;
            self.clipboard = Some(clipboard);
        }

        self.clipboard
            .as_mut()
            .expect("clipboard initialized")
            .set_text(text)
            .context("failed to set clipboard text")
    }
}
