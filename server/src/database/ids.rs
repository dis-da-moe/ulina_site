use common::Nation;

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

pub trait Id<T>{
    fn id(&self) -> T;
}

id_type!(
    (SocialsId, socialsId), 
    (NationId, nationId), NationDiscord, Nation,
    (FlagId, flagId)
);