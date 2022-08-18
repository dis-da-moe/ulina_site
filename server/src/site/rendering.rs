use rocket::response::content::RawHtml;
use sycamore::{SsrNode, view};

static TEMPLATE: &str = include_str!("../../../tools/index.html");

pub trait Render {
    fn render(self) -> RawHtml<String>;
}

impl Render for sycamore::view::View<SsrNode> {
    fn render(self) -> RawHtml<String> {
        let button = sycamore::render_to_string(|| view!(a(href="/tools"){"back"}));
        let content = format!("<body>{}{}", button, sycamore::render_to_string(|| self));
        RawHtml(TEMPLATE.replace("<body>", &content))  
    }

}
