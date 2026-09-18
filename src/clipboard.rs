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

/// Copy from a short-lived CLI process. On Linux the clipboard owner must stay
/// alive to serve the data, so a detached child of ourselves waits until
/// another program takes the clipboard over.
pub fn copy_and_detach(text: &str) -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        use std::io::Write;
        use std::process::{Command, Stdio};

        let exe = std::env::current_exe().context("failed to locate hline binary")?;
        let mut child = Command::new(exe)
            .arg("--clipboard-wait")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("failed to spawn clipboard holder")?;
        child
            .stdin
            .take()
            .expect("piped stdin")
            .write_all(text.as_bytes())
            .context("failed to send text to clipboard holder")?;
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        ClipboardManager::new().copy_text(text.to_string())
    }
}

/// Child side of `copy_and_detach`: read stdin, own the clipboard, block until replaced.
#[cfg(target_os = "linux")]
pub fn hold_clipboard_from_stdin() -> Result<()> {
    use arboard::SetExtLinux;
    use std::io::Read;

    let mut text = String::new();
    std::io::stdin()
        .read_to_string(&mut text)
        .context("failed to read clipboard text")?;
    arboard::Clipboard::new()
        .context("failed to initialize clipboard")?
        .set()
        .wait()
        .text(text)
        .context("failed to set clipboard text")
}
