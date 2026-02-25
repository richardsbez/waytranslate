use crate::clipboard::Clipboard;
use crate::translator::Translator;
use crate::popup::Popup;

pub struct App {
    clipboard: Clipboard,
    translator: Translator,
}

impl App {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            clipboard: Clipboard::new(),
            translator: Translator::new(),
        })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let text = self.clipboard.get_text().await?;

        if text.trim().is_empty() {
            return Ok(());
        }

        let translated = self.translator.translate(&text).await?;

        Popup::show(translated)?;

        Ok(())
    }
}

