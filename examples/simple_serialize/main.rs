#[macro_use]
extern crate log;

use block_kit::block::section::SectionBlock;
use block_kit::composition::text::Text::Markdown;
use block_kit::payload::chat::ChatPostMessagePayload;
use block_kit::payload::message::CommonMessagePayload;

fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let section = SectionBlock::new(Markdown("@here *Hello World*, `section1`".into()));
    let payload = ChatPostMessagePayload::new(
        "random",
        CommonMessagePayload::new().blocks(vec![section.into()]),
    );
    let json = serde_json::to_string_pretty(&payload).unwrap_or("".to_string());
    info!("{}", json);
}
