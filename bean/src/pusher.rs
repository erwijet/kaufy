// i hardly knew her

use std::fmt::Display;

pub enum PusherChannel {
    Orders,
}

impl Display for PusherChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PusherChannel::Orders => write!(f, "orders"),
        }
    }
}

pub async fn notify_user(user_id: i32, title: &str, message: &str) -> Result<(), reqwest::Error> {
    notify(format!("usr{user_id}"), message.to_string(), title.to_string()).await
}

pub async fn notify_channel(channel: PusherChannel, title: &str, message: &str) -> Result<(), reqwest::Error> {
    notify(channel.to_string(), message.to_string(), title.to_string()).await
}

async fn notify(channel: String, message: String, title: String) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .post(format!("https://ntfy.kaufy.holewinski.dev/{channel}"))
        .basic_auth("sys", Some("NzRgVHg3L2opKnA0"))
        .header("X-Title", title)
        .body(message)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
