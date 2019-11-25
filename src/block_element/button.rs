use crate::block_element::BUTTON_TYPE;
use crate::composition::confirmation_dialog::ConfirmationDialog;
use crate::composition::text::Text::Plain;
use crate::composition::text::{PlainText, Text};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ButtonElement {
    #[serde(rename = "type")]
    type_name: &'static str,
    text: Text,
    action_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<Style>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Style {
    Primary,
    Danger,
}

impl ButtonElement {
    pub fn new(text: impl Into<PlainText>, action_id: impl Into<String>) -> Self {
        ButtonElement {
            type_name: BUTTON_TYPE,
            text: Plain(text.into()),
            action_id: action_id.into(),
            url: Option::default(),
            value: Option::default(),
            style: Option::default(),
            confirm: Option::default(),
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }

    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::block_element::button::Style::{Danger, Primary};

    #[test]
    fn test_ser_new() {
        let button = ButtonElement::new(PlainText::new("text"), "action_id");
        let json = serde_json::to_string_pretty(&button).unwrap_or("".to_string());
        let expected = r#"{
  "type": "button",
  "text": {
    "type": "plain_text",
    "text": "text"
  },
  "action_id": "action_id"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_url() {
        let button = ButtonElement::new(PlainText::new("text"), "action_id").url("url");
        let json = serde_json::to_string_pretty(&button).unwrap_or("".to_string());
        let expected = r#"{
  "type": "button",
  "text": {
    "type": "plain_text",
    "text": "text"
  },
  "action_id": "action_id",
  "url": "url"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_value() {
        let button = ButtonElement::new(PlainText::new("text"), "action_id").value("value");
        let json = serde_json::to_string_pretty(&button).unwrap_or("".to_string());
        let expected = r#"{
  "type": "button",
  "text": {
    "type": "plain_text",
    "text": "text"
  },
  "action_id": "action_id",
  "value": "value"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_style() {
        let button = ButtonElement::new(PlainText::new("text"), "action_id").style(Primary);
        let json = serde_json::to_string_pretty(&button).unwrap_or("".to_string());
        let expected = r#"{
  "type": "button",
  "text": {
    "type": "plain_text",
    "text": "text"
  },
  "action_id": "action_id",
  "style": "primary"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_confirm() {
        let confirm = ConfirmationDialog::new(
            PlainText::new("title"),
            Plain(PlainText::new("text")),
            PlainText::new("confirm"),
            PlainText::new("deny"),
        );
        let button = ButtonElement::new(PlainText::new("text"), "action_id").confirm(confirm);
        let json = serde_json::to_string_pretty(&button).unwrap_or("".to_string());
        let expected = r#"{
  "type": "button",
  "text": {
    "type": "plain_text",
    "text": "text"
  },
  "action_id": "action_id",
  "confirm": {
    "title": {
      "type": "plain_text",
      "text": "title"
    },
    "text": {
      "type": "plain_text",
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
  }
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_all() {
        let confirm = ConfirmationDialog::new(
            PlainText::new("title"),
            Plain(PlainText::new("text")),
            PlainText::new("confirm"),
            PlainText::new("deny"),
        );
        let button = ButtonElement::new(PlainText::new("text"), "action_id")
            .url("url")
            .value("value")
            .style(Danger)
            .confirm(confirm);
        let json = serde_json::to_string_pretty(&button).unwrap_or("".to_string());
        let expected = r#"{
  "type": "button",
  "text": {
    "type": "plain_text",
    "text": "text"
  },
  "action_id": "action_id",
  "url": "url",
  "value": "value",
  "style": "danger",
  "confirm": {
    "title": {
      "type": "plain_text",
      "text": "title"
    },
    "text": {
      "type": "plain_text",
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
  }
}"#;
        assert_eq!(json, expected);
    }
}
