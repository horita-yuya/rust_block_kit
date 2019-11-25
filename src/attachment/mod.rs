use crate::attachment::color::Color;
use crate::block::Block;
use serde::Serialize;

pub mod color;

/// WARNING
/// Legacy fields defined in `https://api.slack.com/reference/messaging/attachments` are not supported.
#[derive(Debug, Serialize)]
pub struct Attachment {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    blocks: Vec<Block>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
}

impl Attachment {
    pub fn new(blocks: Vec<Block>) -> Self {
        Attachment {
            blocks,
            color: None,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color.value());
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::attachment::color::Color::Good;
    use crate::block::section::SectionBlock;
    use crate::block::Block::Section;
    use crate::composition::text::PlainText;
    use crate::composition::text::Text::Plain;

    #[test]
    pub fn test_serializing_new() {
        let section = SectionBlock::new(Plain(PlainText::new("text")));
        let block = Section(section);
        let attachments = Attachment::new(vec![block]);
        let json = serde_json::to_string_pretty(&attachments).unwrap();

        let expected = r#"{
  "blocks": [
    {
      "type": "section",
      "text": {
        "type": "plain_text",
        "text": "text"
      }
    }
  ]
}"#;

        assert_eq!(json, expected);
    }

    #[test]
    pub fn test_serializing_color() {
        let section = SectionBlock::new(Plain(PlainText::new("text")));
        let block = Section(section);
        let attachments = Attachment::new(vec![block]).color(Good);
        let json = serde_json::to_string_pretty(&attachments).unwrap();

        let expected = r#"{
  "blocks": [
    {
      "type": "section",
      "text": {
        "type": "plain_text",
        "text": "text"
      }
    }
  ],
  "color": "good"
}"#;

        assert_eq!(json, expected);
    }
}
