use dioxus::prelude::*;

const STYLES_CSS: Asset = asset!("/assets/styles.css");
const OUTPUT_CSS: Asset = asset!("/assets/output.css");

#[component]
pub fn Title() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STYLES_CSS}
        document::Link { rel: "stylesheet", href: OUTPUT_CSS }

        h1 {
            class: "box-gradient mt-6 p-6 rounded-lg shadow text-2xl text-center",
            "Willkommen bei Dompfaff!"
        }
    }
}