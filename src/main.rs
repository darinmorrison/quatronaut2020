// this is the main entry point for our game. it was only slightly modified
// from the main.rs file in https://github.com/amethyst/amethyst-starter-2d
use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    assets::PrefabLoaderSystemDesc,
    utils::application_root_dir,
};

mod entities;
mod state;
mod systems;
use entities::enemy::EnemyPrefab;
use entities::player::PlayerPrefab;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = app_root.join("config").join("display_config.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_system_desc(PrefabLoaderSystemDesc::<EnemyPrefab>::default(), "", &[])
        .with_system_desc(PrefabLoaderSystemDesc::<PlayerPrefab>::default(), "", &[])
        .with(systems::PlayerSystem, "player_system", &["input_system"])
        // TODO: not sure if the input system is needed here?
        .with(systems::LaserSystem, "laser_system", &["input_system"])
        .with(systems::CollisionSystem, "collision_system", &["input_system"])
        .with(systems::AttackedSystem, "attacked_system", &["input_system"])
        .with(systems::EnemyTrackingSystem, "enemy_tracking_system", &["input_system"])
        .with(systems::EnemyMoveSystem, "enemy_move_system", &["input_system"])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;

    // 2.0 is the seconds to delay before enemy wave 1 spawns
    let mut game = Application::new(resources, state::GameplayState::new(2.0), game_data)?;
    game.run();

    Ok(())
}
