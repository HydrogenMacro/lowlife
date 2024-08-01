use macroquad::prelude::*;
use crate::scene::Scene;

#[derive(Default)]
pub struct HomeScreen {

}
impl Scene for HomeScreen {
	fn update(&mut self) {

	}

	fn draw(&self) {
		draw_circle(0., 0., 10., RED);
	}
}
