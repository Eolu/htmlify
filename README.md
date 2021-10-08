Trait used to generate HTML from Rust structures

It contains the following optional definitions:
```rust,ignore
// Defaults to an empty string
fn tag(&self) -> Cow<'static, str>;
// Defaults to empty vec
fn attributes(&self) -> Vec<Attribute>;
// Defaults to empty vec
fn inner_html(&self) -> Cow<'static, str>;
```
as well as the following definition which should not need to be implemented, but may on occasion be useful to be overridden:
```ignore
fn as_raw_html(&self) -> String 
{
    format!
    (
        "<{0} {2}> {1} </{0}>",
        self.tag(),
        self.inner_html(),
        self.attributes()
            .iter()
            .map(Attribute::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    )
}
```
Finally, the following may be called to get a structured js_sys type:
```ignore
fn as_element(&self) -> Option<web_sys::Element>;
```