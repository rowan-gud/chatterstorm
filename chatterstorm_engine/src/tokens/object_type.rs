#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjectType {
    // Permanents
    Artifact,
    Battle,
    Creature,
    Dungeon,
    Enchantment,
    Kindred,
    Land,
    Planeswalker,

    // Non-permanents
    Instant,
    Sorcery,
    Emblem,
    Hero,
    Stickers,

    // Casual
    Bounty,
    Conspiracy,
    Phenomenon,
    Plane,
    Scheme,
    Vanguard,

    // Deprecated
    Tribal,

    // Obselete
    Enchant,
    Interrupt,
    ManaSource,
    Summon,
}

impl TryFrom<usize> for ObjectType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > ObjectType::Summon as usize {
            Err(())
        } else {
            Ok(unsafe { std::mem::transmute::<u8, ObjectType>(value as u8) })
        }
    }
}

enum_set!(ObjectTypes, ObjectType);
