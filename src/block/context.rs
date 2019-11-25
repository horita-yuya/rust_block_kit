use crate::block::CONTEXT_TYPE;
use crate::block_element::image::ImageElement;
use crate::composition::text::Text;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ContextBlockElement {
    ImageContext(ImageElement),
    TextContext(Text),
}

#[derive(Debug, Serialize)]
pub struct ContextBlock {
    #[serde(rename = "type")]
    type_name: &'static str,
    elements: Vec<ContextBlockElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl ContextBlock {
    pub fn new(elements: Vec<ContextBlockElement>) -> Self {
        ContextBlock {
            type_name: CONTEXT_TYPE,
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
    use crate::block::context::ContextBlockElement::{ImageContext, TextContext};
    use crate::composition::text::MarkdownText;
    use crate::composition::text::Text::Markdown;

    #[test]
    fn test_ser_new() {
        let elem1 = TextContext(Markdown(MarkdownText::new("*markdown*")));
        let elem2 = ImageContext(ImageElement::new("image_url", "alt_text"));
        let context = ContextBlock::new(vec![elem1, elem2]);
        let json = serde_json::to_string_pretty(&context).unwrap();

        let expected = r#"{
  "type": "context",
  "elements": [
    {
      "type": "mrkdwn",
      "text": "*markdown*"
    },
    {
      "type": "image",
      "image_url": "image_url",
      "alt_text": "alt_text"
    }
  ]
}"#;
        assert_eq!(json, expected);
    }
}
