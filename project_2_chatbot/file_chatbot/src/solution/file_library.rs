use kalosm::language::*;
use std::fs;

pub fn save_chat_session_to_file(filename: &str, session: &LlamaChatSession) {
    let bytes = session.to_bytes().unwrap();
    fs::write(filename, bytes).expect("Failed to save session to file");
}

pub fn load_chat_session_from_file(filename: &str) -> Option<LlamaChatSession> {
    match fs::read(filename) {
        Ok(bytes) => LlamaChatSession::from_bytes(&bytes).ok(),
        Err(_) => None,
    }
}