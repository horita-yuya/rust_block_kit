use crate::block_element::MULTI_STATIC_SELECT_MENU_TYPE;
use crate::composition::confirmation_dialog::ConfirmationDialog;
use crate::composition::option::OptionObject;
use crate::composition::option_group::OptionGroup;
use crate::composition::text::Text::Plain;
use crate::composition::text::{PlainText, Text};
use serde::Serialize;

/// `initial_options`: Set an array of option objects that exactly match one or more op the options
/// with in `options` or `option_groups`
///
/// # Example:
/// ```rust
/// use block_kit::block_element::multi_select_menu::MultiStaticSelectMenuElement;
/// use block_kit::composition::option::OptionObject;
/// use block_kit::composition::option_group::OptionGroup;
/// use block_kit::composition::text::PlainText;
///
/// let multi = MultiStaticSelectMenuElement::new(PlainText::new("placeholder"), "ac")
///     .option_groups(vec![
///         OptionGroup::new(
///             PlainText::new("label1"),
///             vec![
///                 OptionObject::new(PlainText::new("t1"), "v1"),
///                 OptionObject::new(PlainText::new("t2"), "v2"),
///             ],
///         ),
///         OptionGroup::new(
///             PlainText::new("label2"),
///             vec![
///                 OptionObject::new(PlainText::new("t3"), "v3"),
///                 OptionObject::new(PlainText::new("t4"), "v4"),
///             ],
///         ),
///     ]);
///
/// // Setting initial_options across the groups causes error.
/// // Ok
/// let multi = multi.initial_options(vec![
///     OptionObject::new(PlainText::new("t1"), "v1"),
///     OptionObject::new(PlainText::new("t2"), "v2"),
/// ]);
///
/// // Error - different group
/// // let multi = multi.initial_options(vec![
/// //     OptionObject::new(PlainText::new("t1"), "v1"),
/// //     OptionObject::new(PlainText::new("t3"), "v3"),
/// // ]);
/// ```
#[derive(Debug, Serialize)]
pub struct MultiStaticSelectMenuElement {
    #[serde(rename = "type")]
    type_name: &'static str,
    placeholder: Text,
    action_id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    options: Vec<OptionObject>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    option_groups: Vec<OptionGroup>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    initial_options: Vec<OptionObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,
}

impl MultiStaticSelectMenuElement {
    pub fn new(placeholder: impl Into<PlainText>, action_id: impl Into<String>) -> Self {
        MultiStaticSelectMenuElement {
            type_name: MULTI_STATIC_SELECT_MENU_TYPE,
            placeholder: Plain(placeholder.into()),
            action_id: action_id.into(),
            options: Vec::default(),
            option_groups: Vec::default(),
            initial_options: Vec::default(),
            confirm: Option::default(),
        }
    }

    pub fn options(mut self, options: Vec<OptionObject>) -> Self {
        self.options = options;
        self
    }

    pub fn option_groups(mut self, option_groups: Vec<OptionGroup>) -> Self {
        self.option_groups = option_groups;
        self
    }

    pub fn initial_options(mut self, initial_options: Vec<OptionObject>) -> Self {
        self.initial_options = initial_options;
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
        let multi = MultiStaticSelectMenuElement::new("placeholder", "action_id");
        let json = serde_json::to_string_pretty(&multi).unwrap();

        let expected = r#"{
  "type": "multi_static_select",
  "placeholder": {
    "type": "plain_text",
    "text": "placeholder"
  },
  "action_id": "action_id"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_options() {
        let option = OptionObject::new(PlainText::new("text"), "value");
        let multi =
            MultiStaticSelectMenuElement::new("placeholder", "action_id").options(vec![option]);
        let json = serde_json::to_string_pretty(&multi).unwrap();

        let expected = r#"{
  "type": "multi_static_select",
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
    fn test_ser_option_groups() {
        let option = OptionObject::new(PlainText::new("text"), "value");
        let option_group = OptionGroup::new(PlainText::new("label"), vec![option]);
        let multi = MultiStaticSelectMenuElement::new("placeholder", "action_id")
            .option_groups(vec![option_group]);
        let json = serde_json::to_string_pretty(&multi).unwrap();

        let expected = r#"{
  "type": "multi_static_select",
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
    fn test_ser_initial_options() {
        let option = OptionObject::new(PlainText::new("text"), "value");
        let multi = MultiStaticSelectMenuElement::new("placeholder", "action_id")
            .initial_options(vec![option]);
        let json = serde_json::to_string_pretty(&multi).unwrap();

        let expected = r#"{
  "type": "multi_static_select",
  "placeholder": {
    "type": "plain_text",
    "text": "placeholder"
  },
  "action_id": "action_id",
  "initial_options": [
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
    fn test_ser_confirm() {
        let confirm = ConfirmationDialog::new(
            PlainText::new("title"),
            Plain(PlainText::new("text")),
            PlainText::new("confirm"),
            PlainText::new("deny"),
        );
        let multi = MultiStaticSelectMenuElement::new("placeholder", "action_id").confirm(confirm);
        let json = serde_json::to_string_pretty(&multi).unwrap();

        let expected = r#"{
  "type": "multi_static_select",
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
