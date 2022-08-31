use dioxus::prelude::*;

pub fn header(cx: Scope) -> Element {
    rsx! {cx,
        header{
            link{
                rel: "stylesheet",
                href: "https://fangtaluosi.top/css/page/search.css"
            }
            super::search{}
        }
    }
}