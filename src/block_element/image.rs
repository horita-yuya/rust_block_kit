use crate::block_element::IMAGE_TYPE;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ImageElement {
    #[serde(rename = "type")]
    type_name: &'static str,
    image_url: String,
    alt_text: String,
}

impl ImageElement {
    pub fn new(image_url: impl Into<String>, alt_text: impl Into<String>) -> Self {
        ImageElement {
            type_name: IMAGE_TYPE,
            image_url: image_url.into(),
            alt_text: alt_text.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ser_new() {
        let image = ImageElement::new("url", "alt");
        let json = serde_json::to_string_pretty(&image).unwrap_or("".to_string());
        let expected = r#"{
  "type": "image",
  "image_url": "url",
  "alt_text": "alt"
}"#;
        assert_eq!(json, expected);
    }
}
