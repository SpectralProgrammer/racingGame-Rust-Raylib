
use raylib::prelude::*;

use crate::game_data::{GameData, ControlChoice};
use crate::select_scene::SelectScene;
use crate::scenes::{Scene, SceneSwitch};
use crate::utils::*;

pub struct SettingsScene{
    background_texture: Option<Texture2D>,
}

impl SettingsScene{
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let background_texture = rl
            .load_texture(thread, "Assets/settingsBack.png")
            .expect("Failed to load settings background image");

        Self {
            background_texture: Some(background_texture),
        }
    }
}

impl Scene for SettingsScene{
    fn on_enter(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData, _thread: &RaylibThread) {}

    fn handle_input(
        &mut self,
        rl: &mut RaylibHandle,
        data: &mut GameData,
        thread: &RaylibThread,
    ) -> SceneSwitch {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let click = rl.get_mouse_position();

            // Dynamic positions
            let screen_center_x = data.screen_width as f32 / 2.0;

            let button_width = 420.0;
            let button_height = 50.0;
            let spacing = 20.0;

            let controller_rect = Rectangle::new(
                screen_center_x - button_width / 2.0,
                200.0,
                button_width,
                button_height,
            );
            let keyboard_rect = Rectangle::new(
                screen_center_x - button_width / 2.0,
                controller_rect.y + button_height + spacing,
                button_width,
                button_height,
            );
            let play_rect = Rectangle::new(
                screen_center_x - 220.0 / 2.0,
                keyboard_rect.y + button_height + spacing,
                220.0,
                50.0,
            );

            // Handle clicks
            if check_collision_point_rect(&click, &play_rect) {
                println!("Play button clicked");
                let select_scene =
                    SelectScene::new(rl, thread, data.screen_width, data.screen_height);
                return SceneSwitch::Push(Box::new(select_scene));
            } else if check_collision_point_rect(&click, &keyboard_rect) {
                data.selected_control = Some(ControlChoice::Keyboard);
                println!("Keyboard selected");
            } else if check_collision_point_rect(&click, &controller_rect) {
                data.selected_control = Some(ControlChoice::Controller);
                println!("Controller selected");
            }
        }

        SceneSwitch::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, data: &mut GameData){
        d.clear_background(Color::KHAKI);

        let screen_center_x = data.screen_width as f32 / 2.0;

        // Draw background image
        if let Some(texture) = &self.background_texture {
            let tex_w = texture.width as f32;
            let tex_h = texture.height as f32;
            let win_w = data.screen_width as f32;
            let win_h = data.screen_height as f32;

            let scale = (win_w / tex_w).max(win_h / tex_h);
            let dest_w = tex_w * scale;
            let dest_h = tex_h * scale;
            let dest_x = (win_w - dest_w) / 2.0;
            let dest_y = (win_h - dest_h) / 2.0;

            d.draw_texture_pro(
                texture,
                Rectangle::new(0.0, 0.0, tex_w, tex_h),
                Rectangle::new(dest_x, dest_y, dest_w, dest_h),
                Vector2::zero(),
                0.0,
                Color::WHITE,
            );
        }

        // Layout variables
        let button_width = 420.0;
        let button_height = 50.0;
        let spacing = 20.0;

        let controller_rect = Rectangle::new(
            screen_center_x - button_width / 2.0,
            200.0,
            button_width,
            button_height,
        );
        let keyboard_rect = Rectangle::new(
            screen_center_x - button_width / 2.0,
            controller_rect.y + button_height + spacing,
            button_width,
            button_height,
        );
        let play_rect = Rectangle::new(
            screen_center_x - 220.0 / 2.0,
            keyboard_rect.y + button_height + spacing,
            220.0,
            50.0,
        );

        // Title text
        let title = "Settings";
        let title_size = 60;
        let title_width = d.measure_text(title, title_size);
        d.draw_text(
            title,
            (screen_center_x - title_width as f32 / 2.0) as i32,
            100,
            title_size,
            Color::BLACK,
        );

        // Colors
        let default_color = Color::BURLYWOOD;
        let selected_color = Color::LEMONCHIFFON;

        let controller_color = if data.selected_control == Some(ControlChoice::Controller){
            selected_color
        } else {
            default_color
        };

        let keyboard_color = if data.selected_control == Some(ControlChoice::Keyboard){
            selected_color
        } else {
            default_color
        };

        // Draw controller button
        d.draw_rectangle_rounded(controller_rect, 0.4, 12, controller_color);
        let controller_text = "Controller";
        let controller_text_width = d.measure_text(controller_text, 30);
        d.draw_text(
            controller_text,
            (screen_center_x - controller_text_width as f32 / 2.0) as i32,
            (controller_rect.y + 10.0) as i32,
            30,
            Color::BLACK,
        );

        // Draw keyboard button
        d.draw_rectangle_rounded(keyboard_rect, 0.4, 12, keyboard_color);
        let keyboard_text = "Keyboard";
        let keyboard_text_width = d.measure_text(keyboard_text, 30);
        d.draw_text(
            keyboard_text,
            (screen_center_x - keyboard_text_width as f32 / 2.0) as i32,
            (keyboard_rect.y + 10.0) as i32,
            30,
            Color::BLACK,
        );

        // Draw play button
        d.draw_rectangle_rounded(play_rect, 0.4, 12, Color::BURLYWOOD);
        let play_text = "Play";
        let play_text_width = d.measure_text(play_text, 30);
        d.draw_text(
            play_text,
            (screen_center_x - play_text_width as f32 / 2.0) as i32,
            (play_rect.y + 10.0) as i32,
            30,
            Color::BLACK,
        );
    }

    fn on_exit(&mut self, _rl: &mut RaylibHandle, _data: &mut GameData, _thread: &RaylibThread) {}
    fn update(&mut self, _dt: f32, _data: &mut GameData) -> SceneSwitch {
        SceneSwitch::None
    }
}