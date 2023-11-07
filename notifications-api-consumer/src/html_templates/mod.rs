use super::{Message, NotificationCategory};

pub mod alert;
pub mod critical;
pub mod report;

pub fn build(message: Message) -> String {
    match message.category {
        NotificationCategory::Alert => alert::build(message),
        NotificationCategory::Critical => critical::build(message),
        NotificationCategory::Report => report::build(message),
    }
}
