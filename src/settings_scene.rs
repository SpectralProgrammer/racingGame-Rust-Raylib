
use raylib::prelude::*;

use crate::game_data::{GameData, ControlChoice};
use crate::select_scene::SelectScene;
use crate::scenes::{Scene, SceneSwitch};
use crate::utils::*;

pub struct SettingsScene{
    background_texture: Option<Texture2D>
}

impl SettingsScene {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        
        let background_texture = rl
        .load_texture(thread, "Assets/settingsBack.png")
        .expect("Failed to load select background image");

        Self{
            background_texture: Some(background_texture)
        }

    }
}

impl Scene for SettingsScene{
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData, _thread: &RaylibThread){}

    fn handle_input(&mut self, rl:&mut RaylibHandle, data:&mut GameData, thread: &RaylibThread) -> SceneSwitch{
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT){
            let click = rl.get_mouse_position();

            let play_button_rectangle = Rectangle::new(210.0, 320.0, 220.0, 50.0);

            let keyboard_button_rectangle = Rectangle::new(110.0, 220.0, 420.0, 50.0);
            let controller_button_rectangle = Rectangle::new(keyboard_button_rectangle.x, 150.0, keyboard_button_rectangle.width,keyboard_button_rectangle.height);
            
            if check_collision_point_rect(&click, &play_button_rectangle){
                println!("Play button clicked");
                let select_scene = SelectScene::new(rl, thread);
                return SceneSwitch::Push(Box::new(select_scene));
            }
            else if check_collision_point_rect(&click, &keyboard_button_rectangle){
                data.selected_control = Some(ControlChoice::Keyboard);
                println!("Keyboard selected");
            }
            else if check_collision_point_rect(&click, &controller_button_rectangle){
                data.selected_control = Some(ControlChoice::Controller);
                println!("Controller selected");
            }
        }

        SceneSwitch::None

    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData){
        d.clear_background(Color::KHAKI);

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

        d.draw_text("Settings", 190, 50, 60, Color::BLACK);

        let default_color = Color::BURLYWOOD;
        let selected_color = Color::LEMONCHIFFON;

        let keyboard_button_color = if data.selected_control == Some(ControlChoice::Keyboard){
            selected_color
        }
        else{
            default_color
        };

        let controller_button_color = if data.selected_control == Some(ControlChoice::Controller){
            selected_color
        }
        else{
            default_color
        };

        let keyboard_button = Rectangle{
            x: 110.0,
            y: 220.0,
            width: 420.0,
            height: 50.0
        };

        d.draw_rectangle_rounded(keyboard_button, 0.4, 12, keyboard_button_color);
        d.draw_text("Keyboard", 250, keyboard_button.y as i32 + 10, 30, Color::BLACK);

        let controller_button = Rectangle{
            x: keyboard_button.x,
            y: 150.0,
            width: 420.0,
            height: 50.0
        };

        d.draw_rectangle_rounded(controller_button, 0.4, 12, controller_button_color);
        d.draw_text("Controller", 250, controller_button.y as i32 + 10, 30, Color::BLACK);

        let play_button = Rectangle{ 
            x: 210.0,
            y: 320.0,
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