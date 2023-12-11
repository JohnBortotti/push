use crate::notification::Notification;

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

pub fn build(notification: Notification) -> String {
    match notification.category {
        NotificationCategory::Alert => alert::build(notification),
        NotificationCategory::Critical => critical::build(notification),
        NotificationCategory::Report => report::build(notification),
    }
}
