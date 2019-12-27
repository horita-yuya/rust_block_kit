use crate::block::input::InputBlock;
use crate::block::Block::{Actions, Context, Divider, Image, Input, Section};
use serde::Serialize;

pub mod actions;
pub mod context;
pub mod divider;
pub mod image;
pub mod input;
pub mod section;

pub(self) const SECTION_TYPE: &str = "section";
pub(self) const ACTIONS_TYPE: &str = "actions";
pub(self) const CONTEXT_TYPE: &str = "context";
pub(self) const DIVIDER_TYPE: &str = "divider";
pub(self) const IMAGE_TYPE: &str = "image";
pub(self) const INPUT_TYPE: &str = "input";

/// Blocks are a series of components that can be combined to create visually rich and compellingly interactive messages.
///
/// `BlockType` : `Available in surfaces`
/// Actions : Modals, Messages, Home tabs
/// Context : Modals, Messages, Home tabs
/// Divider : Modals, Messages, Home tabs
/// File    : Messages
/// Image   : Modals, Messages, Home tabs
/// Input   : Modals
/// Section : Modals, Messages, Home tabs
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Block {
    Section(section::SectionBlock),
    Divider(divider::DividerBlock),
    Actions(actions::ActionsBlock),
    Image(image::ImageBlock),
    Context(context::ContextBlock),
    Input(input::InputBlock),
    /*    TODO:
     *    File   */
}

impl From<section::SectionBlock> for Block {
    fn from(block: section::SectionBlock) -> Self {
        Section(block)
    }
}

impl From<divider::DividerBlock> for Block {
    fn from(block: divider::DividerBlock) -> Self {
        Divider(block)
    }
}

impl From<actions::ActionsBlock> for Block {
    fn from(block: actions::ActionsBlock) -> Self {
        Actions(block)
    }
}

impl From<image::ImageBlock> for Block {
    fn from(block: image::ImageBlock) -> Self {
        Image(block)
    }
}

impl From<context::ContextBlock> for Block {
    fn from(block: context::ContextBlock) -> Self {
        Context(block)
    }
}

impl From<input::InputBlock> for Block {
    fn from(block: InputBlock) -> Self {
        Input(block)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::block::actions::ActionsBlock;
    use crate::block::divider::DividerBlock;
    use crate::block::section::SectionBlock;
    use crate::block_element::button::ButtonElement;
    use crate::block_element::BlockElement::Button;
    use crate::composition::text::PlainText;
    use crate::composition::text::Text::Plain;

    #[test]
    fn test_ser_section() {
        let block = Block::Section(SectionBlock::new(Plain(PlainText::new("text"))));
        let json = serde_json::to_string_pretty(&block).unwrap_or("".to_string());
        let expected = r#"{
  "type": "section",
  "text": {
    "type": "plain_text",
    "text": "text"
  }
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_divider() {
        let block = Block::Divider(DividerBlock::new());
        let json = serde_json::to_string_pretty(&block).unwrap_or("".to_string());
        let expected = r#"{
  "type": "divider"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_actions() {
        let block = Block::Actions(ActionsBlock::new(vec![Button(ButtonElement::new(
            PlainText::new("text"),
            "action_id",
        ))]));
        let json = serde_json::to_string_pretty(&block).unwrap_or("".to_string());
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
}
