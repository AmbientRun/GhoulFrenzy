use ambient_api::{camera::screen_position_to_world_ray, core::messages::Frame, prelude::*};
use packages::this::messages::Click;

#[main]
pub fn main() {
    Frame::subscribe(|_| {
        let (delta, cur) = input::get_delta();
        if delta.mouse_buttons.contains(&MouseButton::Left) {
            let ray =
                screen_position_to_world_ray(camera::get_active().unwrap(), cur.mouse_position);
            Click::new(ray.origin, ray.dir).send_server_reliable();
        }
    });
}
