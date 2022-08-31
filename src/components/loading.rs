use dioxus::prelude::*;

#[allow(dead_code)]
pub fn loading1(cx: Scope) -> Element {
    let loading = (1..=11).map(|num|{
        rsx! {cx,
            div{
                key:"{num}",
                class:"line line{num}",
                span{class:"circle circle-top"}
                div{
                    class:"dotted",
                    span{class:"dot dot-top"}
                    span{class:"dot dot-middle-top"}
                    span{class:"dot dot-middle-bottom"}
                    span{class:"dot dot-bottom"}
                }
                span{class:"circle circle-bottom"}
            }
        }
    });
    rsx! {cx,
        link{
            rel: "stylesheet",
            href: "https://fangtaluosi.top/css/components/loading-1.css"
        }
        div{
            class:"loading",
            div{class:"door"}
            div{
                class:"wrapper",
                loading
            }
        }
    }
}