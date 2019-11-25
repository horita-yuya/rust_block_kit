use crate::block_element::button::ButtonElement;
use crate::block_element::date_picker::DatePickerElement;
use crate::block_element::image::ImageElement;
use crate::block_element::multi_select_menu::MultiStaticSelectMenuElement;
use crate::block_element::overflow_menu::OverflowMenuElement;
use crate::block_element::plain_text_input::PlainTextInputElement;
use crate::block_element::select_menu::StaticSelectMenuElement;
use serde::Serialize;

pub mod button;
pub mod date_picker;
pub mod image;
pub mod multi_select_menu;
pub mod overflow_menu;
pub mod plain_text_input;
pub mod select_menu;

pub(self) const BUTTON_TYPE: &str = "button";
pub(self) const DATE_PICKER_TYPE: &str = "datepicker";
pub(self) const IMAGE_TYPE: &str = "image";
pub(self) const OVERFLOW_MENU_TYPE: &str = "overflow";
pub(self) const PLAIN_TEXT_INPUT_TYPE: &str = "plain_text_input";
pub(self) const STATIC_SELECT_MENU_TYPE: &str = "static_select";
pub(self) const MULTI_STATIC_SELECT_MENU_TYPE: &str = "multi_static_select";

/// ButtonElement
/// OverflowMenuElement
/// PlainTextInputElement
/// StaticSelectMenuElement
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum BlockElement {
    Button(ButtonElement),
    OverflowMenu(OverflowMenuElement),
    PlainTextInput(PlainTextInputElement),
    StaticSelectMenu(StaticSelectMenuElement),
    MultiStaticSelectMenu(MultiStaticSelectMenuElement),
    Image(ImageElement),
    DatePicker(DatePickerElement),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::block_element::overflow_menu::OverflowMenuOption::TwoOptions;
    use crate::block_element::BlockElement::{Button, OverflowMenu};
    use crate::composition::option::OptionObject;
    use crate::composition::text::PlainText;

    #[test]
    fn test_ser_button_element() {
        let button = ButtonElement::new(PlainText::new("text"), "action_id");
        let element = Button(button);

        let json = serde_json::to_string_pretty(&element).unwrap_or("".to_string());
        let expected = r#"{
  "type": "button",
  "text": {
    "type": "plain_text",
    "text": "text"
  },
  "action_id": "action_id"
}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn test_ser_overflow_element() {
        let option1 = OptionObject::new(PlainText::new("text1"), "value1");
        let option2 = OptionObject::new(PlainText::new("text2"), "value2");

        let overflow_menu = OverflowMenuElement::new("action", TwoOptions([option1, option2]));
        let element = OverflowMenu(overflow_menu);

        let json = serde_json::to_string_pretty(&element).unwrap_or("".to_string());
        let expected = r#"{
  "type": "overflow",
  "action_id": "action",
  "options": [
    {
      "text": {
        "type": "plain_text",
        "text": "text1"
      },
      "value": "value1"
    },
    {
      "text": {
        "type": "plain_text",
        "text": "text2"
      },
      "value": "value2"
    }
  ]
}"#;

        assert_eq!(json, expected);
    }
}
