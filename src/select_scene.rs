
use raylib::prelude::*;

use crate::game_data::GameData;
use crate::game_scene::GameScene;
use crate::scenes::{Scene, SceneSwitch};
use crate::utils::*;

pub struct SelectScene;

impl Scene for SelectScene{
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData, _thread: &RaylibThread){}

    fn handle_input(&mut self, rl:&mut RaylibHandle, _data:&mut GameData ) -> SceneSwitch{
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT){
            let click = rl.get_mouse_position();

            let play_button_rectangle = Rectangle::new(210.0, 420.0, 220.0, 50.0);

            if check_collision_point_rect(&click, &play_button_rectangle){
                println!("Play button clicked");
                return SceneSwitch::Push(Box::new(GameScene::new(Vector2::new(100.0, 100.0), 90.0)));
            }
        }

        SceneSwitch::None

    }

    fn draw(&self, d: &mut RaylibDrawHandle, _data: &mut GameData){
        d.clear_background(Color::WHITESMOKE);

        d.draw_text("Track Select", 190, 45, 40, Color::BLACK);

        let track1_button = Rectangle{
            x: 90.0,
            y: 90.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(track1_button, 0.4, 12, Color::BURLYWOOD);
        d.draw_text("Track 1", 100, track1_button.y as i32 + 40, 20, Color::BLACK);

        let track2_button = Rectangle{
            x: 210.0,
            y: 90.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(track2_button, 0.4, 12, Color::BURLYWOOD);
        d.draw_text("Track 2", 220, track1_button.y as i32 + 40, 20, Color::BLACK);

        let track3_button = Rectangle{
            x: 330.0,
            y: 90.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(track3_button, 0.4, 12, Color::BURLYWOOD);
        d.draw_text("Track 3", 340, track1_button.y as i32 + 40, 20, Color::BLACK);

        let track4_button = Rectangle{
            x: 450.0,
            y: 90.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(track4_button, 0.4, 12, Color::BURLYWOOD);
        d.draw_text("Track 4", 460, track1_button.y as i32 + 40, 20, Color::BLACK);

        d.draw_text("Car Select", 200, 245, 40, Color::BLACK);
        
        let car1_button = Rectangle{
            x: 100.0,
            y: 290.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(car1_button, 0.4, 12, Color::BURLYWOOD);
        d.draw_text("Car 1", 120, car1_button.y as i32 + 40, 20, Color::BLACK);

        let car2_button = Rectangle{
            x: 210.0,
            y: 290.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(car2_button, 0.4, 12, Color::BURLYWOOD);
        d.draw_text("Car 2", 230, car1_button.y as i32 + 40, 20, Color::BLACK);

        let car3_button = Rectangle{
            x: 330.0,
            y: 290.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(car3_button, 0.4, 12, Color::BURLYWOOD);
        d.draw_text("Car 3", 350, car1_button.y as i32 + 40, 20, Color::BLACK);

        let car4_button = Rectangle{
            x: 450.0,
            y: 290.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(car4_button, 0.4, 12, Color::BURLYWOOD);
        d.draw_text("Car 4", 470, car1_button.y as i32 + 40, 20, Color::BLACK);

        let play_button = Rectangle{ 
            x: 210.0,
            y: 420.0,
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