use crate::composition::option::OptionObject;
use crate::composition::text::Text::Plain;
use crate::composition::text::{PlainText, Text};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OptionGroup {
    label: Text,
    options: Vec<OptionObject>,
}

impl OptionGroup {
    pub fn new(label: impl Into<PlainText>, options: Vec<OptionObject>) -> Self {
        OptionGroup {
            label: Plain(label.into()),
            options,
        }
    }
}
