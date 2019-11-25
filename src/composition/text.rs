use crate::composition::text::Text::Plain;
use serde::Serialize;

/// An object containing some text, formatted either as `plain_text` or using `mrkdwn`,
/// our proprietary textual markup that's just different enough from Markdown to frustrate you.

const PLAIN_TEXT: &str = "plain_text";
const MARKDOWN: &str = "mrkdwn";

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum Text {
    Plain(PlainText),
    Markdown(MarkdownText),
}

#[derive(Debug, Serialize, Clone)]
pub struct PlainText {
    #[serde(rename = "type")]
    type_name: &'static str,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<bool>,
}

#[derive(Debug, Serialize, Clone)]
pub struct MarkdownText {
    #[serde(rename = "type")]
    type_name: &'static str,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    verbatim: Option<bool>,
}

impl From<&str> for PlainText {
    fn from(s: &str) -> Self {
        PlainText::new(s)
    }
}

impl From<String> for PlainText {
    fn from(s: String) -> Self {
        PlainText::new(s)
    }
}

impl From<&str> for MarkdownText {
    fn from(s: &str) -> Self {
        MarkdownText::new(s)
    }
}

impl From<String> for MarkdownText {
    fn from(s: String) -> Self {
        MarkdownText::new(s)
    }
}

impl Default for Text {
    fn default() -> Self {
        Plain(PlainText::default())
    }
}

impl Default for PlainText {
    fn default() -> Self {
        PlainText {
            type_name: PLAIN_TEXT,
            text: String::default(),
            emoji: Option::default(),
        }
    }
}

impl Default for MarkdownText {
    fn default() -> Self {
        MarkdownText {
            type_name: MARKDOWN,
            text: String::default(),
            verbatim: Option::default(),
        }
    }
}

impl PlainText {
    pub fn new(text: impl Into<String>) -> Self {
        PlainText {
            text: text.into(),
            ..PlainText::default()
        }
    }

    pub fn emoji(mut self, emoji: bool) -> Self {
        self.emoji = Some(emoji);
        self
    }
}

impl MarkdownText {
    pub fn new(text: impl Into<String>) -> Self {
        MarkdownText {
            text: text.into(),
            ..MarkdownText::default()
        }
    }

    pub fn verbatim(mut self, verbatim: bool) -> Self {
        self.verbatim = Some(verbatim);
        self
    }
}

#[cfg(test)]
mod test {
    use crate::composition::text::{MarkdownText, PlainText, Text};

    #[test]
    fn test_ser_new() {
        let text = Text::Plain(PlainText::new("plain"));
        let json = serde_json::to_string_pretty(&text).unwrap();
        let expected = r#"{
  "type": "plain_text",
  "text": "plain"
}"#;
        assert_eq!(json, expected);

        let text = Text::Markdown(MarkdownText::new("*markdown*"));
        let json = serde_json::to_string_pretty(&text).unwrap();
        let expected = r#"{
  "type": "mrkdwn",
  "text": "*markdown*"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_plain() {
        let plain: PlainText = "plain".into();
        let json = serde_json::to_string_pretty(&plain).unwrap();
        let expected = r#"{
  "type": "plain_text",
  "text": "plain"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_plain_emoji() {
        let plain: PlainText = "plain".into();
        let plain = plain.emoji(false);
        let json = serde_json::to_string_pretty(&plain).unwrap();
        let expected = r#"{
  "type": "plain_text",
  "text": "plain",
  "emoji": false
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_markdown() {
        let markdown: MarkdownText = "*markdown*".into();
        let json = serde_json::to_string_pretty(&markdown).unwrap();
        let expected = r#"{
  "type": "mrkdwn",
  "text": "*markdown*"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_markdown_verbatim() {
        let markdown: MarkdownText = "*markdown*".into();
        let markdown = markdown.verbatim(false);
        let json = serde_json::to_string_pretty(&markdown).unwrap();
        let expected = r#"{
  "type": "mrkdwn",
  "text": "*markdown*",
  "verbatim": false
}"#;
        assert_eq!(json, expected);
    }
}
