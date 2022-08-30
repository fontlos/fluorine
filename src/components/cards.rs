use dioxus::prelude::*;

#[derive(Props)]
struct CardProps<'a>{
    title:&'a str,
    subtitle:&'a str,
    link:&'a str,
    //tag:&'a str,
}

#[allow(non_snake_case)]
fn Card<'a>(cx:Scope<'a,CardProps<'a>>)->Element{
    rsx!(cx,
        div{
            class:"card",
            span{}
            div{
                class:"content",
                h2{"{cx.props.title}"}
                p{"{cx.props.subtitle}"}
                a{
                    class:"link",
                    href:"{cx.props.link}",
                    target:"_blank",
                    "Read More"
                }
            }
        }
    )
}