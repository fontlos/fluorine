use crate::components::Head;
use crate::components;
use dioxus::prelude::*;

pub fn home(cx: Scope) -> Element{
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
        script{src:"https://cdn.jsdelivr.net/npm/jquery@3.5.1/dist/jquery.min.js"}
        script{src:"https://cdn.jsdelivr.net/npm/particles.js@2.0.0/particles.min.js"}
        script{src:"https://fangtaluosi.top/js/snow.js"}
        components::header{}
        components::nav{}
        main{
            Cards{}
        }
        components::footer{}
        //components::loading1{}
        div{id:"snow"}
    }
}

#[allow(non_snake_case)]
fn Cards(cx: Scope) -> Element {
    rsx! {
        cx,
        link{
            rel: "stylesheet",
            href: "./css/page/card.css"
        }
        div{
            class:"cards",
            components::card{
                title:"Card one",
                subtitle:"something",
                link:""
            }
            components::card{
                title:"Card two",
                subtitle:"something",
                link:""
            }
            components::card{
                title:"Card three",
                subtitle:"something",
                link:""
            }
            components::card{
                title:"Card four",
                subtitle:"something",
                link:""
            }
        }
    }
}