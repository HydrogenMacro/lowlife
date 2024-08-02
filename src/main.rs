mod home_screen;

use bevy::math::vec2;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
	    .add_systems(Update, a)
        .run();
}
fn a(mut gizmos: Gizmos,) {
	gizmos.line_2d(vec2(-10., -10.), vec2(10., 10.), Color::srgb(1., 1., 0.));
}