use gpui::{InteractiveElement, Interactivity, Stateful, StatefulInteractiveElement};
use std::any::Any;
use std::sync::atomic::AtomicBool;

use gpui::prelude::*;
use gpui::*;

#[derive(IntoElement)]
pub struct Checkbox {
    id: Option<ElementId>,
    value: bool,
    is_mouse_button_down: bool,
    on_click: Option<Box<dyn Fn(&bool, &mut WindowContext) + 'static>>,
    interactivity: Interactivity,
}

impl InteractiveElement for Checkbox {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}
impl StatefulInteractiveElement for Checkbox {}

impl Checkbox {
    pub fn on_click(mut self, click_handler: impl Fn(&bool, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(click_handler));
        self
    }

    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn value(mut self, value: bool) -> Self {
        self.value = value;
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id(self.id.unwrap())
            .w_10()
            .h_10()
            .when_else(self.value, |s| s.bg(rgb(0x00ff00)), |s| s.bg(rgb(0x0000ff)))
            .when_some(self.on_click, |this, on_click| {
                this.on_click(move |_, cx| on_click(&!self.value, cx))
            })
            .on_any_mouse_down(|a, b| {
                println!("Mouse down on checkbox");
            })
    }
}

pub fn checkbox() -> Checkbox {
    let mut this = Checkbox {
        id: None,
        value: false,
        is_mouse_button_down: false,
        on_click: None,
        interactivity: Default::default(),
    };

    this.interactivity.on_mouse_down(MouseButton::Left, |a, b| {
        println!("Mouse down on checkbox");
    });

    this
}
