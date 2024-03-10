use gpui::*;

pub struct Select {
    pub value: String,
    pub options: Vec<(String, String)>,
}

pub fn select() -> Select {
    Select {
        value: "".to_string(),
        options: vec![],
    }
}
