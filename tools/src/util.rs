use web_sys::{Element, HtmlCollection, HtmlInputElement, HtmlTextAreaElement, InputEvent};
use yew::TargetCast;

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

pub fn input_text(e: InputEvent) -> String {
    e.target_dyn_into::<HtmlInputElement>()
        .map(|element| element.value())
        .unwrap_or_else(|| e.target_dyn_into::<HtmlTextAreaElement>().unwrap().value())
}

pub fn input_checkbox(e: InputEvent) -> bool {
    e.target_dyn_into::<HtmlInputElement>().unwrap().checked()
}

pub const EMPTY_DIV: &str = "grid place-items-center h-20 text-md italic";
pub const BUTTON_CLASS: &str = "btn btn-primary text-center d-md-flex justify-content-md-center align-items-md-center btn-ulina min-w-[113px] min-h-[42px] text-[18px] h-[35px]";
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
