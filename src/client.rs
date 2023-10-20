use ambient_api::{camera::screen_position_to_world_ray, core::messages::Frame, prelude::*};
use packages::{
    orbit_camera::concepts::{OrbitCamera, OrbitCameraOptional},
    this::messages::Click,
};

#[main]
pub fn main() {
    // OrbitCamera {
    //     is_orbit_camera: (),
    //     optional: OrbitCameraOptional {
    //         lookat_target: Some(vec3(0., 0., 1.)),
    //         camera_angle: Some(vec2(135f32.to_radians(), 45f32.to_radians())),
    //         camera_distance: Some(3.0),
    //     },
    // }
    // .spawn();

    Frame::subscribe(|_| {
        let (delta, cur) = input::get_delta();
        if delta.mouse_buttons.contains(&MouseButton::Left) {
            let ray =
                screen_position_to_world_ray(camera::get_active().unwrap(), cur.mouse_position);
            Click::new(ray.origin, ray.dir).send_server_reliable();
        }
    });
}
