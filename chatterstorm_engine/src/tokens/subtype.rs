#[repr(u16)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Subtype {
    // Artifact
    Attraction,
    Blood,
    Bobblehead,
    Clue,
    Contraption,
    Equipment,
    Food,
    Fortification,
    Gold,
    Incubator,
    Junk,
    Key,
    Map,
    Powerstone,
    Treasure,
    Vehicle,

    // Battle
    Siege,

    // Creature / Kindred
    Advisor,
    Aetherborn,
    Alien,
    Ally,
    Angel,
    Antelope,
    Ape,
    Archer,
    Archon,
    Armadillo,
    Army,
    Artificer,
    Assassin,
    AssemblyWorker,
    Astartes,
    Atog,
    Aurochs,
    Automaton,
    Avatar,
    Azra,
    Badger,
    Balloon,
    Barbarian,
    Bard,
    Basilisk,
    Bat,
    Bear,
    Beast,
    Beaver,
    Beeble,
    Beholder,
    Berserker,
    Bird,
    Blinkmoth,
    Boar,
    Brainiac,
    Bringer,
    Brushwagg,
    Ctan,
    Camarid,
    Camel,
    Capybara,
    Caribou,
    Carrier,
    Cat,
    Centaur,
    Chicken,
    Child,
    Chimera,
    Citizen,
    Clamfolk,
    Cleric,
    Clown,
    Cockatrice,
    Construct,
    Coward,
    Coyote,
    Crab,
    Crocodile,
    Custodes,
    Cyberman,
    Cyborg,
    Cyclops,
    Dalek,
    Dauthi,
    Demigod,
    Demon,
    Deserter,
    Detective,
    Devil,
    Dinosaur,
    Djinn,
    Doctor,
    Dog,
    Dragon,
    Drake,
    Dreadnought,
    Drone,
    Druid,
    Dryad,
    Dwarf,
    Efreet,
    Egg,
    Elder,
    Eldrazi,
    Elemental,
    Elephant,
    Elf,
    Elk,
    Employee,
    Eye,
    Faerie,
    Ferret,
    Fish,
    Flagbearer,
    Fox,
    Fractal,
    Frog,
    Fungus,
    Gamer,
    Gargoyle,
    Germ,
    Giant,
    Gith,
    Gnoll,
    Gnome,
    Goat,
    Goblin,
    God,
    Golem,
    Gorgon,
    Graveborn,
    Gremlin,
    Griffin,
    Guest,
    Hag,
    Halfling,
    Hamster,
    Harpy,
    Head,
    Hellion,
    Hippo,
    Hippogriff,
    Homarid,
    Homunculus,
    Hornet,
    Horror,
    Horse,
    Human,
    Hydra,
    Hyena,
    Illusion,
    Imp,
    Incarnation,
    Inkling,
    Inquisitor,
    Insect,
    Jackal,
    Jaguar,
    Jellyfish,
    Juggernaut,
    Kavu,
    Kirin,
    Kithkin,
    Knight,
    Kobold,
    Kor,
    Kraken,
    Lamia,
    Lammasu,
    Leech,
    Leviathan,
    Lhurgoyf,
    Licid,
    Lizard,
    Manticore,
    Masticore,
    Mercenary,
    Merfolk,
    Metathran,
    Minion,
    Minotaur,
    Mite,
    Mole,
    Monger,
    Mongoose,
    Monk,
    Monkey,
    Moonfolk,
    Mount,
    Mouse,
    Mutant,
    Myr,
    Mystic,
    Naga,
    Nautilus,
    Necron,
    Nephilim,
    Nightmare,
    Nightstalker,
    Ninja,
    Noble,
    Noggle,
    Nomad,
    Nymph,
    Octopus,
    Ogre,
    Ooze,
    Orb,
    Orc,
    Orgg,
    Otter,
    Ouphe,
    Ox,
    Oyster,
    Pangolin,
    Peasant,
    Pegasus,
    Pentavite,
    Performer,
    Pest,
    Phelddagrif,
    Phoenix,
    Phyrexian,
    Pilot,
    Pincher,
    Pirate,
    Plant,
    Pony,
    Porcupine,
    Possum,
    Praetor,
    Primarch,
    Prism,
    Processor,
    Rabbit,
    Raccoon,
    Ranger,
    Rat,
    Rebel,
    Reflection,
    Reveler,
    Rhino,
    Rigger,
    Robot,
    Rogue,
    Rukh,
    Sable,
    Salamander,
    Samurai,
    Sand,
    Saproling,
    Satyr,
    Scarecrow,
    Scientist,
    Scion,
    Scorpion,
    Scout,
    Sculpture,
    Serf,
    Serpent,
    Servo,
    Shade,
    Shaman,
    Shapeshifter,
    Shark,
    Sheep,
    Ship,
    Siren,
    Skeleton,
    Slith,
    Sliver,
    Sloth,
    Slug,
    Snail,
    Snake,
    Soldier,
    Soltari,
    Spawn,
    Specter,
    Spellshaper,
    Sphinx,
    Spider,
    Spike,
    Spirit,
    Splinter,
    Sponge,
    Spy,
    Squid,
    Squirrel,
    Starfish,
    Surrakar,
    Survivor,
    Synth,
    Teddy,
    Tentacle,
    Tetravite,
    Thalakos,
    Thopter,
    Thrull,
    Tiefling,
    TimeLord,
    Townsfolk,
    Toy,
    Treefolk,
    Trilobite,
    Triskelavite,
    Troll,
    Turtle,
    Tyranid,
    Unicorn,
    Urzan,
    Vampire,
    Varmint,
    Vedalken,
    Volver,
    Waiter,
    Wall,
    Walrus,
    Warlock,
    Warrior,
    Wasp,
    Weird,
    Werewolf,
    Whale,
    Wizard,
    Wolf,
    Wolverine,
    Wombat,
    Worm,
    Wraith,
    Wurm,
    Yeti,
    Zombie,
    Zubera,

    // Enchantment
    Aura,
    Background,
    Cartouche,
    Case,
    Class,
    Curse,
    Glimmer,
    Role,
    Rune,
    Saga,
    Shard,
    Shrine,

    // Land
    Cave,
    Cloud,
    Desert,
    Forest,
    Gate,
    Island,
    Lair,
    Locus,
    Mine,
    Mountain,
    Sphere,
    Plains,
    PowerPlant,
    Swamp,
    Tower,
    Urzas,

    // Planeswalker
    Abian,
    Ajani,
    Aminatou,
    Angrath,
    Arlinn,
    Ashiok,
    Bob,
    Bahamut,
    Basri,
    Bolas,
    Calix,
    Chandra,
    Comet,
    Dack,
    Dakkon,
    Daretti,
    Davriel,
    Deb,
    Dihada,
    Domri,
    Dovin,
    Duck,
    Dungeon,
    Ellywick,
    Elminster,
    Elspeth,
    Ersta,
    Estrid,
    Freyalise,
    Garruk,
    Gideon,
    Grist,
    Guff,
    Huatli,
    Inzerva,
    Jace,
    Jared,
    Jaya,
    Jeska,
    Kaito,
    Karn,
    Kasmina,
    Kaya,
    Kiora,
    Koth,
    Liliana,
    Lolth,
    Lukka,
    Master,
    Minsc,
    Mordenkainen,
    Nahiri,
    Narset,
    Niko,
    Nissa,
    Nixilis,
    Oko,
    Quintorius,
    Ral,
    Rowan,
    Saheeli,
    Samut,
    Sarkhan,
    Serra,
    Sivitri,
    Sorin,
    Svega,
    Szat,
    Tamiyo,
    Tasha,
    Teferi,
    Teyo,
    Tezzeret,
    Tibalt,
    Tyvar,
    Ugin,
    Urza,
    Venser,
    Vivien,
    Vraska,
    Vronos,
    Wanderer,
    Will,
    Windgrace,
    Wrenn,
    Xenagos,
    Yanggu,
    Yanling,
    Zariel,

    // Non-permanents
    Adventure,
    Arcane,
    Chorus,
    Lesson,
    Trap,

    // Plane
    TheAbyss,
    Alara,
    Amonkhet,
    Antausia,
    Arcavios,
    Arkhos,
    Azgol,
    Belenon,
    Bolass,
    MeditationRealm,
    Capenna,
    Cridhe,
    Dominaria,
    Echoir,
    Eldraine,
    Equilor,
    Ergamon,
    Fabacin,
    Fiora,
    Gargantikar,
    Gobakhan,
    Ikoria,
    Innistrad,
    Iquatana,
    Ir,
    Ixalan,
    Kaladesh,
    Kaldheim,
    Kamigawa,
    Karsus,
    Kephalai,
    Kinshala,
    Kolbahan,
    Kylem,
    Kyneth,
    Lorwyn,
    Luvion,
    Mercadia,
    Mirrodin,
    Moag,
    Mongseng,
    Muraganda,
    NewPhyrexia,
    Phyrexia,
    Pyrulea,
    Rabiah,
    Rath,
    Ravnica,
    Regatha,
    Segovia,
    SerrasRealm,
    Shadowmoor,
    Shandalar,
    Shenmeng,
    Tarkir,
    Theros,
    Ulgrotha,
    Valla,
    Vryn,
    Wildfire,
    Xerex,
    Zendikar,
    Zhalfir,
    AlfavaMetraxis,
    AndrozaniMinor,
    Apalapucia,
    Darillium,
    Earth,
    Gallifrey,
    HorseheadNebula,
    Kandoka,
    Mars,
    Moon,
    Necros,
    NewEarth,
    OutsideMuttersSpiral,
    Skaro,
    Spacecraft,
    TheDalekAsylum,
    TheLibrary,
    Time,
    Trenzalore,
    UnknownPlanet,

    // Dungeon
    Undercity,
}

impl TryFrom<usize> for Subtype {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > Subtype::Undercity as usize {
            Err(())
        } else {
            Ok(unsafe { std::mem::transmute::<u16, Subtype>(value as u16) })
        }
    }
}

enum_set!(Subtypes, Subtype);
