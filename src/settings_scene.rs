
use raylib::prelude::*;

use crate::game_data::GameData;
use crate::game_scene::GameScene;
use crate::scenes::{Scene, SceneSwitch};
use crate::utils::*;

pub struct SettingsScene;

impl Scene for SettingsScene{
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData, _thread: &RaylibThread){}

    fn handle_input(&mut self, rl:&mut RaylibHandle, _data:&mut GameData ) -> SceneSwitch{
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT){
            let click = rl.get_mouse_position();

            let play_button_rectangle = Rectangle::new(210.0, 285.0, 220.0, 50.0);

            if check_collision_point_rect(&click, &play_button_rectangle){
                println!("Play button clicked");
                return SceneSwitch::Push(Box::new(GameScene::new(Vector2::new(100.0, 100.0), 90.0)));
            }
        }

        SceneSwitch::None

    }

    fn draw(&self, d: &mut RaylibDrawHandle, _data: &mut GameData){
        d.clear_background(Color::KHAKI);

        let play_button = Rectangle{ 
            x: 210.0,
            y: 285.0,
            width: 220.0,
            height: 50.0 
        }; 
        
        d.draw_rectangle_rounded(play_button, 0.4, 12, Color::BURLYWOOD);
        d.draw_text("Play", 290, play_button.y as i32 + 10, 30, Color::BLACK);

    }

    fn on_exit(&mut self, _rl:&mut RaylibHandle, _data:&mut GameData, _thread: &RaylibThread){}

    fn update(&mut self, _dt:f32, _data: &mut GameData)->SceneSwitch{
        SceneSwitch::None
    }
    
}