use serde::Deserialize;
use crate::html_templates::NotificationCategory;

#[derive(Deserialize)]
pub struct Notification {
    pub title: String,
    pub description: String,
    pub category: NotificationCategory,
    pub timestamp: String,
}
