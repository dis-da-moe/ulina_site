use chrono::{Local, NaiveDate, NaiveDateTime};
use common::{to_real, DATE_FORMAT};
use common::{to_ulina, TimeError};
use gloo::timers::callback::Interval;
use yew::prelude::*;

use crate::navbar;
use crate::util::input_text;

pub struct App {
    current_real: chrono::DateTime<Local>,
    current_ulina: chrono::NaiveDateTime,
    convert_real: Option<Result<NaiveDate, TimeError>>,
    convert_ulina: Option<Result<NaiveDate, TimeError>>,
}

const TIME_FORMAT: &str = "%H:%M:%S";
const DATE_INPUT_FORMAT: &str = "%Y-%m-%d";

pub enum Msg {
    Time,
    Input(Convert, String),
}

#[derive(Debug, Clone, Copy)]
pub enum Convert {
    Real,
    Ulina,
}
type DateResult = Result<NaiveDate, TimeError>;

type Converter = fn(i64) -> DateResult;

fn conversion(string: &str, conversion: Converter, result: &mut Option<DateResult>) {
    *result = Some(
        NaiveDate::parse_from_str(string, DATE_INPUT_FORMAT)
            .map_err(|_| TimeError::InvalidDate)
            .and_then(|date| conversion(date.and_hms(0, 0, 0).timestamp())),
    );
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let now = Local::now();
        let timer = {
            let link = ctx.link().clone();
            Interval::new(250, move || link.send_message(Msg::Time))
        };
        timer.forget();
        let now_ulina = to_ulina(now.timestamp()).unwrap();
        App {
            current_real: now.clone(),
            current_ulina: now_ulina,
            convert_real: Some(Ok(now.naive_local().date())),
            convert_ulina: Some(Ok(now_ulina.date())),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Input(convert, input) => {
                self.convert_real = None;
                self.convert_ulina = None;

                if input.is_empty() {
                    return true;
                }

                let (converter, target): (Converter, &mut Option<DateResult>) = match convert {
                    Convert::Real => (to_ulina, &mut self.convert_ulina),
                    Convert::Ulina => (to_real, &mut self.convert_real),
                };

                conversion(&input, converter, target);

                true
            }
            Msg::Time => {
                let now = Local::now();
                self.current_real = now.clone();
                self.current_ulina = to_ulina(now.timestamp()).unwrap();

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = |convert: Convert| {
            ctx.link().callback(move |e: InputEvent| {
                Msg::Input(
                    convert,
                    input_text(e),
                )
            })
        };
        
        html! {
            <>
            {navbar!()}

            {time_section(&self.current_real.naive_utc(), "Real")}
            {time_section(&self.current_ulina, "Ulina")}

            <div class="text-center mt-6 underline font-bold text-lg"> {"Convert"} </div>

            <div class="grid grid-cols-2 place-items-stretch mt-5">
                {time_input("Real time", self.convert_real.clone(), oninput(Convert::Real))}
                {time_input("Ulina time", self.convert_ulina.clone(), oninput(Convert::Ulina))}
            </div>
            </>
        }
    }
}

fn time_section(date_time: &NaiveDateTime, name: &str) -> Html {
    const TIME_SECTION: &str = "grid space-y-2 place-items-center mt-7 border-solid border-slate-600 border-b-4 pb-6 w-[70%] mx-auto";

    html! {
        <div class={TIME_SECTION}>
        <div class="font-bold text-base underline">{format!("{} Time:", name)}</div>
            <div>{date_time.date().format(DATE_FORMAT)}</div>
            <div>{date_time.time().format(TIME_FORMAT)}</div>
        </div>
    }
}

fn time_input(
    name: &str,
    date: Option<Result<NaiveDate, TimeError>>,
    oninput: Callback<InputEvent>,
) -> Html {
    let value = date
        .clone()
        .map_or(None, |time| time.ok())
        .map(|date| date.format(DATE_INPUT_FORMAT).to_string());

    html! {
        <div class="grid justify-center">
            <span class="italic text-sm">{name}</span>
            <input {oninput} style="background:white;color:black;" type="date" value={value}/>

            if let Some(Err(err)) = date{
                <p class="italic text-sm">{format!("{:?}", err)}</p>
            }
        </div>
    }
}
