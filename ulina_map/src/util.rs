use web_sys::{Element, HtmlCollection};

pub fn get_vec(collection: &HtmlCollection) -> Vec<Element> {
    (0..collection.length())
        .map(|i| collection.get_with_index(i).unwrap())
        .collect()
}

pub fn by_id(element: &Element, id: String) -> Option<Element> {
    get_vec(&element.children())
        .into_iter()
        .find(|e| e.id() == id)
}

pub const EMPTY_DIV: &str = "grid place-items-center h-20 text-md italic";

#[allow(unused)]
pub fn log(message: String) {
    web_sys::console::log_1(&message.into());
}

#[macro_export]
macro_rules! debug {
    () => {
        |err| format!("{:?}", err)
    };
}
