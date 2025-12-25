use dioxus::prelude::*;
//use ui::{Echo, Hero};

#[component]
pub fn Home() -> Element {
    rsx! {
       div {
            class: "max-w-4xl mx-auto px-4 sm:px-6 lg:px-8",
           div {
            class: "box-gradient p-6 rounded-lg shadow",
            "Hello, dompfaff!"
           }
       }
    }
}
