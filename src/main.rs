use racingGame::game_data::GameData;
use racingGame::menu_scene::MenuScene;
use racingGame::scenes::SceneManager;

use std::time::Instant;

fn main(){

    // Game window dimensions
    let width = 960;
    let height = 720;

    // Creating game window
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("WD40:Rust-Off")
        .build();

    let mut game_data = GameData::new(width, height);

    let menu_scene = MenuScene::new(&mut rl, &thread);
    let mut scene_manager = SceneManager::new(&mut rl, Box::new(menu_scene), &mut game_data, &thread);

    let mut last_time = Instant::now();
     
    while !rl.window_should_close() && !scene_manager.should_quit(){
        let temp = Instant::now();
        let delta = (temp - last_time).as_secs_f32();
        last_time = temp;
        
        scene_manager.update(&mut rl, delta, &mut game_data, &thread);
        
        let mut d = rl.begin_drawing(&thread);
        scene_manager.draw(&mut d, &mut game_data);
    }
}