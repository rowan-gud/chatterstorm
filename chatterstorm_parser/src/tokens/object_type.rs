#[derive(Debug)]
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
