use crate::block_element::STATIC_SELECT_MENU_TYPE;
use crate::composition::confirmation_dialog::ConfirmationDialog;
use crate::composition::option::OptionObject;
use crate::composition::option_group::OptionGroup;
use crate::composition::text::Text::Plain;
use crate::composition::text::{PlainText, Text};
use serde::Serialize;

/// requires set options or option_groups to display.
#[derive(Debug, Serialize)]
pub struct StaticSelectMenuElement {
    #[serde(rename = "type")]
    type_name: &'static str,
    placeholder: Text,
    action_id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    options: Vec<OptionObject>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    option_groups: Vec<OptionGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_option: Option<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,
}

impl StaticSelectMenuElement {
    pub fn new(placeholder: impl Into<PlainText>, action_id: impl Into<String>) -> Self {
        StaticSelectMenuElement {
            type_name: STATIC_SELECT_MENU_TYPE,
            placeholder: Plain(placeholder.into()),
            action_id: action_id.into(),
            options: Vec::default(),
            option_groups: Vec::default(),
            initial_option: Option::default(),
            confirm: Option::default(),
        }
    }

    /// option_groups should not be if options is specified.
    pub fn options(mut self, options: Vec<OptionObject>) -> Self {
        self.options = options;
        self.option_groups = vec![];
        self
    }

    /// options should not be if option_groups is specified.
    pub fn option_groups(mut self, option_groups: Vec<OptionGroup>) -> Self {
        self.option_groups = option_groups;
        self.options = vec![];
        self
    }

    pub fn initial_option(mut self, initial_option: OptionObject) -> Self {
        self.initial_option = Some(initial_option);
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
        let menu = StaticSelectMenuElement::new("placeholder", "action_id");
        let json = serde_json::to_string_pretty(&menu).unwrap();

        let expected = r#"{
  "type": "static_select",
  "placeholder": {
    "type": "plain_text",
    "text": "placeholder"
  },
  "action_id": "action_id"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_option() {
        let option = OptionObject::new(PlainText::new("text"), "value");
        let menu = StaticSelectMenuElement::new("placeholder", "action_id").options(vec![option]);
        let json = serde_json::to_string_pretty(&menu).unwrap();

        let expected = r#"{
  "type": "static_select",
  "placeholder": {
    "type": "plain_text",
    "text": "placeholder"
  },
  "action_id": "action_id",
  "options": [
    {
      "text": {
        "type": "plain_text",
        "text": "text"
      },
      "value": "value"
    }
  ]
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_option_group() {
        let option = OptionObject::new(PlainText::new("text"), "value");
        let group = OptionGroup::new(PlainText::new("label"), vec![option]);
        let menu =
            StaticSelectMenuElement::new("placeholder", "action_id").option_groups(vec![group]);
        let json = serde_json::to_string_pretty(&menu).unwrap();

        let expected = r#"{
  "type": "static_select",
  "placeholder": {
    "type": "plain_text",
    "text": "placeholder"
  },
  "action_id": "action_id",
  "option_groups": [
    {
      "label": {
        "type": "plain_text",
        "text": "label"
      },
      "options": [
        {
          "text": {
            "type": "plain_text",
            "text": "text"
          },
          "value": "value"
        }
      ]
    }
  ]
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_initial_option() {
        let option = OptionObject::new(PlainText::new("text"), "value");
        let menu = StaticSelectMenuElement::new("placeholder", "action_id").initial_option(option);
        let json = serde_json::to_string_pretty(&menu).unwrap();

        let expected = r#"{
  "type": "static_select",
  "placeholder": {
    "type": "plain_text",
    "text": "placeholder"
  },
  "action_id": "action_id",
  "initial_option": {
    "text": {
      "type": "plain_text",
      "text": "text"
    },
    "value": "value"
  }
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_initial_confirm() {
        let confirm = ConfirmationDialog::new(
            PlainText::new("title"),
            Plain(PlainText::new("text")),
            PlainText::new("confirm"),
            PlainText::new("deny"),
        );
        let menu = StaticSelectMenuElement::new("placeholder", "action_id").confirm(confirm);
        let json = serde_json::to_string_pretty(&menu).unwrap();

        let expected = r#"{
  "type": "static_select",
  "placeholder": {
    "type": "plain_text",
    "text": "placeholder"
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
}
