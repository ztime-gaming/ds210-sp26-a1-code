use kalosm::language::*;

pub struct ChatbotV2 {
    model: Llama,
    session: Option<Chat<Llama>>,
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        ChatbotV2 {
            model,
            session: None,
        }
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        if self.session.is_none() {
            let chat_session = self.model
                .chat()
                .with_system_prompt("The assistant will act like a pirate");
            self.session = Some(chat_session);
        }

        if let Some(chat_session) = self.session.as_mut() {
            match chat_session.add_message(message).await {
                Ok(msg) => msg.to_string(),
                Err(_) => "Error generating response".to_string(),
            }
        } else {
            "Error: Session not available".to_string()
        }
    }
}