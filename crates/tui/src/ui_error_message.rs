#[derive(Debug, Clone)]
pub struct UiErrorMessage {
    title: String,
    message: String,
}

impl UiErrorMessage {
    pub fn from_anyhow(error: anyhow::Error) -> Self {
        let mut chain = error.chain();
        let title = chain
            .next()
            .map(|err| err.to_string())
            .unwrap_or_else(|| "Error".to_string());

        let message = chain
            .next()
            .map(|err| err.to_string())
            .unwrap_or_else(|| title.clone());
        Self { title, message }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_message(&self) -> &str {
        &self.message
    }
}
