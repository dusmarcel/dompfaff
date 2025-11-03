use dioxus::prelude::*;
//use ui::{Echo, Hero};

#[component]
pub fn Home() -> Element {
    rsx! {
       div {
            class: "bg-pink-500",
            "Hello, dompfaff!"
       }
    }
}
