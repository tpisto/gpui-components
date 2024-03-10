use gpui::*;

pub struct TextInput {
    pub value: String,
    pub placeholder: String,
}

pub fn text_input() -> TextInput {
    TextInput {
        value: "".to_string(),
        placeholder: "".to_string(),
    }
}
