use crate::composition::text::Text::Plain;
use crate::composition::text::{PlainText, Text};
use serde::Serialize;

/// An object that defines a dialog that provides a confirmation step to any interactive element.
/// This dialog will ask the user to confirm their action by offering a confirm and deny buttons.
#[derive(Debug, Default, Serialize, Clone)]
pub struct ConfirmationDialog {
    title: Text,
    text: Text,
    confirm: Text,
    deny: Text,
}

impl ConfirmationDialog {
    pub fn new(
        title: impl Into<PlainText>,
        text: Text,
        confirm: impl Into<PlainText>,
        deny: impl Into<PlainText>,
    ) -> Self {
        ConfirmationDialog {
            title: Plain(title.into()),
            text,
            confirm: Plain(confirm.into()),
            deny: Plain(deny.into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::composition::text::MarkdownText;
    use crate::composition::text::Text::Markdown;

    #[test]
    pub fn test_ser_default() {
        let confirm = ConfirmationDialog::default();
        let json = serde_json::to_string_pretty(&confirm).unwrap_or("".to_string());
        let expected = r#"{
  "title": {
    "type": "plain_text",
    "text": ""
  },
  "text": {
    "type": "plain_text",
    "text": ""
  },
  "confirm": {
    "type": "plain_text",
    "text": ""
  },
  "deny": {
    "type": "plain_text",
    "text": ""
  }
}"#;
        assert_eq!(json, expected.to_string());
    }

    #[test]
    pub fn test_ser_new() {
        let confirm = ConfirmationDialog::new(
            PlainText::new("title"),
            Markdown(MarkdownText::new("text")),
            PlainText::new("confirm"),
            PlainText::new("deny"),
        );
        let json = serde_json::to_string_pretty(&confirm).unwrap_or("".to_string());
        let expected = r#"{
  "title": {
    "type": "plain_text",
    "text": "title"
  },
  "text": {
    "type": "mrkdwn",
    "text": "text"
  },
  "confirm": {
    "type": "plain_text",
    "text": "confirm"
  },
  "deny": {
    "type": "plain_text",
    "text": "deny"
  }
}"#;
        assert_eq!(json, expected.to_string());
    }
}
