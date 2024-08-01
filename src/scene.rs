use std::ops::Deref;
use crate::menu::HomeScreen;

pub trait Scene {
	fn update(&mut self);
	fn draw(&self);
}