use std::ops::{Deref, DerefMut};
use macroquad::prelude::collections::storage;
use crate::menu::HomeScreen;

pub trait Scene {
	fn update(&mut self);
	fn draw(&mut self);
}
pub struct CurrentScene(Box<dyn Scene>);
impl CurrentScene {
	pub fn init(scene: impl Scene + 'static) {
		storage::store(CurrentScene(Box::new(scene)));
	}
}
impl Deref for CurrentScene {
	type Target = Box<dyn Scene>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl DerefMut for CurrentScene {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}