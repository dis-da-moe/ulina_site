mod all_nations;
mod commands;
mod create_nation;
mod edit_flag;
mod edit_name_description;
mod edit_socials;
mod nation;
mod ping;
mod time;
mod remove_nation;
mod shared;
pub use shared::create_commands;

use shared::UlinaCommand;
use std::collections::HashMap;

macro_rules! add_commands {
    ($map: expr => $($action: ident),+) => {
        $($map.insert($action::DATA.name, UlinaCommand{
            data: $action::DATA,
            action: |a, b| Box::pin($action::$action(a, b)),
            create: |command| {
                $action::create(command)
            }
        });)+
    };
}

lazy_static! {
    pub static ref COMMANDS: HashMap<&'static str, UlinaCommand> = {
        let mut map = HashMap::new();
        add_commands!(map => ping, nation, remove_nation, commands, create_nation, edit_flag, edit_socials, edit_name_description, all_nations, time);
        map
    };
}
