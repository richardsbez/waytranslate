// src/translator.rs
pub struct Translator;

impl Translator {
    pub fn new() -> Self {
        Self
    }

pub async fn translate(&self, text: &str) -> anyhow::Result<String> {
    let client = reqwest::Client::new();

    let res = client
        .post("http://127.0.0.1:5000/translate")
        .json(&serde_json::json!({
            "q": text,
            "source": "auto",
            "target": "pt",
            "format": "text"
        }))
        .send()
        .await?;

    let json: serde_json::Value = res.json().await?;
    Ok(json["translatedText"].as_str().unwrap().to_string())
}
}
