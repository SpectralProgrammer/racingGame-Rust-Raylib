
use raylib::prelude::*;

use crate::game_data::{GameData, CarChoice, TrackChoice};
use crate::game_scene::GameScene;
use crate::scenes::{Scene, SceneSwitch};
use crate::utils::*;

pub struct SelectScene{
    background_texture: Option<Texture2D>
}

impl SelectScene {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        
        let background_texture = rl
        .load_texture(thread, "Assets/selectBack2.png")
        .expect("Failed to load select background image");

        Self{
            background_texture: Some(background_texture)
        }

    }
}

impl Scene for SelectScene{
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData, _thread: &RaylibThread){}

    fn handle_input(&mut self, rl:&mut RaylibHandle, data:&mut GameData, thread: &RaylibThread) -> SceneSwitch{
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT){
            let click = rl.get_mouse_position();

            let play_button_rectangle = Rectangle::new(190.0, 400.0, 220.0, 50.0);
            
            let car1_button_rectangle = Rectangle::new(90.0, 270.0, 100.0, 100.0);
            let car2_button_rectangle = Rectangle::new(200.0, car1_button_rectangle.y, car1_button_rectangle.width, car1_button_rectangle.height);
            let car3_button_rectangle = Rectangle::new(320.0, car1_button_rectangle.y, car1_button_rectangle.width, car1_button_rectangle.height);
            let car4_button_rectangle = Rectangle::new(440.0, car1_button_rectangle.y, car1_button_rectangle.width, car1_button_rectangle.height);

            let track1_button_rectangle = Rectangle::new(90.0, 100.0, 100.0, 100.0);
            let track2_button_rectangle = Rectangle::new(210.0, track1_button_rectangle.y, track1_button_rectangle.width, track1_button_rectangle.height);
            let track3_button_rectangle = Rectangle::new(330.0, track1_button_rectangle.y, track1_button_rectangle.width, track1_button_rectangle.height);
            let track4_button_rectangle = Rectangle::new(450.0, track1_button_rectangle.y, track1_button_rectangle.width, track1_button_rectangle.height);

            if check_collision_point_rect(&click, &play_button_rectangle){
                println!("Play button clicked");
                return SceneSwitch::Push(Box::new(GameScene::new(rl, thread,Vector2::new(100.0, 100.0), 90.0)));
            }
            else if check_collision_point_rect(&click, &car1_button_rectangle){
                data.selected_car = Some(CarChoice::Car1);
                println!("Car 1 selected");
            }
            else if check_collision_point_rect(&click, &car2_button_rectangle){
                data.selected_car = Some(CarChoice::Car2);
                println!("Car 2 selected");
            }
            else if check_collision_point_rect(&click, &car3_button_rectangle){
                data.selected_car = Some(CarChoice::Car3);
                println!("Car 3 selected");
            }
            else if check_collision_point_rect(&click, &car4_button_rectangle){
                data.selected_car = Some(CarChoice::Car4);
                println!("Car 4 selected");
            }
            else if check_collision_point_rect(&click, &track1_button_rectangle){
                data.selected_track = Some(TrackChoice::Track1);
                println!("Track 1 selected");
            }
            else if check_collision_point_rect(&click, &track2_button_rectangle){
                data.selected_track = Some(TrackChoice::Track2);
                println!("Track 2 selected");
            }
            else if check_collision_point_rect(&click, &track3_button_rectangle){
                data.selected_track = Some(TrackChoice::Track3);
                println!("Track 3 selected");
            }
            else if check_collision_point_rect(&click, &track4_button_rectangle){
                data.selected_track = Some(TrackChoice::Track4);
                println!("Track 4 selected");
            }
        }

        SceneSwitch::None

    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData){
        d.clear_background(Color::WHITESMOKE);

        // Resizing the background image to fill screen
        if let Some(texture) = &self.background_texture{
            let tex_w = texture.width as f32;
            let tex_h = texture.height as f32;

            let win_w = 640.0;
            let win_h = 480.0;

            let scale = (win_w / tex_w).min(win_h / tex_h);

            let dest_w = tex_w * scale;
            let dest_h = (tex_h * scale) + 55.0;

            let dest_x = (win_w - dest_w) / 2.0;
            let dest_y = (win_h - dest_h) / 2.0;

            let source = Rectangle::new(0.0, 0.0, tex_w, tex_h);
            let dest = Rectangle::new(dest_x, dest_y, dest_w, dest_h);

            d.draw_texture_pro(
                texture,
                source,
                dest,
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }

        let default_color = Color::BURLYWOOD;
        let selected_color = Color::LEMONCHIFFON;

        d.draw_text("Track Select", 180, 45, 40, Color::BLACK);

        let track1_button_color = if data.selected_track == Some(TrackChoice::Track1){
            selected_color
        }
        else{
            default_color
        };

        let track2_button_color = if data.selected_track == Some(TrackChoice::Track2){
            selected_color
        }
        else{
            default_color
        };

        let track3_button_color = if data.selected_track == Some(TrackChoice::Track3){
            selected_color
        }
        else{
            default_color
        };

        let track4_button_color = if data.selected_track == Some(TrackChoice::Track4){
            selected_color
        }
        else{
            default_color
        };

        let track1_button = Rectangle{
            x: 90.0,
            y: 100.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(track1_button, 0.4, 12, track1_button_color);
        d.draw_text("Track 1", 100, track1_button.y as i32 + 40, 20, Color::BLACK);

        let track2_button = Rectangle{
            x: 210.0,
            y: 100.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(track2_button, 0.4, 12, track2_button_color);
        d.draw_text("Track 2", 220, track1_button.y as i32 + 40, 20, Color::BLACK);

        let track3_button = Rectangle{
            x: 330.0,
            y: 100.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(track3_button, 0.4, 12, track3_button_color);
        d.draw_text("Track 3", 340, track1_button.y as i32 + 40, 20, Color::BLACK);

        let track4_button = Rectangle{
            x: 450.0,
            y: 100.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(track4_button, 0.4, 12, track4_button_color);
        d.draw_text("Track 4", 460, track1_button.y as i32 + 40, 20, Color::BLACK);

        d.draw_text("Car Select", 200, 220, 40, Color::BLACK);

        let car1_button_color = if data.selected_car == Some(CarChoice::Car1){
            selected_color
        }
        else{
            default_color
        };

        let car2_button_color = if data.selected_car == Some(CarChoice::Car2){
            selected_color
        }
        else{
            default_color
        };

        let car3_button_color = if data.selected_car == Some(CarChoice::Car3){
            selected_color
        }
        else{
            default_color
        };

        let car4_button_color = if data.selected_car == Some(CarChoice::Car4){
            selected_color
        }
        else{
            default_color
        };

        let car1_button = Rectangle{
            x: 90.0,
            y: 270.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(car1_button, 0.4, 12, car1_button_color);
        d.draw_text("Car 1", 120, car1_button.y as i32 + 40, 20, Color::BLACK);

        let car2_button = Rectangle{
            x: 200.0,
            y: 270.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(car2_button, 0.4, 12, car2_button_color);
        d.draw_text("Car 2", 230, car1_button.y as i32 + 40, 20, Color::BLACK);

        let car3_button = Rectangle{
            x: 320.0,
            y: 270.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(car3_button, 0.4, 12, car3_button_color);
        d.draw_text("Car 3", 350, car1_button.y as i32 + 40, 20, Color::BLACK);

        let car4_button = Rectangle{
            x: 440.0,
            y: 270.0,
            width: 100.0,
            height: 100.0
        };

        d.draw_rectangle_rounded(car4_button, 0.4, 12, car4_button_color);
        d.draw_text("Car 4", 470, car1_button.y as i32 + 40, 20, Color::BLACK);

        let play_button = Rectangle{ 
            x: 190.0,
            y: 400.0,
            width: 220.0,
            height: 50.0 
        }; 
        
        d.draw_rectangle_rounded(play_button, 0.4, 12, Color::BURLYWOOD);
        d.draw_text("Play", 270, play_button.y as i32 + 10, 30, Color::BLACK);

    }

    fn on_exit(&mut self, _rl:&mut RaylibHandle, _data:&mut GameData, _thread: &RaylibThread){}

    fn update(&mut self, _dt:f32, _data: &mut GameData)->SceneSwitch{
        SceneSwitch::None
    }
    
}