pub use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Render {
    pub colour: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}
