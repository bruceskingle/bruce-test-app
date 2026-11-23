//! Test App, a desktop app made with Dioxus.

use dioxus::prelude::*;

/// The app's styles, bundled by dx.
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

/// The app's one window.
#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        main {
            h1 { "Test App" }
            p { {bruce_test_app_core::greeting()} }
        }
    }
}
