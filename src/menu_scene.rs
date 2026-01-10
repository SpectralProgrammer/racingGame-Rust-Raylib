use raylib::prelude::*;

use crate::game_data::GameData;
use crate::game_scene::GameScene;
use crate::scenes::{Scene, SceneSwitch};
use crate::utils::*;

pub struct MenuScene;

impl Scene for MenuScene{
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData) {}

    fn handle_input(&mut self, rl:&mut RaylibHandle, data:&mut GameData)->SceneSwitch {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT){
            let click = rl.get_mouse_position();

            // Rect dimensions to be changed
            let rectangle = Rectangle::new(200.0, 200.0, 300.0, 150.0);

            if check_collision_point_rect(&click, &rectangle){
                println!("Clicked");

                // Check to see if this works without the 'return' keyword
                return SceneSwitch::Push(Box::new(GameScene::new(5, data.screen_width, data.screen_width)));
            }
        }

        ScreenSwitch::None

    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData) {
        
    }

    fn on_exit(&mut self, _rl:&mut RaylibHandle, _data:&mut GameData) {
        
    }

    fn update(&mut self, _dt:f32, _data: &mut GameData)->SceneSwitch {
        SceneSwitch::None
    }
    
}