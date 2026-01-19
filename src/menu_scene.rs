
use raylib::prelude::*;

use crate::game_data::GameData;
use crate::game_scene::GameScene;
use crate::scenes::{Scene, SceneSwitch};
use crate::select_scene::SelectScene;
use crate::settings_scene::SettingsScene;
use crate::utils::*;

pub struct MenuScene{
    title_texture: Option<Texture2D>,
    menu_texture: Option<Texture2D>
}

impl MenuScene{
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self{
        let title_texture = rl
            .load_texture(thread, "Assets/title_image.png")
            .expect("Failed to load title image");

        let menu_texture = rl
        .load_texture(thread, "Assets/brickBackground.jpg")
        .expect("Failed to load menu background image");

        Self { title_texture: Some(title_texture), menu_texture: Some(menu_texture) }
    }
}

impl Scene for MenuScene{
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData, _thread: &RaylibThread){}

    fn handle_input(&mut self, rl:&mut RaylibHandle, _data:&mut GameData) -> SceneSwitch{
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT){
            let click = rl.get_mouse_position();

            let play_button_rectangle = Rectangle::new(210.0, 285.0, 220.0, 50.0);
            let settings_button_rectangle = Rectangle::new(210.0, 345.0, 220.0, 50.0);

            if check_collision_point_rect(&click, &play_button_rectangle){
                println!("Play button clicked");
                // return SceneSwitch::Push(Box::new(GameScene::new(Vector2::new(100.0, 100.0), 90.0)));
                return SceneSwitch::Push(Box::new(SelectScene));
            }
            else if check_collision_point_rect(&click,&settings_button_rectangle){
                println!("Settings button clicked");
                return SceneSwitch::Push(Box::new(SettingsScene));
            }
        }

        SceneSwitch::None

    }

    fn draw(&self, d: &mut RaylibDrawHandle, _data: &mut GameData){
        d.clear_background(Color::WHITE);

        // Resizing the background image to fill screen
        if let Some(texture) = &self.menu_texture {
            let tex_w = texture.width as f32;
            let tex_h = texture.height as f32;

            let win_w = 640.0;
            let win_h = 480.0;

            let scale = (win_w / tex_w).min(win_h / tex_h);

            let dest_w = (tex_w * scale) + 350.0;
            let dest_h = tex_h * scale;

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

        // d.draw_text("WD40: Rust Off", 130, 150, 50, Color::BLACK);

        let play_button = Rectangle{ 
            x: 210.0,
            y: 285.0,
            width: 220.0,
            height: 50.0 
        }; 

        let settings_button = Rectangle{
            x: 210.0,
            y: 345.0,
            width: 220.0,
            height: 50.0
        };

        if let Some(texture) = &self.title_texture{
            let title_image_x: i32 = 20;
            let title_image_y = -8;

            d.draw_texture(texture, title_image_x, title_image_y, Color::WHITE);
        }
        
        d.draw_rectangle_rounded(play_button, 0.4, 12, Color::BURLYWOOD);
        d.draw_text("Play", 290, play_button.y as i32 + 10, 30, Color::BLACK);

        d.draw_rectangle_rounded(settings_button, 0.4, 12, Color::BURLYWOOD);
        d.draw_text("Settings", 260, settings_button.y as i32 + 10, 30, Color::BLACK);

    }

    fn on_exit(&mut self, rl:&mut RaylibHandle, _data:&mut GameData, thread: &RaylibThread){
        if let Some(texture) = self.title_texture.take(){
            unsafe{rl.unload_texture(thread, texture.make_weak())};
        }
    }

    fn update(&mut self, _dt:f32, _data: &mut GameData)->SceneSwitch{
        SceneSwitch::None
    }
    
}