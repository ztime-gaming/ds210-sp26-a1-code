use kalosm::language::*;
use file_chatbot::solution::file_library;
use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);
        
        let mut chat: Chat<Llama>;
        
        match cached_chat {
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                chat = chat_session.clone();
            }
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                chat = self.model
                    .chat()
                    .with_system_prompt("The assistant will act like a pirate");
                
                if let Some(session) = file_library::load_chat_session_from_file(filename) {
                    chat = chat.with_session(session);
                }
            }
        }
        
        let response = chat.add_message(message).await;
        
        let session_to_save = chat.session().unwrap();
        file_library::save_chat_session_to_file(filename, &session_to_save);
        
        drop(session_to_save);
        
        self.cache.insert_chat(username, chat);
        
        response.unwrap()
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let _filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);
        
        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                Vec::new()
            }
            Some(_chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                Vec::new()
            }
        }
    }
}