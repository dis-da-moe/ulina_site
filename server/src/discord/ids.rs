macro_rules! id {
    ($kind: ty, $($name: ident, $string: expr)+) => {
        $(pub const $name: $kind = $string;)+
    };
}

id!(
    &str,
    NAME, "name"
    CONTINENT, "continent"
    USER, "user"
    FLAG, "flag"
    NAME_INPUT, "nationName"
    DESCRIPTION_INPUT, "nationDescription"
    PLATFORM, "platform"
    LINK, "link"
    PREVIOUS, "previous"
    NEXT, "next"
    GUIDE, "guide"
    PASTA, "pasta"
);
