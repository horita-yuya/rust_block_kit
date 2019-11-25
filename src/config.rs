use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct SlackConfig {
    #[serde(rename = "slack_bot_token")]
    pub bot_token: String,
    #[serde(rename = "slack_channel")]
    pub channel: String,
}
