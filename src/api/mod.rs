pub mod chat;

macro_rules! base_url {
    () => {
        "https://slack.com/api"
    };
}

pub const CHAT_POST_MESSAGE_URL: &str = concat!(base_url!(), "/chat.postMessage");
