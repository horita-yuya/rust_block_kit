use crate::composition::text::Text::Plain;
use crate::composition::text::{PlainText, Text};
use serde::Serialize;

/// An object that represents a single selectable item in a select menu,
/// multi-select menu, radio button group, or overflow menu.
#[derive(Debug, Default, Serialize, Clone)]
pub struct OptionObject {
    text: Text,
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

impl OptionObject {
    pub fn new(text: impl Into<PlainText>, value: impl Into<String>) -> Self {
        OptionObject {
            text: Plain(text.into()),
            value: value.into(),
            ..OptionObject::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ser_new() {
        let option = OptionObject::new(PlainText::new("text"), "value");
        let json = serde_json::to_string_pretty(&option).unwrap_or("".to_string());
        let expected = r#"{
  "text": {
    "type": "plain_text",
    "text": "text"
  },
  "value": "value"
}"#;
        assert_eq!(json, expected.to_string());
    }
}
