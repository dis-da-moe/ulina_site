use rocket::response::content::RawHtml;
use sycamore::SsrNode;

const TEMPLATE: &str = include_str!("./template.html");

pub trait Render {
    fn render(self) -> RawHtml<String>;
}

impl Render for sycamore::view::View<SsrNode> {
    fn render(self) -> RawHtml<String> {
        RawHtml(TEMPLATE.replace(
            "<template></template>",
            &sycamore::render_to_string(|| self),
        ))
    }
}
