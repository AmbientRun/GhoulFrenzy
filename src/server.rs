use ambient_api::{
    animation::{AnimationPlayerRef, PlayClipFromUrlNodeRef},
    animation_element::{AnimationPlayer, PlayClipFromUrl, Transition},
    core::{
        animation::components::apply_animation_player,
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        player::components::is_player,
        prefab::components::prefab_from_url,
        primitives::components::quad,
        transform::{
            components::{lookat_target, translation},
            concepts::Transformable,
        },
    },
    entity::add_components,
    prelude::*,
};
use packages::{
    dead_meets_lead_content::assets, game_object::components::health,
    unit_schema::components::run_direction, zombie_anims::components::zombie_anims,
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

    spawn_query(is_player()).bind(|players| {
        for (id, _) in players {
            let zombie = Entity::new()
                .with_merge(Transformable {
                    local_to_world: Default::default(),
                    optional: Default::default(),
                })
                .with(
                    prefab_from_url(),
                    assets::url("Data/Models/Units/Zombie1.x"),
                )
                .with(zombie_anims(), EntityId::null())
                .with(run_direction(), Vec2::ZERO)
                .with(health(), 100.);
            add_components(id, zombie);

            // let idle = PlayClipFromUrlNodeRef::new(assets::url(
            //     "Data/Models/Units/Zombie1.x/animations/Run1.anim",
            // ));
            // let anim_player = AnimationPlayerRef::new(idle);
            // entity::add_component(id, apply_animation_player(), anim_player.0);
        }
    });
}
