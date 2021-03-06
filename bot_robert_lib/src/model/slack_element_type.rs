#[derive(Copy, Clone, Debug)]
pub enum SlackElementType {
    NoType,
    PlaneText,
    PlaneTextInput,
    Button,
    Checkboxes
}

impl ToString for SlackElementType {
    fn to_string(&self) -> String {
        match self {
            SlackElementType::NoType => "".to_string(),
            SlackElementType::PlaneText => String::from("plain_text"),
            SlackElementType::PlaneTextInput => String::from("plain_text_input"),
            SlackElementType::Button => String::from("button"),
            SlackElementType::Checkboxes => "checkboxes".to_string()
        }
    }
}

impl From<&String> for SlackElementType {
    fn from(str: &String) -> Self {
        return if *str == SlackElementType::PlaneText.to_string() {
            SlackElementType::PlaneText
        }
        else if *str == SlackElementType::PlaneTextInput.to_string() {
            SlackElementType::PlaneTextInput
        }
        else if *str == SlackElementType::Button.to_string() {
            SlackElementType::Button
        }
        else if *str == SlackElementType::Checkboxes.to_string() {
            SlackElementType::Checkboxes
        }
        else {
            SlackElementType::PlaneText
        }
    }
}