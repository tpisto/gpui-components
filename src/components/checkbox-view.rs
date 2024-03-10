use std::sync::atomic::AtomicBool;

use gpui::prelude::*;
use gpui::*;

// #[derive(IntoElement)]
pub struct Checkbox {
    id: Option<ElementId>,
    value: bool,
    is_mouse_button_down: bool,
    on_click: Option<Box<dyn Fn(&bool, &mut WindowContext) + 'static>>,
}

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

impl Element for Checkbox {
    fn into_any(self) -> AnyElement {
        AnyElement::Checkbox(self)
    }

    type State;

    fn request_layout(
        &mut self,
        state: Option<Self::State>,
        cx: &mut ElementContext,
    ) -> (LayoutId, Self::State) {
        todo!()
    }

    fn paint(&mut self, bounds: Bounds<Pixels>, state: &mut Self::State, cx: &mut ElementContext) {
        todo!()
    }
}

impl IntoElement for Checkbox {
    type Element = Self;

    fn element_id(&self) -> Option<ElementId> {
        todo!()
    }

    fn into_element(self) -> Self::Element {
        todo!()
    }

    fn into_any_element(self) -> AnyElement {
        self.into_element().into_any()
    }
}

impl Render for Checkbox {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id(self.id.clone().unwrap())
            .w_10()
            .h_10()
            .when(self.is_mouse_button_down, |this| this.bg(rgb(0x808080)))
            .when_else(self.value, |s| s.bg(rgb(0x00ff00)), |s| s.bg(rgb(0x0000ff)))
            .when_some(self.on_click.as_mut(), |this, on_click| {
                on_click(&!self.value, cx);
                this
            })
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|a, b, c| {
                    // ABC
                    a.is_mouse_button_down = true;
                }),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|a, b, c| {
                    // ABC
                    a.is_mouse_button_down = false;
                }),
            )

        // .when(true, |s| {
        //     s.on_mouse_down(MouseButton::Left, |b, c| {
        //         println!("");
        //         s.absolute();
        //     })
        // })

        // .on_click(cx.listener(move |this, select, cx| {
        //     if let Some(on_click) = this.on_click.as_mut() {
        //         on_click(&!this.value, cx);
        //     }
        //     // this.on_click.as_mut().unwrap()
        // }))
    }
}

// impl RenderOnce for Checkbox {
//     fn render(self, cx: &mut WindowContext) -> impl IntoElement {
//         div()
//             .id(self.id.unwrap())
//             .w_10()
//             .h_10()
//             .when_else(self.value, |s| s.bg(rgb(0x00ff00)), |s| s.bg(rgb(0x0000ff)))
//             .when_some(self.on_click, |this, on_click| {
//                 this.on_click(move |_, cx| on_click(&!self.value, cx))
//             })
//             .when(true, |s| {
//                 s.on_mouse_down(MouseButton::Left, |b, c| {
//                     println!("");
//                     s.absolute();
//                 })
//             })
//             .on_any_mouse_down(|a, b| {
//                 println!("Mouse down on checkbox");
//             })
//     }
// }

pub fn checkbox() -> impl IntoElement {
    Checkbox {
        id: None,
        value: false,
        on_click: None,
        is_mouse_button_down: true,
    }
}
