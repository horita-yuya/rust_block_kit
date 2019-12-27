use crate::block::Block;
use crate::composition::text::{Text, PlainText};
use crate::payload::view::ViewPayload::{HomeTab, Modal};
use serde::Serialize;
use crate::composition::text::Text::Plain;

/// https://api.slack.com/methods/views.open
#[derive(Debug, Serialize)]
pub struct ViewsOpenPayload {
    trigger_id: String,
    view: ViewPayload,
}

const MODAL_TYPE: &str = "modal";
const HOME_TAB_TYPE: &str = "home";

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ViewPayload {
    Modal(ModalPayload),
    HomeTab(HomeTabPayload),
}

/// https://api.slack.com/reference/surfaces/views
#[derive(Debug, Serialize)]
pub struct ModalPayload {
    #[serde(rename = "type")]
    view_type: String,
    title: Text,
    blocks: Vec<Block>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close: Option<Text>,

    /// Default optional
    /// Required: when input block is within the blocks.
    #[serde(skip_serializing_if = "Option::is_none")]
    submit: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    private_metadata: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    callback_id: Option<String>,

    /// Default false.
    #[serde(skip_serializing_if = "Option::is_none")]
    clear_on_close: Option<bool>,

    /// Default false.
    #[serde(skip_serializing_if = "Option::is_none")]
    notify_on_close: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<String>,
}

/// https://api.slack.com/reference/surfaces/views
#[derive(Debug, Serialize)]
pub struct HomeTabPayload {
    #[serde(rename = "type")]
    view_type: String,
    blocks: Vec<Block>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_metadata: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    callback_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    external_id: Option<String>,
}

impl ViewsOpenPayload {
    pub fn new(trigger_id: impl Into<String>, view_payload: impl Into<ViewPayload>) -> Self {
        ViewsOpenPayload {
            trigger_id: trigger_id.into(),
            view: view_payload.into(),
        }
    }
}

impl ModalPayload {
    pub fn new(title: impl Into<PlainText>, blocks: Vec<Block>) -> Self {
        ModalPayload {
            view_type: MODAL_TYPE.to_string(),
            title: Plain(title.into()),
            blocks,
            close: Option::default(),
            submit: Option::default(),
            private_metadata: Option::default(),
            callback_id: Option::default(),
            clear_on_close: Option::default(),
            notify_on_close: Option::default(),
            external_id: Option::default(),
        }
    }

    pub fn submit(mut self, text: impl Into<PlainText>) -> Self {
        self.submit = Some(Plain(text.into()));
        self
    }

    pub fn close(mut self, text: impl Into<PlainText>) -> Self {
        self.close = Some(Plain(text.into()));
        self
    }
}

impl From<ModalPayload> for ViewPayload {
    fn from(payload: ModalPayload) -> Self {
        Modal(payload)
    }
}

impl HomeTabPayload {
    pub fn new(blocks: Vec<Block>) -> Self {
        HomeTabPayload {
            view_type: HOME_TAB_TYPE.to_string(),
            blocks,
            private_metadata: Option::default(),
            callback_id: Option::default(),
            external_id: Option::default(),
        }
    }
}

impl From<HomeTabPayload> for ViewPayload {
    fn from(payload: HomeTabPayload) -> Self {
        HomeTab(payload)
    }
}
