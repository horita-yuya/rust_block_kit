use crate::block::input::InputElement::{DatePicker, MultiSelectMenu, PlainTextInput, SelectMenu};
use crate::block::INPUT_TYPE;
use crate::block_element::date_picker::DatePickerElement;
use crate::block_element::multi_select_menu::MultiStaticSelectMenuElement;
use crate::block_element::plain_text_input::PlainTextInputElement;
use crate::block_element::select_menu::StaticSelectMenuElement;
use serde::Serialize;
use crate::composition::text::{Text, PlainText};
use crate::composition::text::Text::Plain;

/// https://api.slack.com/reference/block-kit/blocks#input
#[derive(Debug, Serialize)]
pub struct InputBlock {
    #[serde(rename = "type")]
    type_name: String,
    label: Text,
    element: InputElement,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum InputElement {
    PlainTextInput(PlainTextInputElement),
    SelectMenu(StaticSelectMenuElement),
    MultiSelectMenu(MultiStaticSelectMenuElement),
    DatePicker(DatePickerElement),
}

impl InputBlock {
    pub fn new(label: impl Into<PlainText>, element: impl Into<InputElement>) -> Self {
        InputBlock {
            type_name: INPUT_TYPE.to_string(),
            label: Plain(label.into()),
            element: element.into(),
        }
    }
}

impl From<PlainTextInputElement> for InputElement {
    fn from(element: PlainTextInputElement) -> Self {
        PlainTextInput(element)
    }
}

impl From<StaticSelectMenuElement> for InputElement {
    fn from(element: StaticSelectMenuElement) -> Self {
        SelectMenu(element)
    }
}

impl From<MultiStaticSelectMenuElement> for InputElement {
    fn from(element: MultiStaticSelectMenuElement) -> Self {
        MultiSelectMenu(element)
    }
}

impl From<DatePickerElement> for InputElement {
    fn from(element: DatePickerElement) -> Self {
        DatePicker(element)
    }
}
