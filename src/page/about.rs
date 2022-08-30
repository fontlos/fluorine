use crate::components::Head;
use crate::components;
use dioxus::prelude::*;

pub fn about(cx: Scope) -> Element{
    rsx! {cx,
        Head{
            link{
                rel: "Shortcut Icon",
                href: "https://fangtaluosi.top/img/icon/logo.png"
            }
            title{"芳塔洛斯"}
        }
        link{
            rel: "stylesheet",
            href: "./css/base.css"
        }
        link{
            rel: "stylesheet",
            href: "./css/index.css"
        }
        components::header{}
        components::nav{}
        main{
            "about"
        }
        components::footer{}
    }
}