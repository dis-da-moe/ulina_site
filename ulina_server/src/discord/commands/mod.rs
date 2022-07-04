mod commands;
mod create_nation;
mod nation;
mod ping;
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
        add_commands!(map => ping, nation, remove_nation, commands, create_nation);
        map
    };
}
