#[derive(Copy, Clone)]
pub enum SlackBlockType {
    Input,
    Actions,
    Section,
    Divider,
}

impl SlackBlockType {
    pub fn to_s (&self) -> String {
        match self {
            SlackBlockType::Input => String::from("input"),
            SlackBlockType::Actions => String::from("actions"),
            SlackBlockType::Section => String::from("section"),
            SlackBlockType::Divider => String::from("divider"),
        }
    }
}