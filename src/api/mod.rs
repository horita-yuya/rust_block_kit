pub mod chat;
pub mod view;

macro_rules! base_url {
    () => {
        "https://slack.com/api"
    };
}

pub const CHAT_POST_MESSAGE_URL: &str = concat!(base_url!(), "/chat.postMessage");

pub const VIEWS_OPEN_URL: &str = concat!(base_url!(), "/views.open");
