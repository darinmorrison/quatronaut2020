use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, Write, WriteStorage},
};

use crate::{components::movement::Movement, entities::player::Player, resources::level::LevelMetadata};

use std::f32::consts::PI;

#[derive(SystemDesc)]
pub struct MovementTrackingSystem;

impl<'s> System<'s> for MovementTrackingSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Movement>,
        ReadStorage<'s, Player>,
    );

    fn run(&mut self, (transforms, mut movements, players): Self::SystemData) {
        for (movement, transform) in (&mut movements, &transforms).join() {
            for (_player, player_transform) in (&players, &transforms).join() {
                // this updates the x and y velocities on the enemy struct, which
                // can be used in another system to modify the transform
                // we can't modify it here because we can't take ownership of mut
                // transforms in the outer join and still get player transforms in the
                // inner join
                movement.next_move(
                    player_transform.translation().x,
                    player_transform.translation().y,
                    player_transform.translation().z,
                    transform.translation().x,
                    transform.translation().y,
                );
            }
        }
    }
}

// now we can update the transform
#[derive(SystemDesc)]
pub struct TransformUpdateSystem;

#[allow(clippy::type_complexity)]
impl<'s> System<'s> for TransformUpdateSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Movement>,
        Read<'s, Time>,
        Write<'s, LevelMetadata>,
        Entities<'s>,
    );

    fn run(&mut self, (mut transforms, mut movements, time, mut level_metadata, entities): Self::SystemData) {
        for (movement, enemy_entity, enemy_transform) in (&mut movements, &entities, &mut transforms).join() {
            enemy_transform.prepend_translation_x(movement.velocity_x * time.delta_seconds());
            enemy_transform.prepend_translation_y(movement.velocity_y * time.delta_seconds());
            // TODO: decide if we need to clamp movement for enemies. and if so, perhaps look into why
            // this code causes the game to speed through levels and exit quickly
            //let new_x = playable_area.clamp_x(movement.velocity_x * time.delta_seconds());
            //enemy_transform.prepend_translation_x(new_x);

            //let new_y = playable_area.clamp_y(movement.velocity_y * time.delta_seconds());
            //enemy_transform.prepend_translation_y(new_y);

            // these values should be based on game dimensions. the check is needed
            // for enemies that move off screen before getting hit
            let x = enemy_transform.translation().x;
            let y = enemy_transform.translation().y;

            // BIG TODO: the intended rotation here isn't working, probably due to some
            // fundamental misunderstanding from the code commenting narrator. for now there's
            // a quick hack to make sure the flying enemies at least point downward, and this can
            // be revisited later
            if let Some(_player_vec) = movement.locked_direction {
                if !movement.already_rotated {
                    enemy_transform.prepend_rotation_z_axis(PI);
                    movement.already_rotated = true;
                    //let enemy_vec = enemy_transform.translation();
                    //let radians = player_vec.dot(&enemy_vec).acos();
                    //info!("prepending rotation for {} radians", radians);
                    //enemy_transform.prepend_rotation_z_axis(radians); // + FRAC_PI_2);
                    //movement.already_rotated = true;
                }
            }

            // TODO: this should be based on some kind of "playable area" dimensions resource
            let out_of_bounds = x < -500.0 || x > 2500.0 || y < -500.0 || y > 2500.0;

            if out_of_bounds && entities.delete(enemy_entity).is_ok() {
                level_metadata.enemy_destroyed();
                //info!("enemy out of bounds");
                //info!("new enemy count is: {}", enemy_count.count);
            }
        }
    }
}
