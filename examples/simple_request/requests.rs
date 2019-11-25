// For reqwest ex
//use reqwest::Error;
use actix_web::client;
use actix_web::Error;
use block_kit::api::chat::PostMessageResponse;
use block_kit::api::CHAT_POST_MESSAGE_URL;
use block_kit::block::actions::ActionsBlock;
use block_kit::block::context::ContextBlock;
use block_kit::block::context::ContextBlockElement::{ImageContext, TextContext};
use block_kit::block::divider::DividerBlock;
use block_kit::block::section::SectionBlock;
use block_kit::block::Block;
use block_kit::block::Block::{Actions, Context, Divider, Section};
use block_kit::block_element::button::ButtonElement;
use block_kit::block_element::image::ImageElement;
use block_kit::block_element::multi_select_menu::MultiStaticSelectMenuElement;
use block_kit::block_element::select_menu::StaticSelectMenuElement;
use block_kit::block_element::BlockElement::{Button, MultiStaticSelectMenu, StaticSelectMenu};
use block_kit::composition::confirmation_dialog::ConfirmationDialog;
use block_kit::composition::option::OptionObject;
use block_kit::composition::option_group::OptionGroup;
use block_kit::composition::text::Text::{Markdown, Plain};
use block_kit::config::SlackConfig;
use block_kit::payload::chat::ChatPostMessagePayload;
use block_kit::payload::message::CommonMessagePayload;

pub async fn send_hello_world(config: &SlackConfig) -> Result<(), Error> {
    let blocks = get_blocks();
    let common_payload = CommonMessagePayload::new().blocks(blocks);
    let payload = ChatPostMessagePayload::new(&config.channel, common_payload)
        .icon_emoji(":computer:")
        .username("BOT");

    // Ex: reqwest
    //    let res: PostMessageResponse = reqwest::Client::new()
    //        .post(CHAT_POST_MESSAGE_URL)
    //        .header("Authorization", format!("Bearer {}", config.bot_token))
    //        .json(&payload)
    //        .send()
    //        .await
    //        .unwrap()
    //        .json()
    //        .await
    //        .unwrap();
    //
    //    info!("{:?}", res);

    let _ = client::Client::default()
        .post(CHAT_POST_MESSAGE_URL)
        .header("Authorization", format!("Bearer {}", config.bot_token))
        .send_json(&payload)
        .await
        .map_err(|err| {
            error!("{:?}", err);
            format!("error: {:?}", err)
        })
        .unwrap()
        .json::<PostMessageResponse>()
        .await
        .and_then(|response| {
            info!("{:?}", response);
            Ok(())
        });
    Ok(())
}

fn get_blocks() -> Vec<Block> {
    let dialog = ConfirmationDialog::new(
        "Confirmation Dialog",
        Plain("Hello".into()),
        "confirm",
        "deny",
    );

    let multi_select_menu = MultiStaticSelectMenuElement::new("test", "ac")
        .option_groups(vec![
            OptionGroup::new(
                "text",
                vec![OptionObject::new("t1", "v1"), OptionObject::new("t2", "v2")],
            ),
            OptionGroup::new(
                "text",
                vec![OptionObject::new("t3", "v3"), OptionObject::new("t4", "v4")],
            ),
        ])
        .initial_options(vec![
            OptionObject::new("t1", "v1"),
            OptionObject::new("t2", "v2"),
        ])
        .confirm(dialog.clone());

    let section1 = SectionBlock::new(Markdown("@here *Hello World*, `section1`".into()));

    let divider = DividerBlock::new();

    let section2 = SectionBlock::new(Markdown("Select task `section2`".into()))
        .accessory(MultiStaticSelectMenu(multi_select_menu));

    let context = ContextBlock::new(vec![
        TextContext(Markdown("*markdown* `context`".into())),
        ImageContext(ImageElement::new(
            "https://raw.githubusercontent.com/rochacbruno/rust_memes/master/img/ferris_hand_up.jpg",
            "text"
        ))
    ]);

    let actions = ActionsBlock::new(vec![
        Button(ButtonElement::new("`actions` Press", "button_action").confirm(dialog.clone())),
        StaticSelectMenu(
            StaticSelectMenuElement::new("text", "select_menu_action")
                .options(vec![
                    OptionObject::new("text1", "val1"),
                    OptionObject::new("text2", "val2"),
                ])
                .confirm(dialog.clone()),
        ),
    ]);

    vec![
        Section(section1),
        Divider(divider),
        Section(section2),
        Context(context),
        Actions(actions),
    ]
}
