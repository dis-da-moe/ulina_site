pub mod viewbox;
use web_sys::{Element, HtmlCollection, HtmlInputElement, HtmlTextAreaElement, InputEvent};
use yew::{TargetCast, Html, html};

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

pub fn input_field<T: ToString, Y: ToString>(name: Y, value: T, hidden: bool, required: bool) -> Html {
    html! {
        <div>
        if !hidden{
            <label>{name.to_string()}</label>
        }
        <input class="text-input" type="text" name={name.to_string()} value={value.to_string()} hidden={hidden} required={required}/>
        </div>
    }
}

pub fn input_checkbox(e: InputEvent) -> bool {
    e.target_dyn_into::<HtmlInputElement>().unwrap().checked()
}

pub const XMLNS: &str = "http://www.w3.org/2000/svg";
pub const EMPTY_DIV: &str = "grid place-items-center h-20 text-md italic";
pub const BUTTON_CLASS: &str = "btn btn-primary text-center d-md-flex justify-content-md-center align-items-md-center btn-ulina min-w-[113px] min-h-[42px] text-[18px] h-[35px]";
#[allow(unused)]
pub fn log<T: ToString>(message: T) {
    web_sys::console::log_1(&message.to_string().into());
}

#[macro_export]
macro_rules! debug {
    () => {
        |err| format!("{:?}", err)
    };
}
