#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Supertype {
    // Official
    Basic,
    Legendary,
    Ongoing,
    Snow,
    World,

    // Non-official
    Elite,
    Host,
}

impl TryFrom<usize> for Supertype {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > Supertype::Host as usize {
            Err(())
        } else {
            Ok(unsafe { std::mem::transmute::<u8, Supertype>(value as u8) })
        }
    }
}

enum_set!(Supertypes, Supertype);
