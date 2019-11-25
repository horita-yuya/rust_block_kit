use crate::block_element::OVERFLOW_MENU_TYPE;
use crate::composition::confirmation_dialog::ConfirmationDialog;
use crate::composition::option::OptionObject;
use serde::Serialize;

#[derive(Clone)]
pub enum OverflowMenuOption {
    TwoOptions([OptionObject; 2]),
    ThreeOptions([OptionObject; 3]),
    FourOptions([OptionObject; 4]),
    FiveOptions([OptionObject; 5]),
}

#[derive(Debug, Serialize)]
pub struct OverflowMenuElement {
    #[serde(rename = "type")]
    type_name: &'static str,
    action_id: String,
    options: Vec<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,
}

impl OverflowMenuElement {
    pub fn new(action_id: impl Into<String>, options: OverflowMenuOption) -> Self {
        let options = match options {
            self::OverflowMenuOption::TwoOptions(val) => val.to_vec(),
            self::OverflowMenuOption::ThreeOptions(val) => val.to_vec(),
            self::OverflowMenuOption::FourOptions(val) => val.to_vec(),
            self::OverflowMenuOption::FiveOptions(val) => val.to_vec(),
        };

        OverflowMenuElement {
            type_name: OVERFLOW_MENU_TYPE,
            action_id: action_id.into(),
            options,
            confirm: Option::default(),
        }
    }

    pub fn confirm(mut self, confirm: ConfirmationDialog) -> Self {
        self.confirm = Some(confirm);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::block_element::overflow_menu::OverflowMenuOption::TwoOptions;
    use crate::composition::text::PlainText;
    use crate::composition::text::Text::Plain;

    #[test]
    fn test_ser_new() {
        let option1 = OptionObject::new(PlainText::new("text1"), "value1");
        let option2 = OptionObject::new(PlainText::new("text2"), "value2");
        let menu = OverflowMenuElement::new("action", TwoOptions([option1, option2]));
        let json = serde_json::to_string_pretty(&menu).unwrap_or("".to_string());
        let expected = r#"{
  "type": "overflow",
  "action_id": "action",
  "options": [
    {
      "text": {
        "type": "plain_text",
        "text": "text1"
      },
      "value": "value1"
    },
    {
      "text": {
        "type": "plain_text",
        "text": "text2"
      },
      "value": "value2"
    }
  ]
}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_confirm() {
        let option1 = OptionObject::new(PlainText::new("text1"), "value1");
        let option2 = OptionObject::new(PlainText::new("text2"), "value2");
        let confirm = ConfirmationDialog::new(
            PlainText::new("title"),
            Plain(PlainText::new("text")),
            PlainText::new("confirm"),
            PlainText::new("deny"),
        );

        let menu =
            OverflowMenuElement::new("action", TwoOptions([option1, option2])).confirm(confirm);
        let json = serde_json::to_string_pretty(&menu).unwrap_or("".to_string());
        let expected = r#"{
  "type": "overflow",
  "action_id": "action",
  "options": [
    {
      "text": {
        "type": "plain_text",
        "text": "text1"
      },
      "value": "value1"
    },
    {
      "text": {
        "type": "plain_text",
        "text": "text2"
      },
      "value": "value2"
    }
  ],
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
