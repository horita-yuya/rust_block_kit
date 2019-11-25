use serde::Deserialize;

/// Slack slash command payload.
/// This data will be sent with a Content-type header set as application/x-www-form-urlencoded.
///
/// Reference:
/// https://api.slack.com/interactivity/slash-commands
#[derive(Debug, Deserialize)]
pub struct SlashCommandPayload {
    pub command: String,
    pub text: String,
    pub response_url: String,
    pub trigger_id: String,
    pub user_id: String,
    pub user_name: String,
    pub team_id: Option<String>,
    pub enterprise_id: Option<String>,
    pub channel_id: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_minimum() {
        let json = r#"{
        "command": "command",
        "text": "text",
        "response_url": "response_url",
        "trigger_id": "trigger_id",
        "user_id": "user_id",
        "user_name": "user_name"
        }"#;
        let command: SlashCommandPayload = serde_json::from_str(json).unwrap();

        assert_eq!(command.command, "command");
        assert_eq!(command.text, "text");
        assert_eq!(command.response_url, "response_url");
        assert_eq!(command.trigger_id, "trigger_id");
        assert_eq!(command.user_id, "user_id");
        assert_eq!(command.user_name, "user_name");
        assert_eq!(command.team_id, None);
        assert_eq!(command.enterprise_id, None);
        assert_eq!(command.channel_id, None);
    }
}
