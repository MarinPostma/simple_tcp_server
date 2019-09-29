#[derive(Clone)]
pub struct Message {
    pub sender_id: usize,
    pub sender_name: Option<String>,
    pub content: String,
}

impl Message {
    pub fn new(sender_id: usize, content: &str) -> Self {
        Self {
            sender_id: sender_id,
            sender_name: None,
            content: content.into(),
        }
    }
}
