use ambient_api::{
    animation_element::{AnimationPlayer, PlayClipFromUrl, Transition},
    core::animation::components::apply_animation_player,
    prelude::*,
};
use packages::{
    dead_meets_lead_content::assets, game_object::components::health,
    this::components::zombie_anims, unit_schema::components::run_direction,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[main]
pub fn main() {
    let anims = Arc::new(Mutex::new(HashMap::<EntityId, ElementTree>::new()));

    spawn_query(zombie_anims()).bind({
        let anims = anims.clone();
        move |v| {
            let mut anims = anims.lock().unwrap();
            for (id, target) in v {
                let target = if target.is_null() { id } else { target };
                let tree = ZombieAnimation::from_entity(id).el().spawn_tree();
                entity::add_component(
                    target,
                    apply_animation_player(),
                    tree.root_entity().unwrap(),
                );
                anims.insert(id, tree);
            }
        }
    });

    query(zombie_anims()).each_frame(move |res| {
        let mut anims = anims.lock().unwrap();
        for (id, _) in res {
            let tree = anims.get_mut(&id).unwrap();
            tree.migrate_root(&mut World, ZombieAnimation::from_entity(id).el());
            tree.update(&mut World);
        }
    });
}

#[element_component(without_el)]
fn ZombieAnimation(_hooks: &mut Hooks, direction: Vec2, health: f32) -> Element {
    AnimationPlayer {
        root: Transition {
            animations: vec![
                PlayClipFromUrl {
                    url: assets::url("Data/Models/Units/Zombie1.x/animations/Death1.anim"),
                    looping: false,
                }
                .el()
                .key("death"),
                PlayClipFromUrl {
                    url: assets::url("Data/Models/Units/Zombie1.x/animations/Run1.anim"),
                    looping: true,
                }
                .el()
                .key("walk"),
                PlayClipFromUrl {
                    url: assets::url("Data/Models/Units/Zombie1.x/animations/Idle1.anim"),
                    looping: true,
                }
                .el()
                .key("idle"),
            ],
            active: if health <= 0. {
                0
            } else if direction.length() > 0. {
                1
            } else {
                2
            },
            speed: 0.3,
        }
        .el(),
    }
    .el()
}
impl ZombieAnimation {
    fn from_entity(entity: EntityId) -> Self {
        Self {
            direction: entity::get_component(entity, run_direction()).unwrap_or_default(),
            health: entity::get_component(entity, health()).unwrap_or(100.),
        }
    }
}
