#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
