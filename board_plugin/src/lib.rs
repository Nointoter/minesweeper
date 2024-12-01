pub mod components;
pub mod resources;

use crate::components::Coordinates;
use crate::resources::{BoardPosition, TileSize};
use bevy::log;
use bevy::prelude::*;
use resources::tile_map;
use resources::tile_map::TileMap;
use resources::BoardOptions;

pub struct BoardPlugin;

impl BoardPlugin {
    fn create_board(mut commands: Commands, board_options: Option<Res<BoardOptions>>) {
        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };

        let tile_size = match options.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { .. } => panic!(
                "Not supported in this commit due to WindowDescriptor is not available as resource"
            ),
        };

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);

        // We deduce the size of the complete board
        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board_size: {}", board_size);

        // We define the board anchor position (bottom left)
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3 {
                    x: -(board_size.x / 2.0),
                    y: -(board_size.y / 2.0),
                    z: 0.0,
                } + offset
            }
            BoardPosition::Custom(p) => p,
        };

        tile_map.set_bombs(options.bomb_count);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        //adopted 0.8 to 0.9
        commands
            .spawn((
                Name::new("Board"),
                Transform::from_translation(board_position),
                GlobalTransform::default(),
                Visibility::default()
            ))
            .with_children(|parent| {
                parent.spawn((
                    Sprite {
                        color: Color::WHITE,
                        custom_size: Some(board_size),
                        ..Default::default()
                    },
                    Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                    GlobalTransform::default(),
                    Visibility::default(),
                    Name::new("Background"),
                ));
                
                for (y, line) in tile_map.iter().enumerate() {
                    for (x, tile) in line.iter().enumerate() {
                        parent.spawn((
                            Sprite {
                                color: Color::srgb(0.5, 0.5, 0.5),
                                custom_size: Some(Vec2::splat(tile_size - options.tile_padding as f32)),
                                ..Default::default()
                            },
                            Transform::from_xyz(
                                (x as f32 * tile_size) + (tile_size / 2.),
                                (y as f32 * tile_size) + (tile_size / 2.),
                                1.,
                            ),
                            Name::new(format!("Tile: ({}, {})", x, y)),
                            Coordinates {
                                x: x as u16,
                                y: y as u16,
                            },
                        ));
                    }
                }
            });
    }
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board);
        log::info!("Loaded Board Plugin");
    }
}