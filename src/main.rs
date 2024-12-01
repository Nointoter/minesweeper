use bevy::{prelude::*, window::{Window, WindowPlugin, WindowResolution}};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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

    app.add_systems(Startup, camera_setup);
    app.run();
}

fn camera_setup(mut commands: Commands) {    
    commands.spawn(Camera2d::default());
}
