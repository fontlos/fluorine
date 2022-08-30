use crate::components::Head;
use crate::components;
use dioxus::prelude::*;

pub fn resources(cx: Scope) -> Element{
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
            "resources"
        }
        components::footer{}
        //components::loading1{}
        div{id:"snow"}
    }
}

fn resources_page(cx: Scope) -> Element{
    let res =match use_route(&cx).segment("res"){
        Some(res)=>res,
        None=>"",
    };

    let title =match use_route(&cx).segment("res"){
        Some(title)=>title,
        None=>"",
    };

    rsx!{
        cx,
        "res:{res},title:{title}",
    }
}