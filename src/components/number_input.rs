use gpui::*;

pub struct NumberInput {
    pub value: i32,
    pub min: i32,
    pub max: i32,
    pub step: i32,
}

pub fn number_input() -> NumberInput {
    NumberInput {
        value: 0,
        min: 0,
        max: 100,
        step: 1,
    }
}
