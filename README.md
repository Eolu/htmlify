Trait used to generate HTML from Rust structures

It contains the following optional definitions:
```rust,ignore
// Defaults to an empty string
fn tag(&self) -> Cow<'static, str> { /* ... */ }
// Defaults to empty vec
fn attributes(&self) -> Vec<Attribute> { /* ... */ }
// Defaults to empty vec
fn inner_html(&self) -> Vec<Box<dyn Htmlify>> { /* ... */ }
```
The following method should not need to be implemented:
```rust,ignore
// Calls `as_raw_html` for each of the items returned by `inner_html`
fn inner_html_as_string(&self) -> String { /* ... */ }
```
The following definition also should not need to be implemented, but may on occasion be useful to override:
```rust,ignore
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
```
The `yew` feature may be enabled to allow access to the following default function:
```rust,ignore
fn as_yew_node(&self) -> Html { /* ... */ }
```
Finally, the following may be called to get a structured js_sys type:
```rust,ignore
fn as_element(&self) -> Option<web_sys::Element> { /* ... */ }
```

There are 4 other tools included in this crate:
- An `Htmlify` implementation for `&str`. This is a special case which simply returns the string itself when `as_raw_html` is called, and returns `None` when `as_element` is called. This is so text-content "leaves" can be represented. (and the default implementation of `as_element` understands this).
- An `Attribute` struct to store key-value pairs as strings, used specifically for element attributes. These use `Cow<'static, str>` to leave optimization details up to the implementor.
- An `append_to_document_body` function which takes any `impl Htmlify` and attempts to append it to the document body (through web_sys calls). This is here purely for convenience.
- If the `yew` feature is enabled, an `as_yew_node` function will also be available.