use bevy::{prelude::*, window::{Window, WindowPlugin, WindowResolution}};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::BoardPlugin;
use board_plugin::resources::BoardOptions;
use board_plugin::resources::TileSize::Fixed;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(700.0, 800.0),
            title: "Mine Sweeper!".to_string(),
            ..Default::default()
        }),
        exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
        close_when_requested: true,
    }));

    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.insert_resource(BoardOptions {
        map_size: (20,20),
        bomb_count: 40,
        tile_padding: 3.0,
        // different from tutorial due to WindowDescriptor is not available as a resource
        tile_size: Fixed(20.0),
        ..Default::default()
    });

    app.add_plugins(BoardPlugin);

    app.add_systems(Startup, camera_setup);
    app.run();
}

fn camera_setup(mut commands: Commands) {    
    commands.spawn(Camera2d::default());
}
