use dioxus::prelude::*;

const STYLES_CSS: Asset = asset!("/assets/styles.css");
const OUTPUT_CSS: Asset = asset!("/assets/output.css");

#[component]
pub fn Posting() -> Element {
    let mut response = use_signal(|| "foo".to_string());

    rsx! {
        document::Link { rel: "stylesheet", href: STYLES_CSS }
        document::Link { rel: "stylesheet", href: OUTPUT_CSS }

        div {
            class: "box-gradient mt-6 p-6 rounded-lg shadow",
            onclick: move |_| async move {
                let data = api::res().await.unwrap();
                response.set(data);
            },
            {response}
        }
    }
}