use serde::Deserialize;

/// Received when a user interacts with a Block Kit interactive component.
///
/// Reference:
/// https://api.slack.com/reference/interaction-payloads/block-actions
#[derive(Debug, Deserialize)]
pub struct BlockActionsPayload {
    #[serde(rename = "type")]
    pub type_name: String,
    pub trigger_id: String,
    pub user: User,
    pub response_url: String,
    pub actions: Vec<Actions>,
}

#[derive(Debug, Deserialize)]
pub struct Actions {
    #[serde(rename = "type")]
    pub type_name: String,
    pub action_id: String,
    pub block_id: String,
    pub text: Option<Text>,
    pub value: Option<String>,
    pub selected_option: Option<SelectedOption>,
    pub selected_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Text {
    #[serde(rename = "type")]
    pub type_name: String,
    pub text: String,
    pub emoji: bool,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub team_id: String,
}

#[derive(Debug, Deserialize)]
pub struct SelectedOption {
    pub text: Text,
    pub value: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_de_payload() {
        let json = r#"{
  "type": "block_actions",
  "trigger_id": "trigger_id",
  "user": {
    "id": "user_id",
    "username": "name",
    "team_id": "team_id"
  },
  "response_url": "https://example.com",
  "actions": [
    {
      "action_id": "action_id",
      "block_id": "block_id",
      "text": {
        "type": "plain_text",
        "text": "text",
        "emoji": true
      },
      "value": "click",
      "type": "button",
      "selected_option": {
        "text": {
          "type": "plain_text",
          "text": "selected_text",
          "emoji": true
        },
        "value": "selected_value"
      }
    }
  ]
}"#;
        let payload = serde_json::from_str::<BlockActionsPayload>(json).unwrap();
        assert_eq!(payload.type_name, "block_actions");
        assert_eq!(payload.trigger_id, "trigger_id");

        assert_eq!(payload.user.id, "user_id");
        assert_eq!(payload.user.username, "name");
        assert_eq!(payload.user.team_id, "team_id");

        assert_eq!(payload.response_url, "https://example.com");

        assert_eq!(payload.actions[0].action_id, "action_id");
        assert_eq!(payload.actions[0].block_id, "block_id");
        assert_eq!(payload.actions[0].value, Some("click".to_string()));
        assert_eq!(payload.actions[0].type_name, "button");

        assert_eq!(
            payload.actions[0].selected_option.as_ref().unwrap().value,
            "selected_value"
        );
    }
}
