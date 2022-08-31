use dioxus::prelude::*;

pub fn nav(cx: Scope) -> Element {
    rsx! {cx,
        nav{
            link{
                rel: "stylesheet",
                href: "https://fangtaluosi.top/css/components/nav.css"
            }
            link{
                rel: "stylesheet",
                href: "https://fangtaluosi.top/css/icon/iconfont.css"
            }
            div{
                class:"nav",
                a{
                    class:"logo",
                    div{
                        class:"logo-icon",
                        img{
                            src:"https://fangtaluosi.top/img/icon/logo-bg.png",
                            alt:"logo"
                        }
                    }
                    div{
                        class:"logo-title",
                        p{"Good Day!"}
                        h2{"芳塔洛斯"}
                    }
                }
                div{class:"line"}
                div{
                    class:"title",
                    p{"Page"}
                }
                div{
                    class:"page",
                    NavButton{
                        left_icon:"icon-home_light",
                        lable:"Home",
                        right_icon:"icon-shezhi",
                        link:"/",
                    }
                    NavButton{
                        left_icon:"icon-resource",
                        lable:"Resource",
                        right_icon:"",
                        link:"/resources",
                    }
                    NavButton{
                        left_icon:"icon-ego-blog",
                        lable:"Blog",
                        right_icon:"",
                        link:"/blog",
                    }
                    NavButton{
                        left_icon:"icon-about",
                        lable:"About",
                        right_icon:"",
                        link:"/",
                    }
                    NavButton{
                        left_icon:"icon-tag",
                        lable:"Tag",
                        right_icon:"",
                        link:"/",
                    }
                }
                div{class:"line"}
                div{
                    class:"title",
                    p{"Serve"}
                }
                div{
                    class:"serve",
                    NavButton{
                        left_icon:"icon-link",
                        lable:"Contact",
                        right_icon:"",
                        link:"/",
                    }
                    Music{}
                    ToTop{}
                }
            }
        }
    }
}

#[derive(Props)]
struct NavButtonProps<'a>{
    left_icon:&'a str,
    lable:&'a str,
    right_icon:&'a str,
    link:&'a str
}

#[allow(non_snake_case)]
fn NavButton<'a>(cx:Scope<'a,NavButtonProps<'a>>)->Element{
    rsx!{cx,
        Link{
            to:"{cx.props.link}",
            class:"item",
            div{class:"light-line"}
            div{
                class:"left-icon",
                span{class:"iconfont {cx.props.left_icon}"}
            }
            div{
                class:"lable",
                "{cx.props.lable}"
            }
            div{
                class:"right-icon",
                span{class:"iconfont {cx.props.right_icon}"}
            }
        }
    }
}

#[allow(non_snake_case)]
fn Music<'a>(cx:Scope)->Element{
    rsx!{cx,
        div{
            class:"item",
            div{class:"light-line"}
            div{
                class:"left-icon",
                span{class:"iconfont icon-yinle"}
            }
            div{
                class:"lable",
                "Music"
            }
            div{
                class:"right-icon",
                span{class:"iconfont "}
            }
        }
    }
}

#[allow(non_snake_case)]
fn ToTop<'a>(cx:Scope)->Element{
    rsx!{cx,
        a{
            href:"#",
            id:"to-top",
            class:"item",
            div{class:"light-line"}
            div{
                class:"left-icon",
                span{class:"iconfont icon-xiangshang"}
            }
            div{
                class:"lable",
                "ToTop"
            }
            div{
                class:"right-icon",
                span{class:"iconfont "}
            }
        }
    }
}