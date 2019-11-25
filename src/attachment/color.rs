const GOOD_COLOR: &str = "good";
const WARNING_COLOR: &str = "warning";
const DANGER_COLOR: &str = "danger";

#[derive(Debug)]
pub enum Color {
    Hex(String),
    Good,
    Warning,
    Danger,
}

impl Color {
    pub(crate) fn value(&self) -> String {
        match self {
            Color::Hex(value) => value.to_string(),
            Color::Good => GOOD_COLOR.to_string(),
            Color::Warning => WARNING_COLOR.to_string(),
            Color::Danger => DANGER_COLOR.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_serializing_new() {
        let color = Color::Hex("#ffffff".to_string());
        assert_eq!(color.value(), "#ffffff");

        let color = Color::Good;
        assert_eq!(color.value(), "good");

        let color = Color::Warning;
        assert_eq!(color.value(), "warning");

        let color = Color::Danger;
        assert_eq!(color.value(), "danger");
    }
}
