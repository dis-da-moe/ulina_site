use crate::{components::Flag, util::EMPTY_DIV};
use common::NationAll;
use yew::prelude::*;

pub fn show_info(nation: &NationAll, flag_load: Callback<()>, flag_loaded: bool) -> Html {
    let socials = nation.socials.iter().map(|social| {
        html! {
            <>
            {title(&social.platform)}
            <span class="infobox-field">
                <a class="text-blue-600 visited:text-purple-600" href={social.link.clone()}>
                    {&social.link}
                </a>
            </span>
            </>
        }
    });

    html! {
        <>
            <Flag flag={nation.flag_link.clone()} on_load={flag_load} loaded={flag_loaded}/>

            {field_title("Continent", &nation.core.continentName)}

            {for socials}

            if let Some(description) = nation.core.description.as_ref(){
                {field_title("Description", &description)}
            }
        </>
    }
}

pub fn show_trivia(nation: &NationAll) -> Html {
    let trivia: Vec<Html> = [
        ("Leader", nation.core.leader.as_ref()),
        ("Capital", nation.core.capital.as_ref()),
        ("Ideology", nation.core.ideology.as_ref()),
        ("Alliances", nation.core.alliances.as_ref()),
    ]
    .iter()
    .filter_map(|(name, content)| content.map(|content| field_title(name, content)))
    .collect();

    if trivia.is_empty() {
        html! {
            <div class={EMPTY_DIV}>
                {"no trivia"}
            </div>
        }
    } else {
        html! {
            {for trivia}
        }
    }
}

pub fn field_title(title: &str, content: &str) -> Html {
    html! {
        <>
        <span class="infobox-title">
            {title}
        </span>
        <span class="infobox-field">
            {content}
        </span>
        </>
    }
}

pub fn title(title: &str) -> Html {
    html! {
        <span class="infobox-title">
            {title}
        </span>
    }
}
