use crate::block_element::DATE_PICKER_TYPE;
use crate::composition::confirmation_dialog::ConfirmationDialog;
use crate::composition::text::Text::Plain;
use crate::composition::text::{PlainText, Text};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DatePickerElement {
    #[serde(rename = "type")]
    type_name: &'static str,
    action_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,
}

impl DatePickerElement {
    pub fn new(action_id: &str) -> Self {
        DatePickerElement {
            type_name: DATE_PICKER_TYPE,
            action_id: action_id.to_string(),
            placeholder: Option::default(),
            initial_date: Option::default(),
            confirm: Option::default(),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<PlainText>) -> Self {
        self.placeholder = Some(Plain(placeholder.into()));
        self
    }

    pub fn initial_date(mut self, initial_date: impl Into<String>) -> Self {
        self.initial_date = Some(initial_date.into());
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

    #[test]
    fn test_ser_new() {
        let date_picker = DatePickerElement::new("action_id");
        let json = serde_json::to_string_pretty(&date_picker).unwrap();

        let expected = r#"{
  "type": "datepicker",
  "action_id": "action_id"
}"#;
        assert_eq!(json, expected)
    }

    #[test]
    fn test_ser_placeholder() {
        let date_picker =
            DatePickerElement::new("action_id").placeholder(PlainText::new("placeholder"));
        let json = serde_json::to_string_pretty(&date_picker).unwrap();

        let expected = r#"{
  "type": "datepicker",
  "action_id": "action_id",
  "placeholder": {
    "type": "plain_text",
    "text": "placeholder"
  }
}"#;
        assert_eq!(json, expected);

        let date_picker = DatePickerElement::new("action_id").placeholder("placeholder2");
        let json = serde_json::to_string_pretty(&date_picker).unwrap();

        let expected = r#"{
  "type": "datepicker",
  "action_id": "action_id",
  "placeholder": {
    "type": "plain_text",
    "text": "placeholder2"
  }
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_initial_date() {
        let date_picker = DatePickerElement::new("action_id").initial_date("2013-12-11");
        let json = serde_json::to_string_pretty(&date_picker).unwrap();

        let expected = r#"{
  "type": "datepicker",
  "action_id": "action_id",
  "initial_date": "2013-12-11"
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
        let date_picker = DatePickerElement::new("action_id").confirm(confirm);
        let json = serde_json::to_string_pretty(&date_picker).unwrap();

        let expected = r#"{
  "type": "datepicker",
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
        let date_picker = DatePickerElement::new("action_id")
            .placeholder(PlainText::new("placeholder"))
            .initial_date("2013-12-11")
            .confirm(confirm);
        let json = serde_json::to_string_pretty(&date_picker).unwrap();

        let expected = r#"{
  "type": "datepicker",
  "action_id": "action_id",
  "placeholder": {
    "type": "plain_text",
    "text": "placeholder"
  },
  "initial_date": "2013-12-11",
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
