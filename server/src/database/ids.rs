use common::{Nation, Social, NationAll};

use super::NationDiscord;

macro_rules! id_type {
    ($(($name: tt, $field_name: tt) $(, $model: ident)*),+) => {
    $(  #[derive(Debug, Clone, Copy)]
        pub struct $name(pub i64);

    $(
        impl Id<$name> for $model{
            fn id(&self) -> $name{
                $name(self.$field_name)
            }
        }
    )*
    )+
    };
}

pub trait Id<T> {
    fn id(&self) -> T;
}

id_type!(
    (SocialsId, socialsId),
    Social,
    (NationId, nationId),
    NationDiscord,
    Nation,
    (FlagId, flagId)
);

impl Id<NationId> for NationAll{
    fn id(&self) -> NationId {
        self.core.id()
    }
}
