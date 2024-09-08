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

#[derive(Clone, Debug, PartialEq)]
pub struct Halth {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
