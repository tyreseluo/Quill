
#[cfg(feature = "builtin-makepad-widgets")]
pub extern crate makepad_widgets;
#[cfg(feature = "Button")]
pub mod button;
pub mod styles;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    makepad_widgets::live_design(cx);
    styles::live_design(cx);
    #[cfg(feature = "Button")]
    button::live_design(cx);
}
