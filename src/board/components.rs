use bevy::prelude::*;
use crate::board::resources::Player;

#[derive(Component)]
pub struct BoardRoot;

#[derive(Component)]
pub struct Cell {
    pub row: u8,
    pub col: u8,
}

#[derive(Component)]
pub struct CellOwner {
    pub player: Player,
}

#[derive(Component)]
pub struct XSprite;

#[derive(Component)]
pub struct OSprite;
