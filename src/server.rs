use ambient_api::{
    animation::{AnimationPlayerRef, PlayClipFromUrlNodeRef},
    animation_element::{AnimationPlayer, PlayClipFromUrl, Transition},
    core::{
        animation::components::apply_animation_player,
        app::components::main_scene,
        camera::{
            components::fog,
            concepts::{
                PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
            },
        },
        hierarchy::components::parent,
        model::components::model_from_url,
        physics::components::plane_collider,
        player::components::is_player,
        prefab::components::prefab_from_url,
        primitives::components::quad,
        rendering::components::{
            fog_color, fog_density, fog_height_falloff, light_ambient, light_diffuse,
            pbr_material_from_url, sun,
        },
        transform::{
            components::{local_to_world, lookat_target, rotation, scale, translation},
            concepts::{Transformable, TransformableOptional},
        },
    },
    entity::{add_child, add_component, add_components, remove_component, set_component},
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
use std::f32::consts::PI;

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
    .with(fog(), ())
    .spawn();

    let ground = Entity::new()
        .with(local_to_world(), Mat4::IDENTITY)
        .with(plane_collider(), ())
        .spawn();

    for y in 0..100 {
        for x in 0..100 {
            Entity::new()
                .with(local_to_world(), Mat4::IDENTITY)
                .with(parent(), ground)
                .with(translation(), ivec3(x, y, 0).as_vec3() - vec3(50., 50., 0.))
                .with(quad(), ())
                // .with(scale(), Vec3::ONE * 100.)
                .with(
                    pbr_material_from_url(),
                    assets::url("pipeline.toml/114/mat.json"),
                )
                .spawn();
        }
    }

    Entity::new()
        .with(sun(), 0.0)
        .with(rotation(), Quat::from_rotation_y(-0.7))
        .with(main_scene(), ())
        .with(light_diffuse(), 3. * ivec3(247, 250, 152).as_vec3() / 255.)
        .with(light_ambient(), 0.5 * ivec3(0, 106, 255).as_vec3() / 255.)
        .with(fog_color(), vec3(0.1, 1., 0.5))
        .with(fog_density(), 0.01)
        .with(fog_height_falloff(), 0.01)
        .spawn();

    let trees = [
        "Swamptree1.x",
        "Swamptree2.x",
        "Palmtree1.x",
        "Palmtree2.x",
        "Rottentree1.x",
    ];
    let foliage = [
        // "Fern1.x",
        "Fern2.x",
        "FieldPlant1.x",
        "Flower1.x",
        "Forestplants1.x",
        "Forestplants2.x",
        "Leaf1.x",
        "Leaf2.x",
        "Shrub1.x",
        "Shrub2.x",
        "Shrubbery1.x",
        // "Tallgrass1.x",
        // "Tallgrass2.x",
        // "Tallgrass3.x",
    ];
    let stones = [
        "GravelStone1.x",
        "MudPebble1.x",
        "MudPebble2.x",
        "MudPebble3.x",
        "Pebble4.x",
        "Stone1.x",
        "Stone3.x",
        "Stone4.x",
        // "pebble1.x",
        // "pebble2.x",
        // "pebble3.x",
        // "stone2.x",
    ];
    fn spawn(name: &str) {
        Entity::new()
            .with(
                model_from_url(),
                assets::url(&format!("Data/Models/Props/{name}")),
            )
            .with(
                translation(),
                random::<Vec2>().extend(0.) * 100. - vec3(50., 50., 0.),
            )
            .with(rotation(), Quat::from_rotation_z(random::<f32>() * 2. * PI))
            .with(scale(), Vec3::ONE * 0.6 + Vec3::ONE * random::<f32>() * 0.8)
            .spawn();
    }

    for i in 0..100 {
        for tree in trees {
            spawn(tree)
        }
    }
    for i in 0..1000 {
        for tree in foliage {
            spawn(tree)
        }
    }
    for i in 0..100 {
        for tree in stones {
            spawn(tree)
        }
    }

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

// Trees:
// Palmtree1.x
// Palmtree2.x
// Rottentree1.x
// Swamptree1.x
// Swamptree2.x
// Treetrunk1.x
// Treetrunk1Pathmesh.x
// Treetrunk2.x
// Treetrunk2Pathmesh.x
//
// Foliage:
// Fern1.x
// Fern2.x
// FieldPlant1.x
// Flower1.x
// Forestplants1.x
// Forestplants2.x
// Leaf1.x
// Leaf2.x
// Shrub1.x
// Shrub2.x
// Shrubbery1.x
// Tallgrass1.x
// Tallgrass2.x
// Tallgrass3.x
//
// Stones:
// GravelStone1.x
// MudPebble1.x
// MudPebble2.x
// MudPebble3.x
// Pebble4.x
// Stone1.x
// Stone1Pathmesh.x
// Stone3.x
// Stone3Pathmesh.x
// Stone4.x
// Stone4Pathmesh.x
// pebble1.x
// pebble2.x
// pebble3.x
// stone2.x
//
// Man made props:
// Bottle1.x
// Barrel1.x
// Barrel2.x
// Bench1.x
// Bench2.x
// Bucket1.x
// Bucket2.x
// Bucket3.x
// Cannon1.x
// Cannon2.x
// Cannonball1.x
// Cannonball2.x
// Crate1.x
// Crate2.x
// Crate3.x
// Litter1.x
// Crate1Pathmesh.x
// Crate2Pathmesh.x
// Crate3Pathmesh.x
// Barrel1Pathmesh.x
// Barrel2Pathmesh.x
// Bench1Pathmesh.x
// Bench2Pathmesh.x
// Cannon1Hitbox.x
// Cannon2Hitbox.x
// Cart1.x
// Cart1Hitbox.x
// Cart2.x
// Cart2Hitbox.x
// PostLantern1.x
// PostLantern1Pathmesh.x
// Rope1.x
// Skullpile1.x
// Torch1.x
// Totem1.x
//
// Weapons:
// Blaster1.x
// GatlingGun1.x
// Hammer1.x
// HandCannon1.x
// MayaHammer1.x
// Rifle1.x
// Spear1.x
// Sword1.x
//
// Pickups:
// Ammobox1.x
// Chest1.x
//
// Buildings:
// Altar1.x
// Altar1Pathmesh.x
// Bridge1.x
// Bridge1Pathmesh.x
// BridgeBroken1Pathmesh.x
// BridgeRaising1.x
// Fence1.x
// Fence2.x
// Fence3.x
// Fence4.x
// FencePathmesh1.x
// Gate1.x
// GatePathmeshClosed1.x
// GatePathmeshOpen1.x
// Hut1.x
// Hut1Pathmesh.x
// Hut2.x
// Hut2Pathmesh.x
// Palisade1.x
// Palisade1Pathmes.x
// Palisade1Pathmesh.x
// Tent1.x
// Tent1Hitbox.x
// Wall1.x
// Wall1Pathmesh.x
// Wall2.x
// Wall3.x
// WallEnd1.x
// Wallend1Pathmesh.x
//
// Projectiles:
// BlasterProjectile1.x
// CannonballProjectile1.x
//
// Critters:
// Chicken1.x
// Mosquito1.x
// Piranha1.x
//
// Misc:
// Boat1.x
// Boat2.x
// Fireplace1.x
// FloatingPlank1.x
// FloatingPlank2.x
// FloatingTreetrunk1.x
// FloatingTreetrunk1Pathmesh.x
// Piranhasign1.x
// Spiderweb1.x
// WaterLily1.x
// ZombieSit1.x
// ZombieSit2.x
