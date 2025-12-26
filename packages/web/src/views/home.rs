use dioxus::prelude::*;
use ui::{Title, Posting};

#[component]
pub fn Home() -> Element {
    rsx! {
       div {
            class: "max-w-4xl mx-auto px-4 sm:px-6 lg:px-8",
            Title {}
            Posting {}
       }
    }
}
