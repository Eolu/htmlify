#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#[cfg(feature = "yew")]
pub mod yew;
#[cfg(feature = "yew")]
use ::yew::{html, Html};

use std::{borrow::Cow, fmt::Display};

/// Trait used to generate HTML from Rust structures. See module-level 
/// documentation for more details.
pub trait Htmlify
{
    /// Sets the HTML tag associated when converting this to an element.
    fn tag(&self) -> Cow<'static, str> { Cow::Borrowed("") }
    /// Sets the attributes to include when converting this to an element.
    fn attributes(&self) -> Vec<Attribute> { vec!() }
    /// Get the inner HTML
    fn inner_html(&self) -> Vec<Box<dyn Htmlify>> { vec!() }
    /// Stringifies the inner HTML
    fn inner_html_as_string(&self) -> String
    {
        self.inner_html().iter().map(|e| e.as_raw_html()).collect::<Vec<String>>().join("")
    }
    /// Convert this to a raw string of HTML
    fn as_raw_html(&self) -> String 
    {
        format!
        (
            "<{0} {2}> {1} </{0}>",
            self.tag(),
            self.inner_html_as_string(),
            self.attributes()
                .iter()
                .map(Attribute::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
    /// Convert this into a [web_sys::Element]
    #[cfg(feature = "web-sys")]
    fn as_element(&self) -> Option<web_sys::Element>
    {
        use std::borrow::Borrow;
        let document = web_sys::window().and_then(|win| win.document())?;
        let element = document.create_element(self.tag().borrow()).ok()?;
        for attribute in self.attributes()
        {
            element.set_attribute(attribute.name.borrow(), attribute.value.borrow()).ok()?;
        }
        for inner in self.inner_html()
        {
            if let "__STRING_MARKER" = inner.tag().borrow()
            {
                element.append_with_str_1(&inner.as_raw_html()).ok()?;
            }
            else
            {
                element.append_with_node_1(inner.as_element()?.as_ref()).ok()?;
            }
        }
        Some(element)
    }

    /// Convert this into [::yew::Html]
    #[cfg(feature = "yew")]
    fn as_yew_node(&self) -> Html
    {
        html!
        {
            <crate::yew::RawHtml 
                tag={self.tag().to_string() }
                attributes={self.attributes()} 
                html={Cow::from(self.inner_html_as_string())} 
            />
        }
    }
}

/// HTML Attribute wrapper, a simple key-value string pair
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Attribute
{
    pub name: Cow<'static, str>, 
    pub value: Cow<'static, str>
}
impl Attribute
{
    pub fn new(name: impl Into<Cow<'static, str>>, value: impl Into<Cow<'static, str>>) -> Self
    {
        Attribute { name: name.into(), value: value.into() }
    }
}
impl Display for Attribute
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result 
    {
        let value = self.value.to_string();
        if value.is_empty()
        {
            write!(f, "{}", self.name)
        }
        else
        {
            write!(f, "{}=\"{}\"", self.name, self.value)
        }
    }
}

/// Raw strings are a special case of Htmlify
impl Htmlify for &str
{
    /// Used as a sentinal value
    fn tag(&self) -> Cow<'static, str> { Cow::Borrowed("__STRING_MARKER") }
    /// Strings are nothing but raw HTML
    fn as_raw_html(&self) -> String { self.to_string() }
    /// Raw strings are not elements, return None.
    #[cfg(feature = "web-sys")]
    fn as_element(&self) -> Option<web_sys::Element> { None }
}

/// Helper function which appends some `impl Htmlify` to the [web_sys] document 
/// body. Combined with the rest of this crate, may be used to generate the entire 
/// HTML body in arbitrary Rust-driven ways.
#[cfg(feature = "web-sys")]
pub fn append_to_document_body(htmlifiable: impl Htmlify) -> Result<(), impl std::fmt::Debug>
{ 
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.body())
        .and_then(|body| htmlifiable.as_element().map(|e| body.append_with_node_1(e.as_ref())))
        .transpose()
        .map(|_|())
}