use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    model: Llama,
    sessions: HashMap<String, Chat<Llama>>,
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        ChatbotV3 {
            model,
            sessions: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let session = self.sessions.entry(username.clone()).or_insert_with(|| {
            self.model
                .chat()
                .with_system_prompt("The assistant will act like a pirate")
        });

        match session.add_message(message).await {
            Ok(msg) => msg.to_string(),
            Err(_) => "Error generating response".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
        if let Some(session) = self.sessions.get(&username) {
            session
                .session()
                .iter()
                .map(|m| m.to_string())
                .collect()
        } else {
            Vec::new()
        }
    }
}