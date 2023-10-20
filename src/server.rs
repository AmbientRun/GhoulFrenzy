use ambient_api::{
    animation::{AnimationPlayerRef, PlayClipFromUrlNodeRef},
    animation_element::{AnimationPlayer, PlayClipFromUrl, Transition},
    core::{
        animation::components::apply_animation_player,
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        model::components::model_from_url,
        physics::components::plane_collider,
        player::components::is_player,
        prefab::components::prefab_from_url,
        primitives::components::quad,
        transform::{
            components::{local_to_world, lookat_target, rotation, translation},
            concepts::{Transformable, TransformableOptional},
        },
    },
    entity::{add_component, add_components, remove_component, set_component},
    prelude::*,
};
use packages::{
    character_movement::concepts::{CharacterMovement, CharacterMovementOptional},
    dead_meets_lead_content::assets,
    game_object::components::health,
    this::{components::run_to, messages::Click},
    unit::components::{is_on_ground, jumping, run_direction, running, speed, vertical_velocity},
    zombie_anims::components::zombie_anims,
};

#[main]
pub fn main() {
    PerspectiveInfiniteReverseCamera {
        optional: PerspectiveInfiniteReverseCameraOptional {
            aspect_ratio_from_window: Some(entity::resources()),
            main_scene: Some(()),
            translation: Some(Vec3::ONE * 5.),
            ..default()
        },
        ..PerspectiveInfiniteReverseCamera::suggested()
    }
    .make()
    .with(lookat_target(), vec3(0., 0., 0.))
    .spawn();

    Entity::new()
        .with(local_to_world(), Mat4::IDENTITY)
        .with(plane_collider(), ())
        .with(quad(), ())
        .spawn();

    spawn_query(is_player()).bind(|players| {
        for (id, _) in players {
            let zombie = Entity::new()
                .with_merge(Transformable {
                    local_to_world: Default::default(),
                    optional: TransformableOptional {
                        translation: Some(Vec3::Z * 1.),
                        rotation: Some(Quat::IDENTITY),
                        scale: None,
                    },
                })
                .with(model_from_url(), assets::url("Data/Models/Units/Zombie1.x"))
                .with(zombie_anims(), EntityId::null())
                .with_merge(CharacterMovement {
                    // optional: CharacterMovementOptional {
                    //     run_speed_multiplier: Some(1.),
                    //     speed: Some(1.),
                    //     strafe_speed_multiplier: Some(1.),
                    //     air_speed_multiplier: Some(1.),
                    // },
                    ..CharacterMovement::suggested()
                })
                .with(health(), 100.);
            add_components(id, zombie);

            // let idle = PlayClipFromUrlNodeRef::new(assets::url(
            //     "Data/Models/Units/Zombie1.x/animations/Run1.anim",
            // ));
            // let anim_player = AnimationPlayerRef::new(idle);
            // entity::add_component(id, apply_animation_player(), anim_player.0);
        }
    });

    Click::subscribe(|cx, ev| {
        if let Some(hit) = physics::raycast_first(ev.orig, ev.dir) {
            println!("hit: {:?}", hit);
            let id = cx.client_entity_id().unwrap();
            add_component(id, run_to(), hit.position);
        }
    });

    query((run_to(), translation())).each_frame(|entities| {
        for (id, (target, pos)) in entities {
            let delta = target - pos;
            if delta.length() < 0.1 {
                remove_component(id, run_to());
                set_component(id, run_direction(), Vec2::ZERO);
            } else {
                let dir = delta.normalize();
                let rot = dir.y.atan2(dir.x);
                set_component(id, run_direction(), Vec2::X);
                set_component(id, rotation(), Quat::from_rotation_z(rot));
            }
        }
    });
}
