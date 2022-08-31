use dioxus::prelude::*;

pub fn footer(cx: Scope) -> Element {
    rsx! {cx,
        link{
            rel: "stylesheet",
            href: "https://fangtaluosi.top/css/components/footer.css"
        }
        script{src:"https://fangtaluosi.top/js/fish.js"}
        footer{
            p{
                "©芳塔洛斯 Powered by ",
                a{
                    href:"https://dioxuslabs.com",
                    "Dioxus"
                }
            }
            div{class:"flying-fish",id:"flying-fish"}
        }
    }
}