use ambient_api::{
    animation::{AnimationPlayerRef, PlayClipFromUrlNodeRef},
    core::{
        animation::components::apply_animation_player,
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        prefab::components::prefab_from_url,
        primitives::components::quad,
        transform::{
            components::{lookat_target, translation},
            concepts::Transformable,
        },
    },
    prelude::*,
};
use packages::dead_meets_lead_content::assets;

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

    let zombie = Entity::new()
        .with_merge(Transformable {
            local_to_world: Default::default(),
            optional: Default::default(),
        })
        .with(
            prefab_from_url(),
            assets::url("Data/Models/Units/Zombie1.x"),
        )
        .spawn();

    let idle = PlayClipFromUrlNodeRef::new(assets::url(
        "Data/Models/Units/Zombie1.x/animations/Run1.anim",
    ));
    let anim_player = AnimationPlayerRef::new(idle);
    entity::add_component(zombie, apply_animation_player(), anim_player.0);
}