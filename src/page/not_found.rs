use crate::components::Head;
use dioxus::prelude::*;
pub fn not_found(cx: Scope) -> Element {
    let piece = ["one", "two", "three"]
        .iter().map(|n|rsx!(
            div{
                class: "{n}",
                div{
                    class: "content",
                    span{class: "piece"}
                    span{class: "piece"}
                    span{class: "piece"}
                }
            }
        ));
    rsx! {cx,
        Head{
            link{
                rel: "Shortcut Icon",
                href: "https://fangtaluosi.top/img/icon/logo.png"
            }
            title{"404 Not Found | 芳塔洛斯"}
        }
        link{
            rel: "stylesheet",
            href: "./css/404.css"
        }
        main{
            class: "wrapper",
            div{
                class: "container",
                div{
                    class: "scene",
                    div{class: "circle"}
                    piece,
                    p{class: "p404","404"}
                    p{class: "p404","404"}
                }
                div{
                    class: "text",
                    article{
                        p{"Opps! Page Not Found"}
                        Link{
                            to: "/",
                            "Return Index"
                        }
                    }
                }
            }
        }
    }
}
