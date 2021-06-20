use std::collections::HashMap;

use crate::physics::GridPoint;
use ggez::{GameError, GameResult};
use tiled::{Image as TiledImage, Layer, PropertyValue, Tileset};

#[derive(Clone)]
pub enum TerrainTileId {
    ShortGrass,
    MiddleGrass,
    HighGrass,
    Dirt,
    Mud,
    Concrete,
    BrickWall,
}

#[derive(Clone)]
pub struct TerrainTile {
    pub id: TerrainTileId,
    pub tile_width: u32,
    pub tile_height: u32,
    pub relative_tile_width: f32,
    pub relative_tile_height: f32,
    pub tile_x: u32,
    pub tile_y: u32,
    pub opacity: f32,
    pub pedestrian_cost: i32,
}

impl TerrainTile {
    pub fn from_str_id(
        id: &str,
        tile_width: u32,
        tile_height: u32,
        relative_tile_width: f32,
        relative_tile_height: f32,
        tile_x: u32,
        tile_y: u32,
    ) -> Self {
        match id {
            "ShortGrass" => Self {
                id: TerrainTileId::ShortGrass,
                opacity: 0.0,
                tile_width,
                tile_height,
                relative_tile_width,
                relative_tile_height,
                tile_x,
                tile_y,
                pedestrian_cost: 10,
            },
            "MiddleGrass" => Self {
                id: TerrainTileId::MiddleGrass,
                opacity: 0.025,
                tile_width,
                tile_height,
                relative_tile_width,
                relative_tile_height,
                tile_x,
                tile_y,
                pedestrian_cost: 10,
            },
            "HighGrass" => Self {
                id: TerrainTileId::HighGrass,
                opacity: 0.05,
                tile_width,
                tile_height,
                relative_tile_width,
                relative_tile_height,
                tile_x,
                tile_y,
                pedestrian_cost: 11,
            },
            "Dirt" => Self {
                id: TerrainTileId::Dirt,
                opacity: 0.0,
                tile_width,
                tile_height,
                relative_tile_width,
                relative_tile_height,
                tile_x,
                tile_y,
                pedestrian_cost: 10,
            },
            "Mud" => Self {
                id: TerrainTileId::Mud,
                opacity: 0.02,
                tile_width,
                tile_height,
                relative_tile_width,
                relative_tile_height,
                tile_x,
                tile_y,
                pedestrian_cost: 11,
            },
            "Concrete" => Self {
                id: TerrainTileId::Concrete,
                opacity: 0.0,
                tile_width,
                tile_height,
                relative_tile_width,
                relative_tile_height,
                tile_x,
                tile_y,
                pedestrian_cost: 10,
            },
            "BrickWall" => Self {
                id: TerrainTileId::BrickWall,
                opacity: 3.0,
                tile_width,
                tile_height,
                relative_tile_width,
                relative_tile_height,
                tile_x,
                tile_y,
                pedestrian_cost: 50,
            },
            &_ => {
                // FIXME BS NOW: manage errors
                panic!("Unknown tile id {}", id)
            }
        }
    }
}

pub struct Terrain {
    pub tileset: Tileset,
    pub layer: Layer,
    pub image: TiledImage,
    // FIXME (u32, u32) -> GridPoint
    pub tiles: HashMap<(u32, u32), TerrainTile>,
}

impl Terrain {
    pub fn new(
        tileset: Tileset,
        layer: Layer,
        image: TiledImage,
        tiles: HashMap<(u32, u32), TerrainTile>,
    ) -> Self {
        Self {
            tileset,
            layer,
            image,
            tiles,
        }
    }
}

pub fn get_tile_from_terrain_tileset_with_id(
    terrain_tileset: &Tileset,
    id: u32,
    terrain_image_width: u32,
    terrain_image_height: u32,
) -> GameResult<TerrainTile> {
    for tile in terrain_tileset.tiles.iter() {
        if tile.id == id - terrain_tileset.first_gid {
            let str_id = match tile.properties.get("ID") {
                None => {
                    return GameResult::Err(GameError::ResourceLoadError(format!(
                        "Tile {} have no ID property",
                        id
                    )))
                }
                Some(property_value) => match property_value {
                    PropertyValue::StringValue(str_id) => str_id.clone(),
                    _ => {
                        return GameResult::Err(GameError::ResourceLoadError(format!(
                            "Tile {} must have String ID property value",
                            id
                        )))
                    }
                },
            };

            let tile_width = terrain_tileset.tile_width;
            let tile_height = terrain_tileset.tile_height;
            let relative_tile_width = tile_width as f32 / terrain_image_width as f32;
            let relative_tile_height = tile_height as f32 / terrain_image_height as f32;
            let len_by_width = terrain_image_width / tile_width;
            let tile_y = tile.id / len_by_width;
            let tile_x = tile.id - (tile_y * len_by_width);

            return GameResult::Ok(TerrainTile::from_str_id(
                &str_id,
                tile_width,
                tile_height,
                relative_tile_width,
                relative_tile_height,
                tile_x,
                tile_y,
            ));
        }
    }

    return GameResult::Err(GameError::ResourceLoadError(format!(
        "No tile with {} found",
        id
    )));
}

pub fn grid_points_for_square(center_point: &GridPoint, width: i32, height: i32) -> Vec<GridPoint> {
    let mut points = vec![];

    let start_x = center_point.x - (height / 2);
    let end_x = start_x + height;
    let start_y = center_point.y - (width / 2);
    let end_y = start_y + width;

    for x in start_x..end_x {
        for y in start_y..end_y {
            points.push(GridPoint::new(x, y))
        }
    }

    points
}
