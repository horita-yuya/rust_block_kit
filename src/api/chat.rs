use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatMessage {
    pub text: String,
    pub username: String,
    pub bot_id: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub subtype: String,
    pub ts: String,
}

/// Reference:
/// https://api.slack.com/methods/chat.postMessage
#[derive(Debug, Deserialize)]
pub struct PostMessageResponse {
    pub ok: bool,
    pub error: Option<String>,
    pub channel: Option<String>,
    pub ts: Option<String>,
    pub message: Option<ChatMessage>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_de_ok() {
        let json = r#"{
  "ok": true,
  "channel": "random",
  "ts": "150",
  "message": {
    "text": "hello",
    "username": "bot",
    "bot_id": "id",
    "type": "message",
    "subtype": "bot_message",
    "ts": "150"
  }
}"#;
        let res = serde_json::from_str::<PostMessageResponse>(json).unwrap();
        assert!(res.ok);
        assert_eq!(res.channel.unwrap(), "random");
        assert_eq!(res.ts.unwrap(), "150");

        let message = res.message.unwrap();
        assert_eq!(message.text, "hello");
        assert_eq!(message.username, "bot");
        assert_eq!(message.bot_id, "id");
        assert_eq!(message.type_name, "message");
        assert_eq!(message.subtype, "bot_message");
        assert_eq!(message.ts, "150");
    }

    #[test]
    fn test_de_err() {
        let json = r#"{
  "ok": false,
  "error": "error"
}"#;
        let res = serde_json::from_str::<PostMessageResponse>(json).unwrap();
        assert!(!res.ok);
        assert_eq!(res.error.unwrap(), "error");
    }
}
