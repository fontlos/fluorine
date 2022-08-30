use crate::components::Head;
use crate::components;
use dioxus::prelude::*;

pub fn blog(cx: Scope) -> Element{
    rsx! {cx,
        Head{
            link{
                rel: "Shortcut Icon",
                href: "https://fangtaluosi.top/img/icon/logo.png"
            }
            title{"Blog|芳塔洛斯"}
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
            "blog"
        }
        components::footer{}
        //components::loading1{}
        div{id:"snow"}
    }
}

pub fn blog_page(cx: Scope) -> Element {
    let  id =match use_route(&cx).segment("id"){
        Some(id)=>id,
        None=>"",
    };

    rsx!{
        cx,
        "{id}"
    }
}