// src/translator.rs
pub struct Translator;

impl Translator {
    pub fn new() -> Self {
        Self
    }

    pub async fn translate(&self, text: &str) -> anyhow::Result<String> {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
            .build()?;

        let res = client
            .post("https://translate.googleapis.com/translate_a/single")
            .query(&[
                ("client", "gtx"),
                ("sl", "auto"),
                ("tl", "pt"),
                ("dt", "t"),
            ])
            .form(&[("q", text)])  // <-- texto vai no body, não na URL
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(anyhow::anyhow!("API retornou erro: {}", res.status()));
        }

        let json: serde_json::Value = res.json().await?;

        // Textos grandes vêm em vários segmentos, precisamos juntar todos
        let translated = json[0]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Formato inesperado"))?
            .iter()
            .filter_map(|chunk| chunk[0].as_str())
            .collect::<Vec<_>>()
            .join("");

        Ok(translated)
    }
}
