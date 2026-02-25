use std::process::Command;

pub struct Clipboard;

impl Clipboard {
    pub fn new() -> Self { Self }

    pub async fn get_text(&self) -> anyhow::Result<String> {
        // Tenta a seleção do mouse (--primary)
        let output = Command::new("wl-paste")
            .arg("--primary")
            .arg("--no-newline")
            .output();

        let text = match output {
            Ok(out) if out.status.success() && !out.stdout.is_empty() => {
                String::from_utf8_lossy(&out.stdout).to_string()
            }
            _ => {
                // Fallback para o Ctrl+C normal
                let out = Command::new("wl-paste").arg("--no-newline").output()?;
                String::from_utf8_lossy(&out.stdout).to_string()
            }
        };

        Ok(text.trim().to_string())
    }
}
