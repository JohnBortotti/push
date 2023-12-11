use serde::Serialize;
use super::{html_templates, Message};

#[derive(Serialize)]
pub struct SendGridEmail {
    personalizations: Vec<Personalization>,
    from: From,
    subject: String,
    content: Vec<Content>,
}

#[derive(Serialize)]
struct Personalization {
    to: Vec<To>,
}

#[derive(Serialize)]
struct To {
    email: String,
}

#[derive(Serialize)]
struct From {
    email: String,
    name: String,
}

#[derive(Serialize)]
struct Content {
    r#type: String,
    value: String,
}

pub fn build_sendgrid(mail_from_email: &str, mail_from_name: &str, mail_to: &str, message_json: Message) -> SendGridEmail {
    SendGridEmail {
        from: From {
            email: mail_from_email.to_string(),
            name: mail_from_name.to_string(),
        },
        personalizations: vec![Personalization {
            to: vec![To {
                email: mail_to.to_string(),
            }],
        }],
        subject: message_json.title.clone(),
        content: vec![Content {
            r#type: "text/html".to_string(),
            value: html_templates::build(message_json),
        }],
    }
}
