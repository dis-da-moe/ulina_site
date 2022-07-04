use web_sys::{Element, HtmlCollection};

pub fn get_vec(collection: &HtmlCollection) -> Vec<Element> {
    (0..collection.length())
        .map(|i| collection.get_with_index(i).unwrap())
        .collect()
}

pub fn by_id(element: &Element, id: String) -> Option<Element>{
    get_vec(&element.children()).into_iter().find(|e| e.id() == id)
}

#[allow(unused)]
pub fn log(message: String) {
    web_sys::console::log_1(&message.into());
}
