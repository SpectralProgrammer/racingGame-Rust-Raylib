
use raylib::prelude::*;

use crate::game_data::GameData;
use crate::menu_scene::MenuScene;

fn main() {

    // Game window dimensions
    let width = 640;
    let height = 480;

    // Creating game window
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("WD40:Rust-Off")
        .build();

    let mut game_data = GameData::new(width, height);
    let mut scenes: Vec<Box<dyn Scene>> = vec![Box::new(MenuScene)];

    let mut last_time = Instant::now();
    let mut keep_playing = true;
     
    while !rl.window_should_close() && keep_playing{
        let temp = Instant::now();
        let delta = (temp - last_time).as_secs_f32();
        last_time = temp;
        let mut d = rl.begin_drawing(&thread);
         
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}