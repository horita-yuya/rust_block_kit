use crate::block::SECTION_TYPE;
use crate::block_element::BlockElement;
use crate::composition::text::Text;
use serde::Serialize;

/// A section is one of the most flexible blocks available.
/// - simple text
/// - multiple text fields
/// - `Element` accessory
#[derive(Debug, Serialize)]
pub struct SectionBlock {
    #[serde(rename = "type")]
    type_name: &'static str,
    text: Text,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fields: Vec<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accessory: Option<BlockElement>,
}

impl SectionBlock {
    pub fn new(text: Text) -> Self {
        SectionBlock {
            type_name: SECTION_TYPE,
            text,
            block_id: Option::default(),
            fields: Vec::default(),
            accessory: Option::default(),
        }
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }

    pub fn fields(mut self, fields: Vec<Text>) -> Self {
        self.fields = fields;
        self
    }

    pub fn accessory(mut self, accessory: BlockElement) -> Self {
        self.accessory = Some(accessory);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::block_element::button::ButtonElement;
    use crate::block_element::BlockElement::Button;
    use crate::composition::text::PlainText;

    #[test]
    fn test_ser_new() {
        let section = SectionBlock::new(Text::default());
        let json = serde_json::to_string_pretty(&section).unwrap_or("".to_string());
        let expected = r#"{
  "type": "section",
  "text": {
    "type": "plain_text",
    "text": ""
  }
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_block_id() {
        let section = SectionBlock::new(Text::default()).block_id("block");
        let json = serde_json::to_string_pretty(&section).unwrap_or("".to_string());
        let expected = r#"{
  "type": "section",
  "text": {
    "type": "plain_text",
    "text": ""
  },
  "block_id": "block"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_fields() {
        let section = SectionBlock::new(Text::default()).fields(vec![Text::default()]);
        let json = serde_json::to_string_pretty(&section).unwrap_or("".to_string());
        let expected = r#"{
  "type": "section",
  "text": {
    "type": "plain_text",
    "text": ""
  },
  "fields": [
    {
      "type": "plain_text",
      "text": ""
    }
  ]
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_accessory() {
        let button = ButtonElement::new(PlainText::new("text"), "action_id");
        let element = Button(button);
        let section = SectionBlock::new(Text::default()).accessory(element);
        let json = serde_json::to_string_pretty(&section).unwrap_or("".to_string());
        let expected = r#"{
  "type": "section",
  "text": {
    "type": "plain_text",
    "text": ""
  },
  "accessory": {
    "type": "button",
    "text": {
      "type": "plain_text",
      "text": "text"
    },
    "action_id": "action_id"
  }
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_all() {
        let button = ButtonElement::new(PlainText::new("text"), "action_id");
        let element = Button(button);
        let section = SectionBlock::new(Text::default())
            .block_id("block")
            .fields(vec![Text::default()])
            .accessory(element);

        let json = serde_json::to_string_pretty(&section).unwrap_or("".to_string());
        let expected = r#"{
  "type": "section",
  "text": {
    "type": "plain_text",
    "text": ""
  },
  "block_id": "block",
  "fields": [
    {
      "type": "plain_text",
      "text": ""
    }
  ],
  "accessory": {
    "type": "button",
    "text": {
      "type": "plain_text",
      "text": "text"
    },
    "action_id": "action_id"
  }
}"#;
        assert_eq!(json, expected);
    }
}
