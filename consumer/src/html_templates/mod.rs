use super::Message;
use serde::Deserialize;

pub mod alert;
pub mod critical;
pub mod report;

#[derive(Deserialize)]
pub enum NotificationCategory {
    Alert,
    Critical,
    Report,
}

pub fn build(message: Message) -> String {
    match message.category {
        NotificationCategory::Alert => alert::build(message),
        NotificationCategory::Critical => critical::build(message),
        NotificationCategory::Report => report::build(message),
    }
}
