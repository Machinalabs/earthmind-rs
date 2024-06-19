#[derive(Debug)]
pub enum Log {
    Event {
        event_name: String,
        data: Vec<(&'static str, &'static str)>,
    },
    Message(String),
}
