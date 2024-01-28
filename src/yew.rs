use std::borrow::Cow;
use web_sys::Node;
use yew::virtual_dom::VNode;
use yew::{prelude::*, Html};
use crate::{Htmlify, Attribute};

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct Props
{
    pub tag: String,
    pub attributes: Vec<Attribute>,
    pub html: Cow<'static, str>
}

#[function_component]
pub fn RawHtml(props: &Props) -> Html
{
    VNode::VRef(Node::from
    ({
        let element = web_sys::window().unwrap().document().unwrap().create_element(&props.tag).unwrap();
        for attr in props.attributes.iter()
        {
            element.set_attribute(&attr.name, &attr.value).unwrap();
        }
        element.set_inner_html(&props.html);
        element
    }))
}

impl RawHtml
{
    pub fn from<T: Htmlify>(t: &T) -> Html
    {
        html!
        {
            <RawHtml tag={t.tag().to_string()} attributes={t.attributes()} html={Cow::from(t.inner_html_as_string())} />
        }
    }
}

