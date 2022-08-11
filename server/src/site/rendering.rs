use rocket::response::content::RawHtml;
use sycamore::SsrNode;

static TEMPLATE: &str = include_str!("../../../tools/index.html");

pub trait Render {
    fn render(self) -> RawHtml<String>;
}

impl Render for sycamore::view::View<SsrNode> {
    fn render(self) -> RawHtml<String> {
        let content = format!("<body>{}", sycamore::render_to_string(|| self));
        RawHtml(TEMPLATE.replace("<body>", &content))
    }
}
