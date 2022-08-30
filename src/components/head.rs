use dioxus::prelude::*;

#[derive(Props)]
pub struct HeadProps<'a> {
    children: Element<'a>,
}

/// ## 借助 JS 更改 HTML 的 head 标签
#[allow(non_snake_case)]
pub fn Head<'a>(cx: Scope<'a, HeadProps<'a>>) -> Element {
    let eval = dioxus::web::use_eval(&cx);

    if let Some(VNode::Fragment(fragment)) = &cx.props.children {
        fragment.children.iter().for_each(|child| {
            if let VNode::Element(element) = child {
                let tag = element.tag;
                let attributes: String = element
                    .attributes
                    .iter()
                    .map(|attribute| {
                        let name = attribute.name;
                        let value = attribute.value;
                        format!("el.setAttribute('{name}', '{value}');")
                    })
                    .collect();
                let children = &*element.children;

                let inner_text = match children.first() {
                    Some(VNode::Text(text)) => {
                        let text = text.text;
                        format!("el.innerText = '{text}'")
                    }
                    Some(VNode::Fragment(fragment)) if fragment.children.len() == 1 => fragment
                        .children
                        .first()
                        .and_then(|child| {
                            if let VNode::Text(text) = child {
                                let text = text.text.replace("}\n", "} ").replace('\n', "");
                                Some(format!("el.innerHTML = '{text}'"))
                            } else {
                                None
                            }
                        })
                        .unwrap_or_default(),
                    _ => "".to_owned(),
                };

                eval(format!(
                    r#"
                        let el = document.createElement('{tag}')
                        {attributes}
                        {inner_text}
                        document.head.appendChild(el)
                    "#
                ));
            }
        });
    }

    None
}