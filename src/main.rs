#![feature(impl_trait_in_assoc_type)]
mod menu;
mod game;
mod scene;

use std::thread;
use macroquad::prelude::*;
use macroquad::experimental::collections::storage;
use crate::menu::HomeScreen;
use crate::scene::{CurrentScene, Scene};

#[macroquad::main("Lowlife")]
async fn main() {
	CurrentScene::init(HomeScreen::default());

	loop {
		clear_background(DARKGREEN);
		storage::get_mut::<CurrentScene>().update();
		storage::get_mut::<CurrentScene>().draw();
		yakui_macroquad::draw();
		next_frame().await;
	}
}
