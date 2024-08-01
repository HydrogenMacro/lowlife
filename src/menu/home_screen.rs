use macroquad::hash;
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use crate::scene::Scene;

#[derive(Default)]
pub struct HomeScreen {
	a: u32
}
impl Scene for HomeScreen {
	fn update(&mut self) {

	}

	fn draw(&mut self) {
		yakui_macroquad::ui(|yakui| {
			yakui::center(|| {
				yakui::text(50., "hi");
			});
		});
	}
}
