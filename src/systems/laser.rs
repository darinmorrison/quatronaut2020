use amethyst::{
    core::{timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, System, SystemData, WriteStorage},
};

use crate::entities::laser::{Direction::*, Laser};
use log::info;

// this system is concerned only with lasers that have already been spawned.
// the entity exists but the transform needs to be continuously updated based
// on the direction.
// if it collides with a border it should also be destroyed.
#[derive(SystemDesc)]
pub struct LaserSystem;

impl<'s> System<'s> for LaserSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Laser>,
        Entities<'s>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, lasers, entities, time): Self::SystemData) {
        for (entity, laser, transform) in (&entities, &lasers, &mut transforms).join() {
            // constant laser speed.. still shouldn't be hardcoded though.
            let &trans = transform.translation();
            let neg_x = trans.x - laser.speed * time.delta_seconds();
            let neg_y = trans.y - laser.speed * time.delta_seconds();
            let pos_x = trans.x + laser.speed * time.delta_seconds();
            let pos_y = trans.y + laser.speed * time.delta_seconds();

            // probably no reason to compute this every frame for every laser
            // it'd be easier to have the laser track `.next_change` or something
            // similar
            match &laser.direction {
                Left => {
                    transform.set_translation_x(neg_x);
                },
                Right => {
                    transform.set_translation_x(pos_x);
                },
                Up => {
                    transform.set_translation_y(pos_y);
                },
                Down => {
                    transform.set_translation_y(neg_y);
                },
                RightUp => {
                    transform.set_translation_x(pos_x);
                    transform.set_translation_y(pos_y);
                },
                LeftUp => {
                    transform.set_translation_x(neg_x);
                    transform.set_translation_y(pos_y);
                },
                LeftDown => {
                    transform.set_translation_x(neg_x);
                    transform.set_translation_y(neg_y);
                },
                RightDown => {
                    transform.set_translation_x(pos_x);
                    transform.set_translation_y(neg_y);
                },
            }

            // this will change when we add rudimentary collision detection. for now
            // it's just a bounds check that'll delete lasers once they go off screen.
            if trans.x < 0.0 || trans.x > 2500.0 || trans.y < 0.0 || trans.y > 2500.0 {
                let deleted = entities.delete(entity);

                if let Err(msg) = deleted {
                    info!("A terrible error has occured: {:?}", msg)
                }
            }
        }
    }
}
