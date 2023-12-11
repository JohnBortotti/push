use std::fmt;
use rocket::serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Notification<'r> {
    pub title: &'r str,
    pub description: &'r str,
    pub category: NotificationCategory,
}

#[derive(Deserialize, Serialize)]
pub enum NotificationCategory {
    Alert,
    Critical,
    Report,
}

impl fmt::Display for NotificationCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotificationCategory::Alert => write!(f, "Alert"),
            NotificationCategory::Critical => write!(f, "Critical"),
            NotificationCategory::Report => write!(f, "Report"),
        }
    }
}