use crate::attachment::Attachment;
use crate::block::Block;
use serde::Serialize;

/// Common base structure for Slack APIs that publish message.
/// Some additional fields may be required.
/// Reference: `https://api.slack.com/reference/messaging/payload`
#[derive(Debug, Default, Serialize)]
pub struct CommonMessagePayload {
    /// if `blocks` is specified, this is used as fallback string to display in notifications.
    /// if not used as plain text or markdown text.
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mrkdwn")]
    markdown: Option<bool>,

    /// Layout blocks
    #[serde(skip_serializing_if = "Vec::is_empty")]
    blocks: Vec<Block>,

    /// Legacy secondary attachments.
    /// Includes lower priority content - content that doesn't necessarily need to be seen.
    /// Using `blocks` is recommended rather than this. See `https://api.slack.com/messaging/attachments-to-blocks`
    #[serde(skip_serializing_if = "Vec::is_empty")]
    attachments: Vec<Attachment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thread_ts: Option<String>,
}

impl CommonMessagePayload {
    pub fn new() -> Self {
        CommonMessagePayload::default()
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn blocks(mut self, blocks: Vec<Block>) -> Self {
        self.blocks = blocks;
        self
    }

    pub fn attachments(mut self, attachments: Vec<Attachment>) -> Self {
        self.attachments = attachments;
        self
    }

    pub fn thread_ts(mut self, thread_ts: impl Into<String>) -> Self {
        self.thread_ts = Some(thread_ts.into());
        self
    }

    pub fn markdown(mut self, markdown: bool) -> Self {
        self.markdown = Some(markdown);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::block::divider::DividerBlock;
    use crate::block::image::ImageBlock;
    use crate::block::section::SectionBlock;
    use crate::block::Block::{Divider, Image, Section};
    use crate::composition::text::PlainText;
    use crate::composition::text::Text::Plain;

    #[test]
    fn test_ser_blocks() {
        let block: Vec<Block> = vec![
            Section(SectionBlock::new(Plain(PlainText::new("text")))),
            Divider(DividerBlock::new()),
            Image(ImageBlock::new("image_url", "alt_text")),
        ];
        let payload = CommonMessagePayload::new().blocks(block);
        let json = serde_json::to_string_pretty(&payload).unwrap();

        let expected = r#"{
  "blocks": [
    {
      "type": "section",
      "text": {
        "type": "plain_text",
        "text": "text"
      }
    },
    {
      "type": "divider"
    },
    {
      "type": "image",
      "image_url": "image_url",
      "alt_text": "alt_text"
    }
  ]
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_attachments() {
        let payload =
            CommonMessagePayload::new().attachments(vec![Attachment::new(vec![Section(
                SectionBlock::new(Plain(PlainText::new("text"))),
            )])]);
        let json = serde_json::to_string_pretty(&payload).unwrap();

        let expected = r#"{
  "attachments": [
    {
      "blocks": [
        {
          "type": "section",
          "text": {
            "type": "plain_text",
            "text": "text"
          }
        }
      ]
    }
  ]
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_text() {
        let payload = CommonMessagePayload::new().text("text");
        let json = serde_json::to_string_pretty(&payload).unwrap();

        let expected = r#"{
  "text": "text"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_thread_ts() {
        let payload = CommonMessagePayload::new().thread_ts("thread_ts");
        let json = serde_json::to_string_pretty(&payload).unwrap();

        let expected = r#"{
  "thread_ts": "thread_ts"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_markdown() {
        let payload = CommonMessagePayload::new().markdown(false);
        let json = serde_json::to_string_pretty(&payload).unwrap();

        let expected = r#"{
  "mrkdwn": false
}"#;
        assert_eq!(json, expected);
    }
}
