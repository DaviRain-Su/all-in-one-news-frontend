#![allow(non_snake_case)]
use dioxus::prelude::*;

pub mod rebase;
pub mod rustcc;
pub mod types;

pub static REBASE_BASE__API_URL: &str = "https://aion-qr8nz.ondigitalocean.app";
pub static LOCAL_REBASE_BASE__API_URL: &str = "http://localhost:8000";

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("sup");
    // launch the web app
    dioxus_web::launch(App);
}

pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || rebase::PreviewState::Unset);

    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            width: "100%",
            div {
                width: "100%",
                rebase::Aions {},
            }
            // div {
            //     width: "50%",
            //     rustcc::Aions {}
            // }
        }
    })
}
