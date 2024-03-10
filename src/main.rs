mod components;

use components::checkbox::{checkbox, Checkbox};
use gpui::*;

struct HelloWorld {
    text: SharedString,
    checkbox_value: bool,
}

impl Render for HelloWorld {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(
                checkbox()
                    .id(1)
                    .value(self.checkbox_value)
                    .on_click(cx.listener(move |this, checkbox_value, cx| {
                        println!("Checkbox clicked: {}", checkbox_value);
                        this.checkbox_value = *checkbox_value;
                    })),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        // Displays
        let displays = cx.displays();

        let mut window_options = WindowOptions::default();
        window_options.display_id = Some(displays[1].id());

        cx.open_window(window_options, |cx| {
            cx.new_view(|_cx| HelloWorld {
                text: "World".into(),
                checkbox_value: false,
            })
        });
    });
}
