use common::current_url;
use oauth2::{
    basic::{BasicErrorResponseType, BasicTokenType},
    Client, EmptyExtraTokenFields, RevocationErrorResponseType, StandardErrorResponse,
    StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse,
};

pub const ZERO_WIDTH: &str = "\u{200B}";

pub fn capitalise(string: &str) -> String {
    string
        .trim()
        .split(" ")
        .filter_map(|word| {
            let mut chars = word.chars().collect::<Vec<_>>();
            if let Some(first) = chars.get_mut(0) {
                *first = first.to_ascii_uppercase();

                Some(chars.iter().collect::<String>())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn go_to_site(action: &str, subsection: &str) -> String {
    format!("to {} go to {}/{}", action, current_url(), subsection)
}

pub type StandardClient = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
>;
