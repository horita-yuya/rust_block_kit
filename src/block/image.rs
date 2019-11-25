use crate::block::IMAGE_TYPE;
use crate::composition::text::Text::Plain;
use crate::composition::text::{PlainText, Text};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ImageBlock {
    #[serde(rename = "type")]
    type_name: &'static str,
    image_url: String,
    alt_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Text>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl ImageBlock {
    pub fn new(image_url: impl Into<String>, alt_text: impl Into<String>) -> Self {
        ImageBlock {
            type_name: IMAGE_TYPE,
            image_url: image_url.into(),
            alt_text: alt_text.into(),
            title: Option::default(),
            block_id: Option::default(),
        }
    }

    pub fn title(mut self, title: impl Into<PlainText>) -> Self {
        self.title = Some(Plain(title.into()));
        self
    }

    pub fn block_id(mut self, block_id: impl Into<String>) -> Self {
        self.block_id = Some(block_id.into());
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ser_new() {
        let image = ImageBlock::new("url", "alt");
        let json = serde_json::to_string_pretty(&image).unwrap_or("".to_string());
        let expected = r#"{
  "type": "image",
  "image_url": "url",
  "alt_text": "alt"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_title() {
        let image = ImageBlock::new("url", "alt").title(PlainText::new("title"));
        let json = serde_json::to_string_pretty(&image).unwrap_or("".to_string());
        let expected = r#"{
  "type": "image",
  "image_url": "url",
  "alt_text": "alt",
  "title": {
    "type": "plain_text",
    "text": "title"
  }
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_block_id() {
        let image = ImageBlock::new("url", "alt").block_id("id");
        let json = serde_json::to_string_pretty(&image).unwrap_or("".to_string());
        let expected = r#"{
  "type": "image",
  "image_url": "url",
  "alt_text": "alt",
  "block_id": "id"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_all() {
        let image = ImageBlock::new("url", "alt")
            .title(PlainText::new("title"))
            .block_id("id");
        let json = serde_json::to_string_pretty(&image).unwrap_or("".to_string());
        let expected = r#"{
  "type": "image",
  "image_url": "url",
  "alt_text": "alt",
  "title": {
    "type": "plain_text",
    "text": "title"
  },
  "block_id": "id"
}"#;
        assert_eq!(json, expected);
    }
}
