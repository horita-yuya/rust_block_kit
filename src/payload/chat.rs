use crate::payload::message::CommonMessagePayload;
use serde::Serialize;

/// Reference.
/// https://api.slack.com/messaging/sending#publishing_your_message
/// https://api.slack.com/methods/chat.postMessage
///
/// # chat.postMessage api's payload.
/// `channel` and `payload` are required to be perform as expected.
///
/// Outmoded approaches are not supported;
/// https://api.slack.com/docs/message-formatting
/// https://api.slack.com/docs/message-attachments
#[derive(Debug, Serialize)]
pub struct ChatPostMessagePayload {
    channel: String,
    #[serde(flatten)]
    payload: CommonMessagePayload,
    #[serde(skip_serializing_if = "Option::is_none")]
    as_user: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_broadcast: Option<bool>,
}

impl ChatPostMessagePayload {
    /// Use this to generate instance.
    pub fn new(channel: impl Into<String>, payload: CommonMessagePayload) -> Self {
        ChatPostMessagePayload {
            channel: channel.into(),
            payload,
            as_user: Option::default(),
            icon_emoji: Option::default(),
            icon_url: Option::default(),
            username: Option::default(),
            reply_broadcast: Option::default(),
        }
    }

    pub fn as_user(mut self, as_user: bool) -> Self {
        self.as_user = Some(as_user);
        self
    }

    /// `emoji` has higher priority than url.
    /// This means, if both of emoji and url are set, emoji will display.
    ///
    /// Must be used in conjunction with `as_user` set to false, otherwise ignored.
    pub fn icon_emoji(mut self, icon_emoji: impl Into<String>) -> Self {
        self.icon_emoji = Some(icon_emoji.into());
        self
    }

    /// `url` has lower priority than url.
    /// This means, if both of emoji and url are set, url won't display.
    ///
    /// Must be used in conjunction with `as_user` set to false, otherwise ignored.
    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }

    /// Bot's username.
    ///
    /// Must be used in conjunction with `as_user` set to false, otherwise ignored.
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn reply_broadcast(mut self, reply_broadcast: bool) -> Self {
        self.reply_broadcast = Some(reply_broadcast);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::block::section::SectionBlock;
    use crate::block::Block::Section;
    use crate::composition::text::PlainText;
    use crate::composition::text::Text::Plain;
    use crate::payload::message::CommonMessagePayload;

    #[test]
    fn test_ser_new() {
        let section = SectionBlock::new(Plain(PlainText::new("text")));
        let block = Section(section);
        let payload = CommonMessagePayload::new().blocks(vec![block]);
        let payload = ChatPostMessagePayload::new("channel", payload);
        let json = serde_json::to_string_pretty(&payload).unwrap();

        let expected = r#"{
  "channel": "channel",
  "blocks": [
    {
      "type": "section",
      "text": {
        "type": "plain_text",
        "text": "text"
      }
    }
  ]
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_all() {
        let section = SectionBlock::new(Plain(PlainText::new("text")));
        let block = Section(section);
        let payload = CommonMessagePayload::new().blocks(vec![block]);
        let payload = ChatPostMessagePayload::new("channel", payload)
            .as_user(false)
            .icon_emoji(":man:")
            .icon_url("url")
            .username("username")
            .reply_broadcast(false);
        let json = serde_json::to_string_pretty(&payload).unwrap();

        let expected = r#"{
  "channel": "channel",
  "blocks": [
    {
      "type": "section",
      "text": {
        "type": "plain_text",
        "text": "text"
      }
    }
  ],
  "as_user": false,
  "icon_emoji": ":man:",
  "icon_url": "url",
  "username": "username",
  "reply_broadcast": false
}"#;
        assert_eq!(json, expected);
    }
}
