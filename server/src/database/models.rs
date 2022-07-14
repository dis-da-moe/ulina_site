#[macro_export]
macro_rules! nation_id {
    ($interaction: expr) => {
        crate::get_nation!($interaction, NationId, "nationId, name, ownerDiscord")
    };
}

#[macro_export]
macro_rules! nation {
    ($interaction: expr) => {
        crate::get_nation!($interaction, Nation, "*")
    };
}
