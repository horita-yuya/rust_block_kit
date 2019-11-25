use actix_web::client;
use actix_web::Error;
use block_kit::api::chat::PostMessageResponse;
use block_kit::api::CHAT_POST_MESSAGE_URL;
use block_kit::block::actions::ActionsBlock;
use block_kit::block::context::ContextBlock;
use block_kit::block::context::ContextBlockElement::{ImageContext, TextContext};
use block_kit::block::section::SectionBlock;
use block_kit::block_element::button::ButtonElement;
use block_kit::block_element::button::Style::{Danger, Primary};
use block_kit::block_element::image::ImageElement;
use block_kit::block_element::select_menu::StaticSelectMenuElement;
use block_kit::block_element::BlockElement::{Button, StaticSelectMenu};
use block_kit::composition::confirmation_dialog::ConfirmationDialog;
use block_kit::composition::option::OptionObject;
use block_kit::composition::text::Text::Markdown;
use block_kit::config::SlackConfig;
use block_kit::payload::chat::ChatPostMessagePayload;
use block_kit::payload::interaction::InteractiveRespondPayload;
use block_kit::payload::interaction::ResponseType::InChannel;
use block_kit::payload::message::CommonMessagePayload;
use serde::Serialize;

pub async fn send_static_select(config: &SlackConfig) -> Result<(), Error> {
    let section1 = SectionBlock::new(Markdown("*Hello World*".into()));
    let section2 = SectionBlock::new(Markdown("What do you want to do?".into())).accessory(
        StaticSelectMenu(StaticSelectMenuElement::new("Tasks", "tasks").options(vec![
            OptionObject::new("Run test", "test"),
            OptionObject::new("Deploy", "deploy"),
        ])),
    );

    let payload = ChatPostMessagePayload::new(
        &config.channel,
        CommonMessagePayload::new().blocks(vec![section1.into(), section2.into()]),
    );
    send(&payload, &config, None).await.unwrap_or(());
    Ok(())
}

pub async fn send_actions(config: &SlackConfig) -> Result<(), Error> {
    let actions = ActionsBlock::new(vec![
        Button(ButtonElement::new("Start", "start").style(Primary).confirm(
            ConfirmationDialog::new("Deploy", Markdown("*ok?*".into()), "Confirm", "Stop"),
        )),
        Button(ButtonElement::new("Stop", "stop").style(Danger)),
    ]);

    let payload = ChatPostMessagePayload::new(
        &config.channel,
        CommonMessagePayload::new().blocks(vec![actions.into()]),
    );
    send(&payload, &config, None).await.unwrap_or(());

    Ok(())
}

pub async fn send_complete(config: &SlackConfig, response_url: &str) -> Result<(), Error> {
    let section = SectionBlock::new(Markdown("Completed :tada:".into()));
    let context = ContextBlock::new(vec![
        TextContext(Markdown("_see you soon_".into())),
        ImageContext(ImageElement::new("https://raw.githubusercontent.com/rochacbruno/rust_memes/master/img/ferris_hand_up.jpg", "hoge")),
        ImageContext(ImageElement::new("https://raw.githubusercontent.com/rochacbruno/rust_memes/master/img/ferris_hand_up.jpg", "hoge")),
        ImageContext(ImageElement::new("https://raw.githubusercontent.com/rochacbruno/rust_memes/master/img/ferris_hand_up.jpg", "hoge"))
    ]);

    let blocks = vec![section.into(), context.into()];
    let payload = InteractiveRespondPayload::new()
        .payload(CommonMessagePayload::new().blocks(blocks))
        .replace_original(false)
        .response_type(InChannel);

    send(&payload, &config, Some(response_url))
        .await
        .unwrap_or(());
    Ok(())
}

async fn send<T: Serialize>(
    payload: &T,
    config: &SlackConfig,
    url: Option<&str>,
) -> Result<(), Error> {
    let url = url.unwrap_or(CHAT_POST_MESSAGE_URL);
    info!("{}", url);
    let _ = client::Client::default()
        .post(url)
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
