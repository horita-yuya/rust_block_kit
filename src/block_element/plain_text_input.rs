use crate::block_element::PLAIN_TEXT_INPUT_TYPE;
use crate::composition::text::Text;
use serde::Serialize;

/// WARNING
/// Plain-text input elements are currently only available in modals
#[derive(Debug, Serialize)]
pub struct PlainTextInputElement {
    #[serde(rename = "type")]
    type_name: &'static str,
    action_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<u32>,
}

impl PlainTextInputElement {
    pub fn new(action_id: impl Into<String>) -> Self {
        PlainTextInputElement {
            type_name: PLAIN_TEXT_INPUT_TYPE,
            action_id: action_id.into(),
            placeholder: Option::default(),
            initial_value: Option::default(),
            multiline: Option::default(),
            min_length: Option::default(),
            max_length: Option::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ser_new() {
        let input = PlainTextInputElement::new("action_id");
        let json = serde_json::to_string_pretty(&input).unwrap();

        let expected = r#"{
  "type": "plain_text_input",
  "action_id": "action_id"
}"#;
        assert_eq!(json, expected);
    }
}
