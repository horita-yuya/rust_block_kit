use crate::block::ACTIONS_TYPE;
use crate::block_element::BlockElement;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ActionsBlock {
    #[serde(rename = "type")]
    type_name: &'static str,
    elements: Vec<BlockElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl ActionsBlock {
    pub fn new(elements: Vec<BlockElement>) -> Self {
        ActionsBlock {
            type_name: ACTIONS_TYPE,
            elements,
            block_id: Option::default(),
        }
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
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
        let actions = ActionsBlock::new(vec![Button(ButtonElement::new(
            PlainText::new("text"),
            "action_id",
        ))]);
        let json = serde_json::to_string_pretty(&actions).unwrap_or("".to_string());
        let expected = r#"{
  "type": "actions",
  "elements": [
    {
      "type": "button",
      "text": {
        "type": "plain_text",
        "text": "text"
      },
      "action_id": "action_id"
    }
  ]
}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_block_id() {
        let actions = ActionsBlock::new(vec![Button(ButtonElement::new(
            PlainText::new("text"),
            "action_id",
        ))])
        .block_id("block");
        let json = serde_json::to_string_pretty(&actions).unwrap_or("".to_string());
        let expected = r#"{
  "type": "actions",
  "elements": [
    {
      "type": "button",
      "text": {
        "type": "plain_text",
        "text": "text"
      },
      "action_id": "action_id"
    }
  ],
  "block_id": "block"
}"#;

        assert_eq!(json, expected);
    }
}
