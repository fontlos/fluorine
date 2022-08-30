use dioxus::prelude::*;

pub fn search<'a>(cx:Scope)->Element{
    rsx!(cx,
        div{
            class:"search-wrap",
            div{
                class:"searchBox",
                div{class:"shadow"}
                input{
                    r#type:"text",
                    placeholder:"Search for...",
                }
                span{class:"iconfont icon-search"}
            }
        }
    )
}