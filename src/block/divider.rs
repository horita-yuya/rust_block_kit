use crate::block::DIVIDER_TYPE;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DividerBlock {
    #[serde(rename = "type")]
    type_name: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl DividerBlock {
    pub fn new() -> Self {
        DividerBlock {
            type_name: DIVIDER_TYPE,
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

    #[test]
    fn test_s_new() {
        let divider = DividerBlock::new();
        let json = serde_json::to_string_pretty(&divider).unwrap_or("".to_string());

        let expected = r#"{
  "type": "divider"
}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_block_id() {
        let divider = DividerBlock::new().block_id("block");
        let json = serde_json::to_string_pretty(&divider).unwrap_or("".to_string());

        let expected = r#"{
  "type": "divider",
  "block_id": "block"
}"#;
        assert_eq!(json, expected);
    }
}
