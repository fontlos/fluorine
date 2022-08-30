mod page;
mod components;

use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    rsx! {cx,
        Router{
            Route{
                to: "/",
                page::home{}
            }
            Route{
                to: "/resources",
                page::resources{}
            }
            // Route{
            //     to: "/resources/:res",
            //     page::resources{}
            // }
            Route{
                to: "/resources/:res/:title",
                page::resources_page{}
            }
            Route{
                to: "/blog",
                page::blog{}
            }
            Route{
                to:"/blog/:id",
                page::blog_page{}
            }
            Route{
                to: "",
                page::not_found{}
            }
        }
    }
}
