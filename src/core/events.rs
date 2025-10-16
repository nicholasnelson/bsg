use bevy::prelude::*;
use crate::core::tile::TileId;

#[derive(Message, Clone, Copy)]
pub struct PlaceTile { pub x: u32, pub y: u32, pub tile: TileId }

#[derive(Message, Clone, Copy)]
pub struct RemoveTile { pub x: u32, pub y: u32 }
