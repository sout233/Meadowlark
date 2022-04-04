use vizia::*;

use crate::ui::icons::IconCode;

pub struct Icon {}

impl Icon {
    // Creates an Icon with a set size for the outer frame and the icon.
    pub fn new<'a>(cx: &'a mut Context, icon: IconCode, frame_size: f32, icon_size: f32) -> Handle<'a, Self> {
        Self {}.build2(cx, |cx| {
            let icon_str: &str = icon.into();

            // Icon can't be bigger than the frame it's held in.
            if icon_size > frame_size {
                icon_size = frame_size;
            }

            Label::new(cx, icon_str)
                .width(Pixels(frame_size))
                .height(Pixels(frame_size))
                .font_size(icon_size)
                .font("meadowlark")
                .class("icon");
        })
    }

    // Creates an Icon with a frame of `24px` and an icon of `16px`.
    pub fn default<'a>(cx: &'a mut Context, icon: IconCode) -> Handle<'a, Self> {
        Self::new(cx, icon, 24.0, 16.0)
    }
}

impl View for Icon {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {}
}
