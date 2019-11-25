use crate::payload::message::CommonMessagePayload;
use serde::Serialize;

pub mod block_actions;

/// Publishing messages back to the place where the interaction happened,
/// using `response_url` which is in interactive component payload, like
/// `BlockActionsPayload`.
#[derive(Debug, Default, Serialize)]
pub struct InteractiveRespondPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    payload: Option<CommonMessagePayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_type: Option<ResponseType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace_original: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_original: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    InChannel,
    Ephemeral,
}

impl InteractiveRespondPayload {
    pub fn new() -> Self {
        InteractiveRespondPayload::default()
    }

    pub fn payload(mut self, payload: CommonMessagePayload) -> Self {
        self.payload = Some(payload);
        self
    }

    pub fn response_type(mut self, response_type: ResponseType) -> Self {
        self.response_type = Some(response_type);
        self
    }

    pub fn replace_original(mut self, replace_original: bool) -> Self {
        self.replace_original = Some(replace_original);
        self
    }

    pub fn delete_original(mut self, delete_original: bool) -> Self {
        self.delete_original = Some(delete_original);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::payload::interaction::ResponseType::InChannel;

    #[test]
    fn test_ser_new() {
        let payload = InteractiveRespondPayload::new();
        let json = serde_json::to_string_pretty(&payload).unwrap();

        let expected = "{}";
        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_all() {
        let common_payload = CommonMessagePayload::new().text("text");
        let payload = InteractiveRespondPayload::new()
            .payload(common_payload)
            .response_type(InChannel)
            .replace_original(false)
            .delete_original(true);
        let json = serde_json::to_string_pretty(&payload).unwrap();

        let expected = r#"{
  "text": "text",
  "response_type": "in_channel",
  "replace_original": false,
  "delete_original": true
}"#;
        assert_eq!(json, expected);
    }
}
