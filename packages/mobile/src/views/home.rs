use dioxus::prelude::*;
use ui::Title;

#[component]
pub fn Home() -> Element {
    rsx! {
        Title {}
    }
}